# teloxide::errors

> Reference documentation for error types

## Error Types

### RequestError

Errors caused by sending requests to Telegram.

```rust
use teloxide::errors::RequestError;

match bot.send_message(chat_id, text).await {
    Err(RequestError::Api(api_err)) => { /* API error */ }
    Err(RequestError::Network(err)) => { /* Network error */ }
    Err(RequestError::RetryAfter(secs)) => { /* Rate limited */ }
    Err(RequestError::MigrateToChatId(new_id)) => { /* Chat migrated */ }
    _ => {}
}
```

### ApiError

A kind of API error returned by Telegram.

```rust
use teloxide::errors::ApiError;

match err {
    ApiError::BotBlocked => { /* Bot blocked by user */ }
    ApiError::BotKicked => { /* Bot kicked from group */ }
    ApiError::UserDeactivated => { /* User deactivated */ }
    ApiError::NotEnoughRights => { /* Insufficient permissions */ }
    ApiError::MessageToEditNotFound => { /* Message not found */ }
    ApiError::MessageIsNotModified => { /* Content unchanged */ }
    _ => { /* Other API error */ }
}
```

### DownloadError

Errors caused by downloading files.

```rust
use teloxide::errors::DownloadError;

match bot.download_file(&path, &mut writer).await {
    Err(DownloadError::Io(io_err)) => { /* IO error */ }
    Err(DownloadError::Network(req_err)) => { /* Network error */ }
    _ => {}
}
```

### AsResponseParameters

Trait for extracting response parameters from errors.

## Error Handling Patterns

### With LoggingErrorHandler

```rust
use teloxide::prelude::*;

// In Dispatcher
.error_handler(LoggingErrorHandler::with_custom_text("Bot error"))
```

### With OnError trait

```rust
use teloxide::error_handlers::OnError;

result.log_on_error().await;
```

### Custom Error Handler

```rust
use teloxide::dispatching::UpdateHandler;

fn error_handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync>> {
    dptree::endpoint(|err: Box<dyn std::error::Error + Send + Sync>| async move {
        log::error!("Handler error: {:?}", err);
        respond(())
    })
}
```
