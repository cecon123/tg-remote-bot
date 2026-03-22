---
name: teloxide
description: "Documentation for teloxide crate. Keywords: telegram, bot, api, webhook, polling, dispatcher, handler, message, callback, inline, command, dialogue, dptree, repl, BotCommands"
---

# teloxide

> **Version:** 0.17.0 | **Source:** docs.rs

## Overview

A full-featured framework for building Telegram bots in Rust. Handles the Telegram Bot API (v9.1) so you can focus on business logic. Uses `dptree` for declarative update dispatching with dependency injection.

```toml
# Minimal setup
teloxide = { version = "0.17", features = ["macros"] }

# With webhooks support (axum-based)
teloxide = { version = "0.17", features = ["macros", "webhooks-axum"] }

# Full features
teloxide = { version = "0.17", features = ["full"] }
```

## Quick Start

### REPL (Simple)

```rust
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
```

### Dispatcher (Advanced)

```rust
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    Dispatcher::builder(bot, Update::filter_message().branch(
        Message::filter_text().endpoint(|bot: Bot, msg: Message| async move {
            bot.send_message(msg.chat.id, "Hello!").await?;
            respond(())
        })
    ))
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
```

## Key Types

### Bot

The requests sender. Created from environment variable `TELOXIDE_TOKEN`.

```rust
let bot = Bot::from_env();
let bot = Bot::new("your-token-here");
```

### Message / Update

Core Telegram types. `Update` is the top-level wrapper; `Message` represents a single message.

```rust
// Filter for text messages, inject String
Message::filter_text().endpoint(|bot: Bot, msg: Message, text: String| async move {
    // handle text
    respond(())
})
```

### BotCommands (derive macro)

```rust
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands:")]
enum Command {
    #[command(description = "display help")]
    Help,
    #[command(description = "start interaction")]
    Start,
}
```

## Dispatching System

### Handler Chain (dptree)

The dispatching model uses `dptree` for composable handler chains with dependency injection.

```rust
fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Start].endpoint(start));

    Update::filter_message()
        .branch(command_handler)
        .branch(Message::filter_text().endpoint(text_handler))
}

// Handler functions receive dependencies via DI
async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}
```

### Filter Methods

| Filter | Injected Type |
|--------|---------------|
| `Update::filter_message()` | `Message` |
| `Update::filter_callback_query()` | `CallbackQuery` |
| `Update::filter_inline_query()` | `InlineQuery` |
| `Message::filter_text()` | `String` |
| `Message::filter_command::<C>()` | `C` |
| `Message::filter_map(\|m\| ...)` | Custom |

### Dispatcher Builder

```rust
Dispatcher::builder(bot, handler)
    .dependencies(dptree::deps![storage, config])  // DI dependencies
    .enable_ctrlc_handler()                          // Ctrl+C handling
    .default_handler(|upd| async move {              // Unhandled updates
        log::warn!("Unhandled update: {:?}", upd);
    })
    .error_handler(LoggingErrorHandler::with_custom_text("Error"))  // Error handling
    .build()
    .dispatch()
    .await;
```

## Dialogues (State Management)

```rust
use teloxide::dispatching::dialogue::{self, InMemStorage, Dialogue};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
enum State {
    #[default]
    Start,
    WaitingForName,
    WaitingForAge { name: String },
}

// In handler:
async fn handler(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::WaitingForName).await?;
    // ...
    Ok(())
}

// In Dispatcher setup:
dialogue::enter::<Update, InMemStorage<State>, State, _>()
    .branch(message_handler)
```

## Update Listeners

### Long Polling (default)

```rust
let listener = teloxide::update_listeners::polling_default(bot);
Dispatcher::builder(bot, schema).build().dispatch_with_listener(listener, ...).await;
```

### Webhooks (requires `webhooks-axum` feature)

```rust
use teloxide::update_listeners::webhooks;

let addr = ([0, 0, 0, 0], 8443).into();
let url = "https://example.com/webhook".parse()?;

let (listener, _) = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
    .await?;

Dispatcher::builder(bot, schema).build().dispatch_with_listener(listener, ...).await;
```

## Error Handling

```rust
use teloxide::errors::{ApiError, RequestError, DownloadError};

match bot.send_message(chat_id, text).await {
    Err(RequestError::Api(ApiError::BotBlocked)) => { /* blocked */ }
    Err(RequestError::Network(e)) => { /* network error */ }
    Ok(msg) => { /* success */ }
    _ => { /* other error */ }
}
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `macros` | `#[derive(BotCommands)]` macro |
| `ctrlc_handler` | Ctrl+C handling (default) |
| `webhooks` | Webhook utilities |
| `webhooks-axum` | Axum-based webhook server |
| `throttle` | `Throttle` bot adaptor |
| `cache-me` | `CacheMe` bot adaptor |
| `trace-adaptor` | `Trace` bot adaptor |
| `erased` | `ErasedRequester` adaptor |
| `redis-storage` | Redis dialogue storage |
| `sqlite-storage-*` | SQLite dialogue storage |
| `native-tls` | native-tls (default) |
| `rustls` | rustls TLS |

## Common Patterns

### Sending Messages

```rust
bot.send_message(chat_id, "Hello").await?;
bot.send_photo(chat_id, InputFile::url(url)).await?;
bot.send_document(chat_id, InputFile::file("file.pdf")).await?;
bot.send_dice(chat_id).await?;
bot.answer_callback_query(query.id).await?;
```

### Inline Keyboard

```rust
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

let keyboard = InlineKeyboardMarkup::new(vec![
    vec![InlineKeyboardButton::callback("Yes", "yes"), InlineKeyboardButton::callback("No", "no")],
]);

bot.send_message(chat_id, "Choose:").reply_markup(keyboard).await?;
```

### Bot Commands

```rust
use teloxide::prelude::*;
use teloxide::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start,
    Help,
    #[command(parse_with = "split", description = "greet <name>")]
    Greet(String),
}
```

## Documentation

- `./references/dispatching.md` - Dispatching system
- `./references/errors.md` - Error types
- `./references/update-listeners.md` - Polling and webhooks

## Links

- [docs.rs](https://docs.rs/teloxide)
- [crates.io](https://crates.io/crates/teloxide)
- [GitHub](https://github.com/teloxide/teloxide)
- [Examples](https://github.com/teloxide/teloxide/tree/master/crates/teloxide/examples)
- [DPTREE Guide](https://github.com/teloxide/teloxide/blob/master/DPTREE_GUIDE.md)
