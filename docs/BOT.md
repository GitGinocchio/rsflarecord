# 🤖 Bot Builder - Flarecord API Reference

Complete guide to configuring and initializing your Flarecord bot.

---

## Table of Contents

1. [Overview](#overview)
2. [BotBuilder](#botbuilder)
3. [Registration Methods](#registration-methods)
4. [Command Synchronization](#command-synchronization)
5. [Deployment Patterns](#deployment-patterns)
6. [Best Practices](#best-practices)

---

## Overview

The **BotBuilder** is the main entry point for creating and configuring your Discord bot. It handles:
- Command registration
- Bot initialization
- Command synchronization with Discord
- Request routing

### Quick Start

```rust
let bot = BotBuilder::new()
    .register_command(HelloCommand)
    .register_command(PingCommand)
    .build();
```

---

## BotBuilder

### Creating a Bot

```rust
use flarecord::bot::builder::BotBuilder;

let bot = BotBuilder::new()
    .register_command(MyCommand)
    .build();
```

### Builder Methods

| Method | Purpose | Returns |
|--------|---------|---------|
| `new()` | Create new builder | `BotBuilder` |
| `.register_command(cmd)` | Register `Command` trait impl | `BotBuilder` |
| `.register_command_handler(name, desc, fn)` | Quick inline command | `BotBuilder` |
| `.build()` | Finalize bot | `Bot` |

### Full Initialization

```rust
use flarecord::bot::builder::BotBuilder;
use flarecord::prelude::*;
use std::sync::{Arc, LazyLock};

static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    Arc::new(BotBuilder::new()
        .register_command(HelloCommand)
        .register_command(PingCommand)
        .register_command(SearchCommand)
        .build())
});
```

---

## Registration Methods

### Method 1: Command Trait Implementation

Best for organized, reusable commands:

```rust
#[async_trait(?Send)]
impl Command for HelloCommand {
    fn name(&self) -> String { "hello".into() }
    fn description(&self) -> String { "Say hello!".into() }
    fn options(&self) -> BotResult<CommandOptions> { Ok(None) }
    async fn execute(&self, _: CommandInteraction, _: CommandContext) -> BotResult<CommandResponse> {
        Ok(CommandResponseBuilder::new().content("Hello!").build())
    }
}

// Register
BotBuilder::new()
    .register_command(HelloCommand)
    .build()
```

**See**: [COMMANDS.md](./COMMANDS.md)

### Method 2: Inline Handlers

Quick for simple commands:

```rust
BotBuilder::new()
    .register_command_handler(
        "ping",
        "Respond with pong",
        |_interaction, _ctx| async {
            Ok(CommandResponseBuilder::new()
                .content("🏓 Pong!")
                .build())
        }
    )
    .build()
```

**Use when:**
- Command is very simple
- No complex logic needed
- Quick prototyping

### Multiple Commands

```rust
BotBuilder::new()
    .register_command(HelloCommand)
    .register_command(PingCommand)
    .register_command(SearchCommand)
    .register_command(HelpCommand)
    .build()
```

---

## Command Synchronization

Flarecord automatically syncs commands with Discord. This means:
1. Bot registers all commands with Discord on startup
2. Commands appear in `/command` autocomplete
3. Updates are pushed on redeployment

### Sync Once Pattern (Production)

Sync commands once per worker lifecycle for efficiency:

```rust
#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Sync only once - subsequent calls are no-op
    BOT.sync_commands_once(&env).await?;
    
    // Handle request
    BOT.handle(req, env).await
}
```

### Manual Sync

Force a sync at any time:

```rust
BOT.sync_commands(&env).await?;
```

### Environment Variables Needed

For synchronization, you need in your worker:
- `DISCORD_BOT_TOKEN` - Your bot's token
- `DISCORD_BOT_APPLICATION_ID` - Your app's ID
- `DISCORD_PUBLIC_KEY` - For signature verification

Set these as secrets:

```bash
wrangler secret put DISCORD_BOT_TOKEN
wrangler secret put DISCORD_BOT_APPLICATION_ID
wrangler secret put DISCORD_PUBLIC_KEY
```

---

## Deployment Patterns

### Pattern 1: Lazy Initialization (Recommended)

Create bot once, reuse for all requests:

```rust
use std::sync::{Arc, LazyLock};
use worker::*;

static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    Arc::new(BotBuilder::new()
        .register_command(HelloCommand)
        .build())
});

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    BOT.sync_commands_once(&env).await?;
    BOT.handle(req, env).await
}
```

**Benefits:**
- Bot created only once
- Better performance
- Lower memory usage
- More efficient

**See example**: [examples/with_commands/src/lib.rs](../../examples/with_commands/src/lib.rs)

### Pattern 2: Inline Creation (Quick Start)

Create fresh bot for each request:

```rust
#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let bot = BotBuilder::new()
        .register_command_handler("ping", "Pong!", |_, _| async {
            Ok(CommandResponseBuilder::new().content("Pong!").build())
        })
        .build();
    
    bot.handle(req, env).await
}
```

**Benefits:**
- Simple for prototyping
- No static state

**Drawbacks:**
- Bot recreated for every request
- Less efficient
- Higher resource usage

**See example**: [examples/simple/src/lib.rs](../../examples/simple/src/lib.rs)

---

## Best Practices

### 1. Use Lazy Initialization in Production

```rust
// ✅ Production
static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    Arc::new(BotBuilder::new()
        .register_command(MyCommand)
        .build())
});

// ❌ Not for production
#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let bot = BotBuilder::new()
        .register_command(MyCommand)
        .build();
    bot.handle(req, env).await
}
```

### 2. Call Sync Once

```rust
// ✅ Good
BOT.sync_commands_once(&env).await?;

// ❌ Bad - Inefficient
BOT.sync_commands(&env).await?;  // Syncs on every request
```

### 3. Group Related Commands

Organize commands by domain:

```rust
// ✅ Good organization
BotBuilder::new()
    // Moderation commands
    .register_command(WarningCommand)
    .register_command(KickCommand)
    .register_command(BanCommand)
    // Utility commands
    .register_command(HelpCommand)
    .register_command(StatsCommand)
    // Info commands
    .register_command(ServerInfoCommand)
    .register_command(UserInfoCommand)
    .build()
```

### 4. Separate Command Logic

Keep command implementations separate:

```
src/
├── lib.rs              // Bot setup
└── commands/
    ├── mod.rs          // Command module
    ├── ping.rs         // PingCommand
    ├── hello.rs        // HelloCommand
    └── admin/
        ├── mod.rs
        ├── warn.rs     // WarnCommand
        └── kick.rs     // KickCommand
```

### 5. Document Commands

Add docstrings to command structs:

```rust
/// Says hello to a user
/// 
/// # Example
/// `/hello @alice` → "Hello alice!"
pub struct HelloCommand;
```

---

## Troubleshooting

### "Bot not responding"

1. Verify `sync_commands_once()` is called
2. Check Discord has the command registered:
   - Open Discord settings
   - Regenerate bot token if needed
3. View worker logs:
   ```bash
   wrangler tail
   ```

### "Command sync failed"

Verify these environment variables are set:

```bash
wrangler secret list
# Should show:
# DISCORD_BOT_TOKEN
# DISCORD_BOT_APPLICATION_ID
# DISCORD_PUBLIC_KEY
```

### "Interaction Failed"

1. Verify `DISCORD_PUBLIC_KEY` is correct
2. Check worker is running:
   ```bash
   wrangler deployments list
   ```
3. View error logs:
   ```bash
   wrangler tail --status error
   ```

---

## See Also

- **[COMMANDS.md](./COMMANDS.md)** - Creating commands
- **[GETTING_STARTED.md](../GETTING_STARTED.md)** - Setup guide
- **[Examples](../../examples/)** - Working examples
- **[TROUBLESHOOTING.md](../TROUBLESHOOTING.md)** - Common issues

---

**Next**: Learn about the architecture in [ARCHITECTURE.md](./ARCHITECTURE.md)! 🚀
