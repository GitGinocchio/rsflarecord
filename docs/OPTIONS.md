# 📝 Command Options - Flarecord API Reference

Complete guide to command options and parameters in Flarecord.

---

## Table of Contents

1. [Overview](#overview)
2. [Option Types](#option-types)
3. [CommandOptionBuilder](#commandoptionbuilder)
4. [Builder Methods](#builder-methods)
5. [Best Practices](#best-practices)
6. [Examples](#examples)

---

## Overview

**Command Options** are parameters that your slash commands accept. They appear as input fields, dropdowns, or menus in Discord.

### Option Flow

```
User runs: /search query:"discord" limit:20
                    ↓
Discord parses parameters
                    ↓
Flarecord resolves options
                    ↓
Your execute() method receives resolved data
                    ↓
You access via: ctx.data.get_resolved_*()
```

---

## Option Types

Flarecord supports all Discord slash command option types:

| Type | Method | Discord Type | Example |
|------|--------|--------------|---------|
| String | `::string()` | Text input | Names, queries |
| Integer | `::integer()` | Number | Counts, IDs |
| Boolean | `::boolean()` | Toggle | Flags, yes/no |
| User | `::user()` | User mention | @user picker |
| Channel | `::channel()` | Channel mention | #channel picker |
| Role | `::role()` | Role mention | @role picker |
| Attachment | `::attachment()` | File upload | Images, files |

---

## CommandOptionBuilder

The `CommandOptionBuilder` creates type-safe command options using the builder pattern.

### String Option

```rust
CommandOptionBuilder::string("name", "Your name")
    .required(true)
    .min_value(1)
    .max_value(50)
    .build()?
```

**Access in command:**
```rust
let name = ctx.data.get_resolved_string("name")?;
```

### Integer Option

```rust
CommandOptionBuilder::integer("count", "How many?")
    .required(false)
    .min_value(1)
    .max_value(100)
    .build()?
```

**Access in command:**
```rust
let count = ctx.data.get_resolved_integer("count").unwrap_or(10);
```

### Boolean Option

```rust
CommandOptionBuilder::boolean("enabled", "Enable feature?")
    .required(false)
    .build()?
```

**Access in command:**
```rust
let enabled = ctx.data.get_resolved_boolean("enabled").unwrap_or(true);
```

### User Option

```rust
CommandOptionBuilder::user("user", "Select a user")
    .required(true)
    .build()?
```

**Access in command:**
```rust
let user = ctx.data.get_resolved_user("user")?;
println!("{}", user.mention()); // @username
```

### Channel Option

```rust
CommandOptionBuilder::channel("channel", "Select a channel")
    .required(false)
    .build()?
```

**Access in command:**
```rust
let channel = ctx.data.get_resolved_channel("channel")?;
```

### Role Option

```rust
CommandOptionBuilder::role("role", "Select a role")
    .required(false)
    .build()?
```

**Access in command:**
```rust
let role = ctx.data.get_resolved_role("role")?;
```

### Attachment Option

```rust
CommandOptionBuilder::attachment("file", "Upload a file")
    .required(true)
    .build()?
```

**Access in command:**
```rust
let attachment = ctx.data.get_resolved_attachment("file")?;
```

---

## Builder Methods

### Common Methods

All option builders support these methods:

| Method | Purpose | Example |
|--------|---------|---------|
| `.required(bool)` | Make option required/optional | `.required(true)` |
| `.min_value(i64)` | Set minimum value | `.min_value(1)` |
| `.max_value(i64)` | Set maximum value | `.max_value(100)` |
| `.build()` | Finalize the option | `.build()?` |

### String-Specific Methods

```rust
CommandOptionBuilder::string("query", "Search term")
    .required(true)
    .min_value(1)           // Minimum length
    .max_value(100)         // Maximum length
    .build()?
```

### Integer-Specific Methods

```rust
CommandOptionBuilder::integer("count", "Number")
    .required(false)
    .min_value(1)           // Minimum value
    .max_value(1000)        // Maximum value
    .build()?
```

---

## Best Practices

### 1. Keep Option Names Descriptive

```rust
// ✅ Good
CommandOptionBuilder::string("search_query", "What to search for?")

// ❌ Bad
CommandOptionBuilder::string("q", "Query")
```

### 2. Set Appropriate Constraints

```rust
// ✅ Good - Prevents invalid input
CommandOptionBuilder::integer("age", "Your age")
    .required(true)
    .min_value(0)
    .max_value(150)
    .build()?

// ❌ Bad - No validation
CommandOptionBuilder::integer("age", "Your age")
    .build()?
```

### 3. Make Options Required When Necessary

```rust
// Required: This command needs the query
CommandOptionBuilder::string("query", "Search term")
    .required(true)
    .build()?

// Optional: Has a sensible default
CommandOptionBuilder::integer("limit", "Max results")
    .required(false)
    .build()?
```

### 4. Use Descriptive Descriptions

Descriptions appear in Discord's help text:

```rust
// ✅ Clear and helpful
CommandOptionBuilder::string("username", "Discord username (without @)")
    .required(true)
    .build()?

// ❌ Vague
CommandOptionBuilder::string("name", "Name")
    .required(true)
    .build()?
```

### 5. Validate in Your Command

Even with constraints, always validate:

```rust
let count = ctx.data.get_resolved_integer("count")?;

if count < 1 || count > 100 {
    return Ok(CommandResponseBuilder::new()
        .content("❌ Count must be between 1 and 100")
        .ephemeral()
        .build());
}
```

---

## Examples

### Example 1: Search Command

```rust
fn options(&self) -> BotResult<CommandOptions> {
    Ok(Some(vec![
        CommandOptionBuilder::string("query", "What to search for?")
            .required(true)
            .min_value(1)
            .max_value(100)
            .build()?,
        CommandOptionBuilder::integer("limit", "Max results (1-50)")
            .required(false)
            .min_value(1)
            .max_value(50)
            .build()?,
    ]))
}

async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let query = ctx.data.get_resolved_string("query")?;
    let limit = ctx.data.get_resolved_integer("limit").unwrap_or(10);
    
    Ok(CommandResponseBuilder::new()
        .content(format!("Searching for '{}' (limit: {})", query, limit))
        .build())
}
```

### Example 2: Warning System

```rust
fn options(&self) -> BotResult<CommandOptions> {
    Ok(Some(vec![
        CommandOptionBuilder::user("user", "User to warn")
            .required(true)
            .build()?,
        CommandOptionBuilder::string("reason", "Warning reason")
            .required(true)
            .min_value(5)
            .max_value(500)
            .build()?,
        CommandOptionBuilder::integer("duration", "Duration in hours (0 = permanent)")
            .required(false)
            .min_value(0)
            .max_value(720)  // 30 days
            .build()?,
    ]))
}

async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let target_user = ctx.data.get_resolved_user("user")?;
    let reason = ctx.data.get_resolved_string("reason")?;
    let duration = ctx.data.get_resolved_integer("duration").unwrap_or(0);
    
    let message = format!(
        "⚠️ {} warned by {} (duration: {} hours)\nReason: {}",
        target_user.mention(),
        interaction.author()?.mention(),
        if duration == 0 { "permanent".into() } else { duration.to_string() },
        reason
    );
    
    Ok(CommandResponseBuilder::new()
        .content(message)
        .build())
}
```

### Example 3: Config Command

```rust
fn options(&self) -> BotResult<CommandOptions> {
    Ok(Some(vec![
        CommandOptionBuilder::string("setting", "Setting name")
            .required(true)
            .build()?,
        CommandOptionBuilder::string("value", "New value")
            .required(true)
            .min_value(1)
            .max_value(200)
            .build()?,
        CommandOptionBuilder::boolean("apply_global", "Apply to all channels?")
            .required(false)
            .build()?,
    ]))
}

async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let setting = ctx.data.get_resolved_string("setting")?;
    let value = ctx.data.get_resolved_string("value")?;
    let apply_global = ctx.data.get_resolved_boolean("apply_global").unwrap_or(false);
    
    Ok(CommandResponseBuilder::new()
        .content(format!(
            "✅ Set {} = {} (global: {})",
            setting, value, apply_global
        ))
        .build())
}
```

---

## See Also

- **[COMMANDS.md](./COMMANDS.md)** - Command trait and creation
- **[RESPONSES.md](./RESPONSES.md)** - Building responses
- **[INTERACTIONS.md](./INTERACTIONS.md)** - Accessing interaction data
- **[Examples](../../examples/)** - Working code examples

---

**Next**: Learn how to build responses in [RESPONSES.md](./RESPONSES.md)! 🚀
