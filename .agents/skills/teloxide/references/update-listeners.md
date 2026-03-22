# teloxide::update_listeners

> Reference documentation for update listeners (polling and webhooks)

## Overview

Telegram supports two ways of getting updates:
1. **Long Polling** - Bot repeatedly asks Telegram for updates
2. **Webhooks** - Telegram pushes updates to bot's HTTP endpoint

## Long Polling

### Default Polling

```rust
use teloxide::prelude::*;

let bot = Bot::from_env();
let listener = teloxide::update_listeners::polling_default(bot.clone());

Dispatcher::builder(bot, schema)
    .build()
    .dispatch_with_listener(listener, LoggingErrorHandler::new())
    .await;
```

### Custom Polling

```rust
use teloxide::update_listeners::Polling;

let listener = Polling::builder(bot.clone())
    .timeout(Duration::from_secs(10))
    .limit(100)
    .allowed_updates(vec![UpdateKind::Message, UpdateKind::CallbackQuery])
    .build();

Dispatcher::builder(bot, schema)
    .build()
    .dispatch_with_listener(listener, LoggingErrorHandler::new())
    .await;
```

### Polling Options

| Option | Description | Default |
|--------|-------------|---------|
| `timeout` | Poll timeout in seconds | 10 |
| `limit` | Max updates per poll | 100 |
| `allowed_updates` | Filter update kinds | All |

## Webhooks

### With Axum (requires `webhooks-axum` feature)

```rust
use teloxide::prelude::*;
use teloxide::update_listeners::webhooks;

let bot = Bot::from_env();
let addr = ([0, 0, 0, 0], 8443).into();
let url = "https://example.com/webhook".parse()?;

let (listener, stop_flag) = webhooks::axum(
    bot.clone(),
    webhooks::Options::new(addr, url),
)
.await?;

Dispatcher::builder(bot, schema)
    .build()
    .dispatch_with_listener(listener, LoggingErrorHandler::new(), stop_flag)
    .await;
```

### Webhook Options

| Option | Description |
|--------|-------------|
| `address` | Socket address to listen on |
| `url` | Public URL for Telegram to send updates to |
| `certificate` | SSL certificate (optional) |

## UpdateListener Trait

```rust
pub trait UpdateListener: Stream<Item = Update> {
    fn stop(&mut self);
    // ...
}
```

## StatefulListener

Create a listener from custom functions:

```rust
use teloxide::update_listeners::{StatefulListener, UpdateListener};

// Create from state and functions
let listener = StatefulListener::new(state, stream_fn, stop_fn);
```
