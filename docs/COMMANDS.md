# 🎯 Commands - Flarecord API Reference

Complete guide to creating and managing Discord slash commands with Flarecord.

---

## Table of Contents

1. [Command Trait](#command-trait)
2. [Creating a Command](#creating-a-command)
3. [Command Registration](#command-registration)
4. [Trait Methods](#trait-methods)
5. [Best Practices](#best-practices)
6. [Examples](#examples)

---

## Command Trait

The `Command` trait is the core interface for creating custom Discord commands in Flarecord.

### Trait Definition

```rust
#[async_trait(?Send)]
pub trait Command: Send + Sync {
    /// Unique command name (lowercase, no spaces)
    fn name(&self) -> String;
    
    /// Description shown in Discord
    fn description(&self) -> String;
    
    /// Command options/parameters
    fn options(&self) -> BotResult<CommandOptions>;
    
    /// Command execution logic
    async fn execute(
        &self,
        interaction: CommandInteraction,
        ctx: CommandContext,
    ) -> BotResult<CommandResponse>;
}
```

### Why Use Traits?

- **Type Safety**: Compile-time guarantees for command structure
- **Flexibility**: Multiple implementations for different behaviors
- **Testing**: Easy to mock and test individual commands
- **Organization**: Clear separation of concerns

---

## Creating a Command

### Step 1: Define the Command Struct

```rust
pub struct HelloCommand;
```

### Step 2: Implement the Command Trait

```rust
use flarecord::prelude::*;
use async_trait::async_trait;

#[async_trait(?Send)]
impl Command for HelloCommand {
    fn name(&self) -> String {
        "hello".into()
    }

    fn description(&self) -> String {
        "Say hello to someone!".into()
    }

    fn options(&self) -> BotResult<CommandOptions> {
        let user_option = CommandOptionBuilder::user("user", "Who to greet?")
            .required(false)
            .build()?;

        Ok(Some(vec![user_option]))
    }

    async fn execute(
        &self,
        interaction: CommandInteraction,
        ctx: CommandContext,
    ) -> BotResult<CommandResponse> {
        let author = interaction.author()?;

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

### Step 3: Register the Command

```rust
use std::sync::{Arc, LazyLock};
use worker::*;
use flarecord::prelude::*;

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

**See example**: [examples/with_commands/src/lib.rs](../../examples/with_commands/src/lib.rs)

---

## Command Registration

### Using BotBuilder

```rust
let bot = BotBuilder::new()
    .register_command(HelloCommand)
    .register_command(PingCommand)
    .register_command(SearchCommand)
    .build();
```

### Inline Commands (Quick Start)

For simple commands, use inline handlers:

```rust
BotBuilder::new()
    .register_command_handler(
        "ping",
        "Pong!",
        |_interaction, _ctx| async {
            Ok(CommandResponseBuilder::new()
                .content("🏓 Pong!")
                .build())
        }
    )
    .build()
```

**See example**: [examples/simple/src/lib.rs](../../examples/simple/src/lib.rs)

### Auto-Synchronization

Flarecord automatically synchronizes commands with Discord:

```rust
// Call this once per worker lifecycle
BOT.sync_commands_once(&env).await?;

// Or sync manually
BOT.sync_commands(&env).await?;
```

---

## Trait Methods

### 1. `name() -> String`

The unique identifier for your command.

**Rules:**
- Lowercase only
- No spaces (use hyphens: `user-info`)
- Max 32 characters
- Unique across all commands

```rust
fn name(&self) -> String {
    "search".into()
}
```

### 2. `description() -> String`

Help text shown in Discord's command list.

**Guidelines:**
- Keep it short (max 100 characters)
- Describe what the command does
- Be clear and specific

```rust
fn description(&self) -> String {
    "Search for messages or files".into()
}
```

### 3. `options() -> BotResult<CommandOptions>`

Define command parameters.

**Returns:**
- `Ok(None)` - No parameters
- `Ok(Some(vec![...]))` - List of parameters

```rust
fn options(&self) -> BotResult<CommandOptions> {
    let query = CommandOptionBuilder::string("query", "Search term")
        .required(true)
        .build()?;
    
    let limit = CommandOptionBuilder::integer("limit", "Max results")
        .required(false)
        .max_value(50)
        .build()?;
    
    Ok(Some(vec![query, limit]))
}
```

See [OPTIONS.md](./OPTIONS.md) for all option types and builders.

### 4. `execute() -> BotResult<CommandResponse>` (async)

The main command logic.

**Parameters:**
- `interaction` - Info about the slash command and user
- `ctx` - Command context with resolved data

```rust
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    // Your command logic here
    Ok(CommandResponseBuilder::new()
        .content("Command executed!")
        .build())
}
```

See [RESPONSES.md](./RESPONSES.md) for response building.
See [INTERACTIONS.md](./INTERACTIONS.md) for interaction data access.

---

## Best Practices

### 1. Keep Commands Small

Separate complex logic into helper functions:

```rust
#[async_trait(?Send)]
impl Command for SearchCommand {
    // ... trait methods ...
    
    async fn execute(
        &self,
        interaction: CommandInteraction,
        ctx: CommandContext,
    ) -> BotResult<CommandResponse> {
        let query = ctx.data.get_resolved_string("query")?;
        
        // Use helper function
        let results = self.perform_search(&query).await?;
        
        Ok(CommandResponseBuilder::new()
            .content(format!("Found {} results", results.len()))
            .build())
    }
}

impl SearchCommand {
    async fn perform_search(&self, query: &str) -> BotResult<Vec<SearchResult>> {
        // Complex logic here
        Ok(vec![])
    }
}
```

### 2. Handle Errors Gracefully

Always use the `?` operator and provide user feedback:

```rust
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let query = ctx.data.get_resolved_string("query")
        .ok_or(Error::MissingOption("query".into()))?;
    
    if query.is_empty() {
        return Ok(CommandResponseBuilder::new()
            .content("❌ Query cannot be empty")
            .ephemeral()
            .build());
    }
    
    Ok(CommandResponseBuilder::new()
        .content("✅ Searching...")
        .build())
}
```

### 3. Use Ephemeral for Personal Data

Hide sensitive or personal information:

```rust
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let user = ctx.data.get_resolved_user("user")?;
    
    // Only visible to the command invoker
    Ok(CommandResponseBuilder::new()
        .content(format!("User ID: {}", user.id))
        .ephemeral()
        .build())
}
```

### 4. Validate Input Early

Check constraints before processing:

```rust
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let count = ctx.data.get_resolved_integer("count")?;
    
    if count < 1 || count > 100 {
        return Ok(CommandResponseBuilder::new()
            .content("❌ Count must be between 1 and 100")
            .ephemeral()
            .build());
    }
    
    // Valid input, proceed
    Ok(CommandResponseBuilder::new()
        .content(format!("Processing {} items", count))
        .build())
}
```

### 5. Use Descriptive Names

Make it clear what each command does:

```rust
// ✅ Good
fn name(&self) -> String { "user-info".into() }
fn description(&self) -> String { "Get information about a user".into() }

// ❌ Bad
fn name(&self) -> String { "u".into() }
fn description(&self) -> String { "User command".into() }
```

---

## Examples

### Example 1: Simple Ping Command

```rust
pub struct PingCommand;

#[async_trait(?Send)]
impl Command for PingCommand {
    fn name(&self) -> String { "ping".into() }
    
    fn description(&self) -> String { "Check bot latency".into() }
    
    fn options(&self) -> BotResult<CommandOptions> { Ok(None) }
    
    async fn execute(
        &self,
        _interaction: CommandInteraction,
        _ctx: CommandContext,
    ) -> BotResult<CommandResponse> {
        Ok(CommandResponseBuilder::new()
            .content("🏓 Pong!")
            .build())
    }
}
```

### Example 2: Command with User Parameter

```rust
pub struct GreetCommand;

#[async_trait(?Send)]
impl Command for GreetCommand {
    fn name(&self) -> String { "greet".into() }
    
    fn description(&self) -> String { "Greet someone special".into() }
    
    fn options(&self) -> BotResult<CommandOptions> {
        Ok(Some(vec![
            CommandOptionBuilder::user("user", "User to greet")
                .required(true)
                .build()?,
            CommandOptionBuilder::string("message", "Custom greeting")
                .required(false)
                .build()?,
        ]))
    }
    
    async fn execute(
        &self,
        _interaction: CommandInteraction,
        ctx: CommandContext,
    ) -> BotResult<CommandResponse> {
        let user = ctx.data.get_resolved_user("user")?;
        let message = ctx.data.get_resolved_string("message")
            .unwrap_or("Hello there! 👋");
        
        Ok(CommandResponseBuilder::new()
            .content(format!("{}, {}!", message, user.mention()))
            .build())
    }
}
```

### Example 3: Command with Validation

```rust
pub struct RandomCommand;

#[async_trait(?Send)]
impl Command for RandomCommand {
    fn name(&self) -> String { "random".into() }
    
    fn description(&self) -> String { "Generate random number".into() }
    
    fn options(&self) -> BotResult<CommandOptions> {
        Ok(Some(vec![
            CommandOptionBuilder::integer("min", "Minimum")
                .required(false)
                .build()?,
            CommandOptionBuilder::integer("max", "Maximum")
                .required(false)
                .build()?,
        ]))
    }
    
    async fn execute(
        &self,
        _interaction: CommandInteraction,
        ctx: CommandContext,
    ) -> BotResult<CommandResponse> {
        let min = ctx.data.get_resolved_integer("min").unwrap_or(1);
        let max = ctx.data.get_resolved_integer("max").unwrap_or(100);
        
        if min >= max {
            return Ok(CommandResponseBuilder::new()
                .content("❌ Minimum must be less than maximum!")
                .ephemeral()
                .build());
        }
        
        let random = (min..=max).last().unwrap(); // Simplified
        
        Ok(CommandResponseBuilder::new()
            .content(format!("🎲 Random number: {}", random))
            .build())
    }
}
```

---

## See Also

- **[OPTIONS.md](./OPTIONS.md)** - Command parameters and option types
- **[RESPONSES.md](./RESPONSES.md)** - Response building
- **[INTERACTIONS.md](./INTERACTIONS.md)** - Interaction context
- **[Examples](../../examples/)** - Working code examples

---

**Next**: Learn about [Command Options](./OPTIONS.md) to add parameters to your commands! 🚀
