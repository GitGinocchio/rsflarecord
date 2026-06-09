# 📤 Command Responses - Flarecord API Reference

Complete guide to building Discord responses with Flarecord.

---

## Table of Contents

1. [Overview](#overview)
2. [Response Types](#response-types)
3. [CommandResponseBuilder](#commandresponsebuilder)
4. [Visibility Modes](#visibility-modes)
5. [Best Practices](#best-practices)
6. [Examples](#examples)

---

## Overview

**Responses** are what your bot sends back to Discord when a command is executed. They can be simple text, rich embeds, files, or interactive components.

### Response Constraints

Discord has specific requirements for responses:
- Text messages: **max 2000 characters**
- Embeds: **max 10 per message**
- Response time: **max 3 seconds** (use deferred for longer operations)
- Visibility: Public or ephemeral (private)

---

## Response Types

### Basic Text Response

```rust
CommandResponseBuilder::new()
    .content("Hello, world!")
    .build()
```

**Result:**
```
Hello, world!
```

### Response with Embed

```rust
CommandResponseBuilder::new()
    .content("Here's the data:")
    .embed(
        EmbedBuilder::new()
            .title("Title")
            .description("Description")
            .color(0x0099ff)
            .build()?
    )
    .build()
```

### Response with Attachment

```rust
CommandResponseBuilder::new()
    .content("Check this file!")
    .attachment(AttachmentBuilder::new()
        .filename("data.txt")
        .data(vec![1, 2, 3])
        .build()?)
    .build()
```

---

## CommandResponseBuilder

Build type-safe responses using the fluent builder pattern.

### Fluent API

```rust
CommandResponseBuilder::new()
    .content("Message text")           // Main message (optional)
    .ephemeral()                       // Hide from others (optional)
    .deferred()                        // Long operation (optional)
    .embed(embed)                      // Add embed (optional)
    .attachment(attachment)            // Add file (optional)
    .build()                           // Finalize
```

### Methods Reference

| Method | Purpose | Example |
|--------|---------|---------|
| `.content(text)` | Set message text (max 2000) | `.content("Hello!")` |
| `.ephemeral()` | Make response private | `.ephemeral()` |
| `.deferred()` | Defer for long operations | `.deferred()` |
| `.embed(embed)` | Add embed | `.embed(my_embed)` |
| `.attachment(file)` | Add file | `.attachment(my_file)` |
| `.build()` | Finalize | `.build()` |

---

## Visibility Modes

### Public Response (Default)

Visible to everyone in the channel:

```rust
CommandResponseBuilder::new()
    .content("This everyone can see!")
    .build()
```

### Ephemeral (Private) Response

Visible only to the command invoker:

```rust
CommandResponseBuilder::new()
    .content("This is just for you!")
    .ephemeral()
    .build()
```

**Use cases:**
- Personal data (user IDs, emails)
- Error messages
- Sensitive information
- Confirmation dialogs

### Deferred Response

For operations taking longer than 3 seconds:

```rust
CommandResponseBuilder::new()
    .deferred()
    .build()

// Then perform long operation
let data = fetch_data().await;

// Edit response later (via webhook)
// Implementation depends on your setup
```

---

## Best Practices

### 1. Keep Messages Concise

Discord messages are limited to 2000 characters:

```rust
// ✅ Good
CommandResponseBuilder::new()
    .content("✅ 5 items processed successfully")
    .build()

// ❌ Bad - Too verbose
CommandResponseBuilder::new()
    .content("This is a very long message that explains everything about what happened...")
    .build()
```

### 2. Use Emojis for Visual Clarity

```rust
// ✅ Good - Clear status
CommandResponseBuilder::new()
    .content("✅ Success!\n❌ Error occurred\n⚠️ Warning!\n🔄 Processing...")
    .build()

// ❌ Bad - No visual distinction
CommandResponseBuilder::new()
    .content("Success. Error occurred. Warning. Processing...")
    .build()
```

### 3. Mark Personal Data as Ephemeral

```rust
// ✅ Good - Only visible to user
CommandResponseBuilder::new()
    .content(format!("Your ID: {}", user_id))
    .ephemeral()
    .build()

// ❌ Bad - Public personal data
CommandResponseBuilder::new()
    .content(format!("Your ID: {}", user_id))
    .build()
```

### 4. Use Embeds for Complex Data

```rust
// ✅ Good - Structured
CommandResponseBuilder::new()
    .embed(
        EmbedBuilder::new()
            .title("User Info")
            .field("Username", "john#1234", true)
            .field("ID", "123456789", true)
            .build()?
    )
    .build()

// ❌ Bad - Unstructured
CommandResponseBuilder::new()
    .content("Username: john#1234, ID: 123456789")
    .build()
```

### 5. Validate Before Responding

```rust
// ✅ Good - Validate input
let count = ctx.data.get_resolved_integer("count")?;

if count < 1 || count > 100 {
    return Ok(CommandResponseBuilder::new()
        .content("❌ Count must be between 1 and 100")
        .ephemeral()
        .build());
}

// Process...

// ❌ Bad - No validation
let count = ctx.data.get_resolved_integer("count")?;
// Process immediately
```

---

## Examples

### Example 1: Success Response

```rust
async fn execute(
    &self,
    _interaction: CommandInteraction,
    _ctx: CommandContext,
) -> BotResult<CommandResponse> {
    Ok(CommandResponseBuilder::new()
        .content("✅ Operation completed successfully!")
        .build())
}
```

### Example 2: Error Response

```rust
async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let value = ctx.data.get_resolved_string("value")?;
    
    if value.is_empty() {
        return Ok(CommandResponseBuilder::new()
            .content("❌ Value cannot be empty!")
            .ephemeral()
            .build());
    }
    
    Ok(CommandResponseBuilder::new()
        .content("✅ Saved!")
        .build())
}
```

### Example 3: User Info Response

```rust
async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let user = ctx.data.get_resolved_user("user")?;
    
    Ok(CommandResponseBuilder::new()
        .content(format!("👤 {}", user.mention()))
        .ephemeral()  // Only visible to invoker
        .build())
}
```

### Example 4: Deferred Response (Long Operation)

```rust
async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let query = ctx.data.get_resolved_string("query")?;
    
    // Respond immediately
    // Note: Actual deferred message editing requires webhook implementation
    Ok(CommandResponseBuilder::new()
        .content(format!("🔍 Searching for '{}'...", query))
        .build())
}
```

---

## Future Features

### Message Components (Coming Soon)

Buttons, dropdowns, and text inputs will be supported:

```rust
// ⏳ Not yet available
CommandResponseBuilder::new()
    .content("Click a button:")
    .component(ButtonBuilder::new()
        .label("Click me!")
        .build()?)
    .build()
```

### Modals (Coming Soon)

Modal dialogs for user input:

```rust
// ⏳ Not yet available
ModalBuilder::new()
    .title("User Info")
    .field("Name", "text")
    .field("Email", "email")
    .build()
```

---

## See Also

- **[COMMANDS.md](./COMMANDS.md)** - Command creation
- **[OPTIONS.md](./OPTIONS.md)** - Command parameters
- **[INTERACTIONS.md](./INTERACTIONS.md)** - Interaction data
- **[Examples](../../examples/)** - Working code examples

---

**Next**: Learn about interaction data in [INTERACTIONS.md](./INTERACTIONS.md)! 🚀
