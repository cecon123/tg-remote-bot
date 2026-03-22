use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;

use anyhow::Result;
use teloxide::prelude::*;

use crate::service::config;

pub mod auth;
pub mod md;
pub mod rate_limit;
pub mod router;

pub type ActiveJob = Arc<Mutex<Option<RunningJob>>>;

pub struct RunningJob {
    pub pid: u32,
    pub handle: JoinHandle<()>,
}

pub struct AgentState {
    pub active_job: ActiveJob,
    pub agent_version: &'static str,
    pub super_user_id: i64,
    pub rate_limiter: rate_limit::RateLimiter,
    pub start_time: Instant,
}

pub fn format_duration(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;
    if days > 0 {
        format!("{days}d {hours}h {mins}m {secs}s")
    } else if hours > 0 {
        format!("{hours}h {mins}m {secs}s")
    } else if mins > 0 {
        format!("{mins}m {secs}s")
    } else {
        format!("{secs}s")
    }
}

pub fn truncate_str(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_string();
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    let mut result = s[..end].to_string();
    result.push_str("\n...(truncated)");
    result
}

fn should_shutdown(rx: &Receiver<()>) -> bool {
    rx.try_recv().is_ok()
}

async fn sleep_with_shutdown(dur: Duration, rx: &Receiver<()>) {
    let deadline = Instant::now() + dur;
    loop {
        if should_shutdown(rx) {
            return;
        }
        if Instant::now() >= deadline {
            return;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

pub async fn run_until(shutdown_rx: Receiver<()>) -> Result<()> {
    let cfg = config::load()?;
    let bot = Bot::new(&cfg.bot_token);

    let state = Arc::new(AgentState {
        active_job: Arc::new(Mutex::new(None)),
        agent_version: env!("CARGO_PKG_VERSION"),
        super_user_id: cfg.super_user_id,
        rate_limiter: rate_limit::RateLimiter::new(),
        start_time: Instant::now(),
    });

    let mut retry_delay = Duration::from_secs(5);
    let max_delay = Duration::from_secs(60);

    loop {
        if should_shutdown(&shutdown_rx) {
            log::info!("Shutdown signal received");
            break;
        }

        log::info!("Starting polling (retry delay: {}s)...", retry_delay.as_secs());

        let listener = teloxide::update_listeners::polling_default(bot.clone()).await;

        let mut dispatcher = Dispatcher::builder(bot.clone(), router::schema())
            .dependencies(dptree::deps![state.clone()])
            .enable_ctrlc_handler()
            .default_handler(|upd| async move {
                log::trace!("Unhandled update: {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text("Bot error"))
            .build();

        let start = Instant::now();

        tokio::select! {
            _ = dispatcher.dispatch_with_listener(
                listener,
                LoggingErrorHandler::with_custom_text("Listener error"),
            ) => {
                let elapsed = start.elapsed();
                if elapsed < Duration::from_secs(30) {
                    log::warn!(
                        "Polling terminated (likely by another instance), retrying in {}s...",
                        retry_delay.as_secs()
                    );
                } else {
                    retry_delay = Duration::from_secs(5);
                }
            }
        }

        if should_shutdown(&shutdown_rx) {
            log::info!("Shutdown signal received");
            break;
        }

        sleep_with_shutdown(retry_delay, &shutdown_rx).await;
        retry_delay = (retry_delay * 2).min(max_delay);
    }

    log::info!("Bot stopped");
    Ok(())
}
