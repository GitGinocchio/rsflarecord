
# Flarecord

**Flarecord** is a lightweight, type-safe, and asynchronous framework for building Discord bots on **Cloudflare Workers**. Built with Rust, it leverages WASM for high performance and strict type-safety, ensuring your bot is both fast and reliable.

## 🚀 Getting Started

### 1. Requirements

* [Rust](https://rustup.rs/) installed (version 1.85+ recommended).
* [Wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/) installed.

### 2. Setup

Add `flarecord` to your `Cargo.toml`:

```toml
[dependencies]
flarecord = { git = "https://github.com/GitGinocchio/flarecord-rs" }

```

### 3. Build your first bot

Implementing a command is simple using the `Command` trait.

```rust
use flarecord::prelude::*;
use async_trait::async_trait;

pub struct Hello;

#[async_trait(?Send)]
impl Command for Hello {
    fn name(&self) -> String { "hello".into() }
    fn description(&self) -> String { "Say hi to someone!".into() }

    async fn execute(&self, interaction: Interaction, ctx: CommandContext) -> CommandResult {
        let author = interaction.author().ok_or(Error::Generic("Missing author".into()))?;
        let user = ctx.data.get_resolved_user("user");

        let message = match user {
            Some(user) => format!("Hello {0}, {1} greeted you", user.mention(), author.mention()),
            None => format!("Hello {0}!", author.name)
        };
        
        Ok(CommandResponseBuilder::new()
            .content(message)
            .ephemeral()
            .build())
    }
}

```

### 4. Entry Point

Configure your worker to handle incoming interactions:

```rust
use std::sync::{Arc, LazyLock};
use flarecord::bot::{Bot, builder::BotBuilder};

// Initialize the Bot instance only when necessary after worker cold starts
static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    BotBuilder::new()
        .register_command(Hello)
        .build()
});

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Automatically syncs commands if not already done
    BOT.sync_commands_once(&env).await?;
    
    BOT.handle(req, env).await
}

```

## ✨ Key Features

* **Type-Safe Interactions**: Use `get_option_<type>` and `get_resolved_<type>` to extract data safely.
* **Automatic Command Sync**: Never manually update your slash commands again; `sync_commands_once` handles it for you.
* **WASM Optimized**: Designed specifically for the Cloudflare Workers runtime.
* **Ergonomic API**: A builder pattern for responses and a trait-based system for commands.

## 🛠 Interaction Data Access

Flarecord provides a clean API to retrieve interaction data. Distinguish between raw input and resolved entities:

| Method | Purpose |
| --- | --- |
| `get_option_user("name")` | Retrieves the raw **User ID** input. |
| `get_resolved_user("name")` | Retrieves the **full entity** (User/Member) from Discord's cache. |

## 🔑 Environment Variables

Configure these secrets in your Cloudflare Worker dashboard:

* `DISCORD_BOT_TOKEN`: Your bot's token.
* `DISCORD_BOT_APPLICATION_ID`: Your app ID.
* `DISCORD_PUBLIC_KEY`: Used for verifying interaction signatures.

---

### Pro-tips for contributors

* **Syncing**: The `sync_commands_once` method uses an atomic flag (`Ordering::Release`/`Acquire`) to ensure the synchronization only happens once per worker instance lifecycle, preventing unnecessary API overhead.
* **Efficiency**: Use `OnceLock` or `LazyLock` for your `Bot` instance to leverage the singleton pattern across worker requests.

## Credits

Inspired by [stateless-discord-bot](https://github.com/siketyan/stateless-discord-bot)