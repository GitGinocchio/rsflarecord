# ❌ Error Handling - Flarecord API Reference

Complete guide to error handling and error types in Flarecord.

---

## Table of Contents

1. [Error Types](#error-types)
2. [BotResult Alias](#botresult-alias)
3. [Handling Errors](#handling-errors)
4. [Best Practices](#best-practices)
5. [Examples](#examples)

---

## Error Types

Flarecord uses the `Error` enum for all error cases:

```rust
pub enum Error {
    // Configuration/Setup Errors
    LockPoisoned,
    MissingHeader(String),
    
    // Input Validation Errors
    MissingOption(String),
    InvalidPayload(String),
    InvalidInteraction(String),
    InvalidOptionName(String),
    InvalidOptionType(String),
    
    // Data Processing Errors
    ResolveError(String),
    
    // Cryptography/Security
    CryptoError(ed25519_dalek::SignatureError),
    ParseHexFailed(FromHexError),
    
    // Data Format Errors
    JsonFailed(serde_json::Error),
    
    // External Service Errors
    WorkerError(String),
    RequestError(String),
    
    // Custom Errors
    Generic(String),
}
```

### Error Categories

| Category | Types | When |
|----------|-------|------|
| **Setup** | `LockPoisoned`, `MissingHeader` | Bot initialization |
| **Validation** | `MissingOption`, `InvalidPayload` | Invalid input |
| **Data** | `ResolveError`, `InvalidOptionType` | Data resolution fails |
| **Security** | `CryptoError`, `ParseHexFailed` | Signature verification fails |
| **Format** | `JsonFailed` | JSON parsing fails |
| **External** | `WorkerError`, `RequestError` | Cloudflare/Discord errors |
| **Custom** | `Generic` | User-defined errors |

---

## BotResult Alias

`BotResult<T>` is a type alias that makes error handling cleaner:

```rust
pub type BotResult<T> = Result<T, Error>;

// Instead of:
fn my_function() -> Result<String, Error> {
    Ok("value".into())
}

// Write:
fn my_function() -> BotResult<String> {
    Ok("value".into())
}
```

---

## Handling Errors

### The ? Operator

The `?` operator automatically converts errors and propagates them:

```rust
// Short - uses ?
let user = ctx.data.get_resolved_user("user")?;

// Equivalent to:
let user = match ctx.data.get_resolved_user("user") {
    Some(u) => u,
    None => return Err(Error::MissingOption("user".into())),
};
```

### .ok_or() for Options

Convert `Option<T>` to `Result<T, Error>`:

```rust
// Convert Some(x) to Ok(x), None to Err
let user = ctx.data.get_resolved_user("user")
    .ok_or(Error::MissingOption("user".into()))?;
```

### .unwrap_or() for Defaults

Use a default value if `None`:

```rust
// Use default if not provided
let limit = ctx.data.get_resolved_integer("limit")
    .unwrap_or(10);
```

### match for Complex Logic

```rust
match ctx.data.get_resolved_user("user") {
    Some(user) => {
        // User provided
        println!("User: {}", user.mention());
    },
    None => {
        // No user provided
        println!("No user specified");
    },
}
```

### if let for Simple Cases

```rust
if let Some(user) = ctx.data.get_resolved_user("user") {
    println!("User: {}", user.mention());
}
```

---

## Best Practices

### 1. Use ? Operator in Commands

```rust
// ✅ Good - Clean and concise
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let user = ctx.data.get_resolved_user("user")?;
    let author = interaction.author()?;
    
    Ok(CommandResponseBuilder::new()
        .content(format!("{} greeted {}", author.mention(), user.mention()))
        .build())
}

// ❌ Bad - Verbose and error-prone
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let user = match ctx.data.get_resolved_user("user") {
        Some(u) => u,
        None => return Err(Error::MissingOption("user".into())),
    };
    
    // ... more boilerplate
}
```

### 2. Provide User-Friendly Error Messages

```rust
// ✅ Good - Clear error message
if count < 1 || count > 100 {
    return Ok(CommandResponseBuilder::new()
        .content("❌ Count must be between 1 and 100")
        .ephemeral()
        .build());
}

// ❌ Bad - Generic error
return Ok(CommandResponseBuilder::new()
    .content("Invalid input")
    .build());
```

### 3. Distinguish Between Errors and Invalid Input

```rust
// Errors: Command fails (return Err)
async fn execute(&self, _: CommandInteraction, _: CommandContext) -> BotResult<CommandResponse> {
    let user = ctx.data.get_resolved_user("user")?;  // Error
    Ok(CommandResponseBuilder::new().content("").build())
}

// Invalid Input: Command succeeds but shows error to user (return Ok)
async fn execute(&self, _: CommandInteraction, ctx: CommandContext) -> BotResult<CommandResponse> {
    let count = ctx.data.get_resolved_integer("count")?;
    
    if count < 1 {
        // Not an error, just invalid input - inform user
        return Ok(CommandResponseBuilder::new()
            .content("❌ Count must be at least 1")
            .ephemeral()
            .build());
    }
    
    Ok(CommandResponseBuilder::new().content("✅ Done").build())
}
```

### 4. Use Default Values for Optional Data

```rust
// ✅ Good - Sensible default
let limit = ctx.data.get_resolved_integer("limit")
    .unwrap_or(10);

// ❌ Bad - No fallback
let limit = ctx.data.get_resolved_integer("limit")?;  // Errors if missing
```

### 5. Log Errors for Debugging

```rust
// When in debug mode, print error details
async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let user = ctx.data.get_resolved_user("user")
        .ok_or_else(|| {
            eprintln!("❌ Error: Missing 'user' option");
            Error::MissingOption("user".into())
        })?;
    
    Ok(CommandResponseBuilder::new().content("✅").build())
}
```

---

## Examples

### Example 1: Handling Missing Required Option

```rust
async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    // This will error if "query" is not provided
    let query = ctx.data.get_resolved_string("query")?;
    
    Ok(CommandResponseBuilder::new()
        .content(format!("Searching: {}", query))
        .build())
}
```

### Example 2: Handling Optional with Default

```rust
async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    // Use default if not provided
    let limit = ctx.data.get_resolved_integer("limit")
        .unwrap_or(10);
    
    Ok(CommandResponseBuilder::new()
        .content(format!("Limit: {}", limit))
        .build())
}
```

### Example 3: Input Validation

```rust
async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let count = ctx.data.get_resolved_integer("count")?;
    
    // Validate range
    if count < 1 || count > 100 {
        return Ok(CommandResponseBuilder::new()
            .content("❌ Count must be 1-100")
            .ephemeral()
            .build());
    }
    
    Ok(CommandResponseBuilder::new()
        .content(format!("✅ Processing {} items", count))
        .build())
}
```

### Example 4: Complex Error Handling

```rust
async fn execute(
    &self,
    interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    // Get required user
    let target = ctx.data.get_resolved_user("user")
        .ok_or(Error::MissingOption("user".into()))?;
    
    // Get optional reason with default
    let reason = ctx.data.get_resolved_string("reason")
        .unwrap_or("No reason provided");
    
    // Get invoker
    let author = interaction.author()?;
    
    // Validate
    if reason.len() > 500 {
        return Ok(CommandResponseBuilder::new()
            .content("❌ Reason too long (max 500 chars)")
            .ephemeral()
            .build());
    }
    
    // Execute
    Ok(CommandResponseBuilder::new()
        .content(format!(
            "⚠️ {} was actioned by {}\nReason: {}",
            target.mention(),
            author.mention(),
            reason
        ))
        .build())
}
```

### Example 5: Match on Error Type

```rust
async fn execute(
    &self,
    _interaction: CommandInteraction,
    ctx: CommandContext,
) -> BotResult<CommandResponse> {
    let value = ctx.data.get_resolved_string("value")
        .ok_or_else(|| Error::MissingOption("value".into()))?;
    
    match my_processing_function(&value).await {
        Ok(result) => Ok(CommandResponseBuilder::new()
            .content(format!("✅ Result: {}", result))
            .build()),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Ok(CommandResponseBuilder::new()
                .content("❌ Processing failed")
                .ephemeral()
                .build())
        }
    }
}

async fn my_processing_function(value: &str) -> BotResult<String> {
    // Your logic here
    Ok(value.to_uppercase())
}
```

---

## Debugging Tips

### Enable Logging

```bash
RUST_LOG=debug wrangler dev
```

### Print Errors

```rust
eprintln!("Error: {:?}", error);
eprintln!("Error message: {}", error);
```

### Check Logs

```bash
wrangler tail --status error
```

---

## See Also

- **[COMMANDS.md](./COMMANDS.md)** - Command creation
- **[RESPONSES.md](./RESPONSES.md)** - Response building
- **[INTERACTIONS.md](./INTERACTIONS.md)** - Interaction data
- **[TROUBLESHOOTING.md](../TROUBLESHOOTING.md)** - Common issues

---

**Next**: Learn about bot configuration in [BOT.md](./BOT.md)! 🚀
