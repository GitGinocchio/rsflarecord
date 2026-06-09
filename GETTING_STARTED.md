# 🚀 Getting Started with Flarecord

This guide will walk you through creating your first Discord bot with Flarecord on Cloudflare Workers.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Environment Setup](#environment-setup)
4. [Creating Your First Bot](#creating-your-first-bot)
5. [Deploying to Cloudflare Workers](#deploying-to-cloudflare-workers)
6. [Testing Your Bot](#testing-your-bot)

---

## Prerequisites

Before getting started, ensure you have:

- **Rust 1.85+** - [Install Rustup](https://rustup.rs/)
- **Node.js 18+** - Required for Wrangler
- **Wrangler CLI** - Cloudflare's deployment tool
  ```bash
  npm install -g @cloudflare/wrangler
  ```
- **Discord Developer Account** - [Create one here](https://discord.com/developers/applications)

### Verify Installation

```bash
# Check Rust version
rustc --version

# Check Wrangler version
wrangler --version

# Check Node version
node --version
```

---

## Installation

### Step 1: Create a New Rust Project

```bash
cargo new my-discord-bot
cd my-discord-bot
```

### Step 2: Configure for Cloudflare Workers

Add Wrangler configuration in the project root:

```bash
wrangler init
```

This creates a `wrangler.toml` file. Update it:

```toml
name = "my-discord-bot"
type = "javascript"
account_id = "YOUR_ACCOUNT_ID"
workers_dev = true

[env.production]
route = "example.com/bot"
zone_id = "YOUR_ZONE_ID"
```

### Step 3: Update Cargo.toml

Add Flarecord and required dependencies:

```toml
[dependencies]
flarecord = { git = "https://github.com/GitGinocchio/flarecord-rs" }
worker = "^0.8"
reqwest = { version = "^0.13", features = ["json"] }
async-trait = "^0.1"
serde = { version = "^1.0", features = ["derive"] }
serde_json = "^1.0"
thiserror = "^2.0"

[lib]
crate-type = ["cdylib"]
```

### Step 4: Install Dependencies

```bash
cargo build --target wasm32-unknown-unknown
```

---

## Environment Setup

### Create a Discord Application

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Click "New Application"
3. Name your application (e.g., "MyBot")
4. Go to "Bot" tab and click "Add Bot"
5. Copy the **Bot Token** and **Application ID**

### Configure Cloudflare Worker Secrets

Store sensitive values as secrets:

```bash
wrangler secret put DISCORD_BOT_TOKEN
wrangler secret put DISCORD_BOT_APPLICATION_ID
wrangler secret put DISCORD_PUBLIC_KEY
```

Paste the values when prompted.

### Get Discord Public Key

1. In the Developer Portal, go to General Information
2. Copy the **Public Key**
3. Use it in the secret command above

### Add Bot to Your Server

1. Go to "OAuth2" → "URL Generator"
2. Select scopes: `bot`
3. Select permissions: `Send Messages`, `Use Slash Commands`, `Embed Links`
4. Copy the generated URL and open it in your browser
5. Select your test server

---

## Creating Your First Bot

### Project Structure

```
my-discord-bot/
├── src/
│   ├── lib.rs           # Entry point
│   └── commands/
│       └── hello.rs     # Your first command
├── Cargo.toml
├── wrangler.toml
└── .env                 # (optional) Local env vars
```

### Step 1: Create the Hello Command

Create `src/commands/hello.rs`:

```rust
use flarecord::prelude::*;
use async_trait::async_trait;

pub struct Hello;

#[async_trait(?Send)]
impl Command for Hello {
    fn name(&self) -> String {
        "hello".into()
    }

    fn description(&self) -> String {
        "Say hello to someone!".into()
    }

    fn options(&self) -> Result<Option<Vec<CommandOption>>> {
        let user_option = CommandOptionBuilder::user("user", "the user to greet")
            .required(false)
            .build()?;

        Ok(Some(vec![user_option]))
    }

    async fn execute(
        &self,
        interaction: Interaction,
        ctx: CommandContext,
    ) -> CommandResult {
        let author = interaction
            .author()
            .ok_or(Error::Generic("Missing author".into()))?;

        let message = match ctx.data.get_resolved_user("user") {
            Some(user) => format!("Hello {}, {} greeted you!", user.mention(), author.mention()),
            None => format!("Hello {}!", author.mention()),
        };

        Ok(CommandResponseBuilder::new()
            .content(message)
            .build())
    }
}
```

### Step 2: Set Up the Entry Point

Update `src/lib.rs`:

```rust
use std::sync::{Arc, LazyLock};
use worker::*;
use flarecord::prelude::*;

mod commands;
use commands::hello::Hello;

static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    BotBuilder::new()
        .register_command(Hello)
        .build()
});

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Sync commands once per worker lifecycle
    BOT.sync_commands_once(&env).await?;

    // Handle the interaction
    BOT.handle(req, env).await
}
```

### Step 3: Update Module Declaration

In `src/lib.rs`, ensure modules are declared:

```rust
mod commands {
    pub mod hello;
}
```

---

## Deploying to Cloudflare Workers

### Build the Project

```bash
# Build for WASM target
cargo build --target wasm32-unknown-unknown --release

# Publish to Cloudflare
wrangler publish
```

The CLI will output your worker's URL:

```
✓ Deployed to https://my-discord-bot.your-name.workers.dev
```

### Configure Discord Webhook

1. Go to Discord Developer Portal → Your Application
2. In "General Information", scroll to "Interactions Endpoint URL"
3. Enter your worker URL: `https://my-discord-bot.your-name.workers.dev`
4. Discord will send a verification request (automatic via Flarecord)
5. Click "Save Changes"

---

## Testing Your Bot

### 1. Invoke the Command in Discord

In any channel where your bot has permissions:

```
/hello
/hello @username
```

### 2. View Logs

```bash
# Stream live logs
wrangler tail

# Filter by level
wrangler tail --status error
```

### 3. Local Testing (Optional)

For local development without deploying:

```bash
# Use Miniflare for local Cloudflare emulation
cargo install miniflare

# Run locally
wrangler dev
```

---

## Common Issues

### "Interaction Failed"
- Verify `DISCORD_PUBLIC_KEY` is correct
- Check bot token is valid
- Ensure worker URL is configured in Discord settings

### "Command Not Appearing"
- Wait 1-2 minutes for command sync
- Verify `sync_commands_once()` is called
- Check bot permissions in server

### "Command Timeout"
- Ensure bot code responds within 3 seconds
- Use ephemeral responses for long operations
- Implement deferred responses for complex logic

For more troubleshooting, see [TROUBLESHOOTING.md](./TROUBLESHOOTING.md).

---

## Next Steps

- 📚 Learn about [Core Architecture](./ARCHITECTURE.md)
- 📖 Read [API Documentation](./API.md)
- 🤝 Check [Contributing Guide](./CONTRIBUTING.md)
- 💡 Explore [Examples](../examples/)

---

**Congratulations!** You've created your first Flarecord bot. 🎉
