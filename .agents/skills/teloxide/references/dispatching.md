# teloxide::dispatching

> Reference documentation for the dispatching system

## Overview

The dispatching module provides an update dispatching model based on `dptree`. It uses a declarative chain of responsibility pattern with dependency injection.

## Key Concepts

- **Handler Chain**: Composable chain of handlers using `dptree`
- **Branching**: `a.branch(b)` - try `a`, if it neglects update, try `b`
- **Pattern Matching**: `dptree::case!` macro for filtering enums
- **Endpoints**: Final handler functions with `dptree::Handler::endpoint`
- **Dependency Injection**: Parameters auto-injected based on handler signature

## Core Types

### Dispatcher

The base for update dispatching.

```rust
use teloxide::prelude::*;

let bot = Bot::from_env();
let handler = Update::filter_message().branch(
    Message::filter_text().endpoint(|bot: Bot, msg: Message| async move {
        respond(())
    })
);

Dispatcher::builder(bot, handler)
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
```

### DispatcherBuilder

Builder pattern for configuring Dispatcher.

| Method | Description |
|--------|-------------|
| `dependencies(deps)` | Add DI dependencies |
| `enable_ctrlc_handler()` | Enable Ctrl+C handling |
| `default_handler(handler)` | Handler for unhandled updates |
| `error_handler(handler)` | Handler for errors |

### UpdateHandler

```rust
type UpdateHandler<E> = dptree::Handler<DependencyMap, ResponseResult<()>, E>;
```

## Filter Traits

### UpdateFilterExt

Filters on `Update` type.

| Method | Injects |
|--------|---------|
| `filter_message()` | `Message` |
| `filter_edited_message()` | `Message` |
| `filter_channel_post()` | `Message` |
| `filter_callback_query()` | `CallbackQuery` |
| `filter_inline_query()` | `InlineQuery` |
| `filter_chosen_inline_result()` | `ChosenInlineResult` |

### MessageFilterExt

Filters on `Message` type.

| Method | Injects |
|--------|---------|
| `filter_text()` | `String` |
| `filter_photo()` | `Vec<PhotoSize>` |
| `filter_document()` | `Document` |
| `filter_command::<C>()` | `C` |
| `filter_map(fn)` | Custom |

### HandlerExt

Extension methods for `dptree` handlers.

| Method | Description |
|--------|-------------|
| `filter(fn)` | Filter by function |
| `filter_map(fn)` | Filter and map |
| `map(fn)` | Transform |
| `endpoint(fn)` | Final handler |

## Command Filtering

```rust
use teloxide::prelude::*;
use teloxide::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start,
    Help,
    #[command(parse_with = "split")]
    Greet(String),
}

// Simple command filter
let handler = teloxide::filter_command::<Command, _>()
    .branch(case![Command::Start].endpoint(start))
    .branch(case![Command::Help].endpoint(help));

// With mention (e.g., /start@my_bot)
let handler = teloxide::filter_mention_command::<Command, _>();
```

## Dialogue System

### Storage Types

```rust
use teloxide::dispatching::dialogue::{InMemStorage, Dialogue};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
```

### enter()

```rust
use teloxide::dispatching::dialogue;

dialogue::enter::<Update, InMemStorage<State>, State, _>()
    .branch(message_handler)
    .branch(callback_query_handler)
```

### State Transitions

```rust
// Update state
dialogue.update(State::WaitingForName).await?;

// Exit dialogue
dialogue.exit().await?;
```
