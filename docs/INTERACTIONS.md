# 💬 Interactions - Flarecord API Reference

Complete guide to accessing and using interaction data in Flarecord.

---

## Table of Contents

1. [Overview](#overview)
2. [CommandInteraction](#commandinteraction)
3. [CommandContext](#commandcontext)
4. [CommandData](#commanddata)
5. [Best Practices](#best-practices)
6. [Examples](#examples)

---

## Overview

**Interactions** represent the Discord user's input and context. They contain information about who ran the command, where it was run, and what parameters they provided.

### Interaction Data Flow

```
Discord User runs: /hello @alice
          ↓
Discord API sends interaction
          ↓
Flarecord verifies signature
          ↓
Bot calls your execute()
          ↓
You receive:
  - interaction: CommandInteraction (who, where, when)
  - ctx: CommandContext (what, resolved data)
```

---

## CommandInteraction

The `CommandInteraction` contains metadata about the command execution.

### Available Information

```rust
pub struct CommandInteraction {
    pub id: u64,                      // Interaction ID
    pub application_id: u64,          // Your bot's application ID
    pub guild_id: Option<u64>,        // Server ID (if in guild)
    pub channel_id: u64,              // Channel ID where command was run
    pub author_id: u64,               // User ID who ran command
    // ... more fields
}
```

### Getting User Information

```rust
// Get the user who ran the command
let author = interaction.author()?;
println!("{}", author.mention());      // @username
println!("{}", author.id);             // 123456789
println!("{}", author.username);       // username (without @)
```

### Getting Guild/Channel Information

```rust
// Get server ID
if let Some(guild_id) = interaction.guild_id {
    println!("Server ID: {}", guild_id);
}

// Get channel ID
println!("Channel ID: {}", interaction.channel_id);
```

### Checking Execution Context

```rust
// Is the command running in a guild?
if interaction.guild_id.is_some() {
    println!("Command run in server");
} else {
    println!("Command run in DM");
}
```

---

## CommandContext

The `CommandContext` bundles the interaction with resolved data and provides convenient accessors.

### Structure

```rust
pub struct CommandContext {
    pub data: CommandData,
    pub interaction: CommandInteraction,
}
```

### Accessing Command Parameters

Use `ctx.data` to get options the user provided:

```rust
// String option
let query = ctx.data.get_resolved_string("query")?;

// Integer option
let count = ctx.data.get_resolved_integer("count")?;

// Boolean option
let enabled = ctx.data.get_resolved_boolean("enabled")?;

// User option (resolved)
let user = ctx.data.get_resolved_user("user")?;

// Channel option (resolved)
let channel = ctx.data.get_resolved_channel("channel")?;

// Role option (resolved)
let role = ctx.data.get_resolved_role("role")?;

// Attachment option
let attachment = ctx.data.get_resolved_attachment("file")?;
```

---

## CommandData

The `CommandData` contains all resolved parameters for your command.

### Resolution Methods

All methods return `Option<T>` - use `.ok_or()` or `.unwrap_or()` to handle:

```rust
// Method 1: Explicit error handling
let user = ctx.data.get_resolved_user("user")
    .ok_or(Error::MissingOption("user".into()))?;

// Method 2: Default value
let limit = ctx.data.get_resolved_integer("limit")
    .unwrap_or(10);

// Method 3: Chain operations
let query = ctx.data.get_resolved_string("query")
    .map(|q| q.to_lowercase())
    .ok_or(Error::MissingOption("query".into()))?;
```

### Accessing User Information

```rust
let user = ctx.data.get_resolved_user("user")?;

// User properties
println!("Mention: {}", user.mention());        // @username
println!("ID: {}", user.id);                   // 123456789
println!("Username: {}", user.username);       // username
println!("Is bot: {}", user.bot);              // true/false
```

### Accessing Channel Information

```rust
let channel = ctx.data.get_resolved_channel("channel")?;

// Channel properties
println!("Channel ID: {}", channel.id);
println!("Channel name: {:?}", channel.name);
```

### Accessing Role Information

```rust
let role = ctx.data.get_resolved_role("role")?;

// Role properties
println!("Role ID: {}", role.id);
println!("Role name: {}", role.name);
println!("Role color: {:?}", role.color);
```

---

## Best Practices

### 1. Always Check Required Data

```rust
// ✅ Good
let user = ctx.data.get_resolved_user("user")
    .ok_or(Error::MissingOption("user".into()))?;

// ❌ Bad - Panics if user not found
let user = ctx.data.get_resolved_user("user").unwrap();
```

### 2. Provide Defaults for Optional Data

```rust
// ✅ Good
let limit = ctx.data.get_resolved_integer("limit")
    .unwrap_or(10);

// ❌ Bad - No fallback
let limit = ctx.data.get_resolved_integer("limit")?;
```

### 3. Use Pattern Matching When Useful

```rust
// ✅ Good
match ctx.data.get_resolved_user("user") {
    Some(user) => println!("User: {}", user.mention()),
    None => println!("No user specified"),
}

// ✅ Also good
if let Some(user) = ctx.data.get_resolved_user("user") {
    println!("User: {}", user.mention());
}
```

### 4. Validate Resolved Data

```rust
let user = ctx.data.get_resolved_user("target")?;

// Additional validation
if user.bot {
    return Ok(CommandResponseBuilder::new()
        .content("❌ Cannot target bot users")
        .ephemeral()
        .build());
}
```

### 5. Access Interaction Info When Needed

```rust
// Get the command invoker
let author = interaction.author()?;

// Get server context
let guild_id = interaction.guild_id
    .ok_or(Error::Generic("Command must be run in a server".into()))?;

// Get channel context
let channel_id = interaction.channel_id;
```

---

## Examples

### Example 1: Using User Parameters

```rust
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let target_user = ctx.data.get_resolved_user("user")?;
    let author = interaction.author()?;
    
    Ok(CommandResponseBuilder::new()
        .content(format!(
            "{} was greeted by {}",
            target_user.mention(),
            author.mention()
        ))
        .build())
}
```

### Example 2: Using String Parameters

```rust
async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let query = ctx.data.get_resolved_string("query")?;
    let limit = ctx.data.get_resolved_integer("limit")
        .unwrap_or(10);
    
    Ok(CommandResponseBuilder::new()
        .content(format!(
            "🔍 Searching for '{}' (limit: {})",
            query, limit
        ))
        .build())
}
```

### Example 3: Guild-Only Command

```rust
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    // Require guild context
    let guild_id = interaction.guild_id
        .ok_or(Error::Generic("This command only works in servers".into()))?;
    
    let member = ctx.data.get_resolved_user("member")?;
    
    Ok(CommandResponseBuilder::new()
        .content(format!(
            "📌 User {} in guild {}",
            member.mention(),
            guild_id
        ))
        .build())
}
```

### Example 4: Complex Interaction

```rust
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    // Get invoker
    let author = interaction.author()?;
    
    // Get parameters
    let target = ctx.data.get_resolved_user("user")?;
    let reason = ctx.data.get_resolved_string("reason")?;
    
    // Optional parameter with default
    let severity = ctx.data.get_resolved_integer("severity")
        .unwrap_or(1);
    
    // Validate
    if severity < 1 || severity > 10 {
        return Ok(CommandResponseBuilder::new()
            .content("❌ Severity must be 1-10")
            .ephemeral()
            .build());
    }
    
    // Build response
    Ok(CommandResponseBuilder::new()
        .content(format!(
            "⚠️ Action against {} (severity: {})\n\
             Action by: {}\n\
             Reason: {}",
            target.mention(), severity, author.mention(), reason
        ))
        .build())
}
```

### Example 5: Optional Channel Parameter

```rust
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let channel = match ctx.data.get_resolved_channel("channel") {
        Some(ch) => ch,
        None => {
            // Use current channel as default
            format!("<#{}>", interaction.channel_id)
        },
    };
    
    Ok(CommandResponseBuilder::new()
        .content(format!("📢 Using channel: {}", channel))
        .build())
}
```

---

## See Also

- **[COMMANDS.md](./COMMANDS.md)** - Command creation
- **[OPTIONS.md](./OPTIONS.md)** - Command parameters
- **[RESPONSES.md](./RESPONSES.md)** - Building responses
- **[Examples](../../examples/)** - Working code examples

---

**Next**: Learn about error handling in [ERRORS.md](./ERRORS.md)! 🚀
