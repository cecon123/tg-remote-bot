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
    #[allow(dead_code)]
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

/// Format a duration in seconds to a human-readable string (e.g., "2d 3h 15m 30s").
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

/// Truncate a string to at most `max_bytes`, respecting UTF-8 char boundaries.
pub fn truncate_str(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_string();
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

/// Truncate, append suffix if truncated, then escape for MarkdownV2.
pub fn truncate_and_escape(text: &str, max_bytes: usize) -> String {
    let truncated = truncate_str(text, max_bytes);
    let suffix = if truncated.len() < text.len() { "\n...(truncated)" } else { "" };
    md::escape(&format!("{truncated}{suffix}"))
}

fn should_shutdown(rx: &Receiver<()>) -> bool {
    rx.try_recv().is_ok()
}

/// Sleep for `dur`, but return early if shutdown signal received.
async fn sleep_with_shutdown(dur: Duration, rx: &Receiver<()>) {
    let deadline = Instant::now() + dur;
    while Instant::now() < deadline {
        if should_shutdown(rx) {
            return;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

/// Run the bot polling loop with exponential backoff retry on failure.
///
/// `enable_ctrlc` is true for foreground mode (--run), false for daemon mode.
pub async fn run_until(shutdown_rx: Receiver<()>, cfg: config::AppConfig, enable_ctrlc: bool) -> Result<()> {
    let bot = Bot::new(&cfg.bot_token);

    let state = Arc::new(AgentState {
        active_job: Arc::new(Mutex::new(None)),
        agent_version: env!("CARGO_PKG_VERSION"),
        super_user_id: cfg.super_user_id,
        rate_limiter: rate_limit::RateLimiter::new(),
        start_time: Instant::now(),
    });

    // Exponential backoff: 5s → 10s → 20s → 40s → 60s (max)
    let mut retry_delay = Duration::from_secs(5);
    let max_delay = Duration::from_secs(60);

    loop {
        if should_shutdown(&shutdown_rx) {
            log::info!("Shutdown signal received");
            break;
        }

        log::info!(
            "Starting polling (retry delay: {}s)...",
            retry_delay.as_secs()
        );

        let listener = teloxide::update_listeners::polling_default(bot.clone()).await;

        let mut builder = Dispatcher::builder(bot.clone(), router::schema())
            .dependencies(dptree::deps![state.clone()])
            .default_handler(|upd| async move {
                log::trace!("Unhandled update: {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text("Bot error"));

        if enable_ctrlc {
            builder = builder.enable_ctrlc_handler();
        }

        let mut dispatcher = builder.build();

        let start = Instant::now();

        tokio::select! {
            _ = dispatcher.dispatch_with_listener(
                listener,
                LoggingErrorHandler::with_custom_text("Listener error"),
            ) => {
                // Short-lived session (< 30s) likely means TerminatedByOtherGetUpdates.
                // Don't reset backoff in that case.
                if start.elapsed() < Duration::from_secs(30) {
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
