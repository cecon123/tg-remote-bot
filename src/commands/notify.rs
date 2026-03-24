use teloxide::prelude::*;
use teloxide::types::ChatId;

#[allow(dead_code)]
pub fn start_login_watcher(
    _bot: Bot,
    _chat_id: ChatId,
    _tx: tokio::sync::mpsc::UnboundedSender<String>,
) {
}
