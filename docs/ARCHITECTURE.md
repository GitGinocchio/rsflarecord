# 🏗️ Architecture Overview

This document explains the internal design and architecture of Flarecord.

## Table of Contents

1. [High-Level Architecture](#high-level-architecture)
2. [Core Components](#core-components)
3. [Module Structure](#module-structure)
4. [Design Patterns](#design-patterns)
5. [Best Practices](#best-practices)

---

## High-Level Architecture

```
┌─────────────────────────────────────────────┐
│       Cloudflare Worker (WASM)              │
├─────────────────────────────────────────────┤
│                                             │
│  ┌──────────────────────────────────────┐  │
│  │      HTTP Request (Discord Event)    │  │
│  └────────────┬─────────────────────────┘  │
│               │                             │
│  ┌────────────▼─────────────────────────┐  │
│  │  Bot::handle(req, env)               │  │
│  │  - Verify signature                  │  │
│  │  - Parse interaction                 │  │
│  └────────────┬─────────────────────────┘  │
│               │                             │
│  ┌────────────▼─────────────────────────┐  │
│  │  Command Dispatch                    │  │
│  │  - Match command name                │  │
│  │  - Build context                     │  │
│  │  - Execute handler                   │  │
│  └────────────┬─────────────────────────┘  │
│               │                             │
│  ┌────────────▼─────────────────────────┐  │
│  │  Build Response                      │  │
│  │  - Serialize to Discord format       │  │
│  │  - Return JSON response              │  │
│  └────────────┬─────────────────────────┘  │
│               │                             │
│  ┌────────────▼─────────────────────────┐  │
│  │     Discord (Response)                │  │
│  └──────────────────────────────────────┘  │
│                                             │
└─────────────────────────────────────────────┘
```

---

## Core Components

### 1. **Bot** (`bot` module)

The main orchestrator that handles all Discord interactions.

#### Responsibilities:
- Verify interaction signatures using ED25519
- Parse incoming Discord requests
- Dispatch commands to appropriate handlers
- Synchronize commands with Discord API
- Return formatted responses

#### Key Types:

```rust
pub struct Bot {
    commands: HashMap<String, Arc<dyn Command>>,
    sync_once: AtomicBool,
}

impl Bot {
    pub async fn handle(&self, req: Request, env: Env) -> Result<Response>
    pub async fn sync_commands_once(&self, env: &Env) -> Result<()>
}
```

#### Key Features:
- **Atomic Command Sync**: Uses `AtomicBool` with `Ordering::Release`/`Acquire` to ensure commands are synced exactly once per worker lifecycle
- **Signature Verification**: Validates Discord's request using public key cryptography
- **Error Resilience**: Comprehensive error handling with typed errors

### 2. **Command Trait** (`traits` module)

The core extension point for custom command logic.

```rust
#[async_trait(?Send)]
pub trait Command: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn options(&self) -> Result<Option<Vec<CommandOption>>>;
    async fn execute(&self, interaction: Interaction, ctx: CommandContext) -> CommandResult;
}
```

#### Implementing Commands:
- Commands must be `Send + Sync` for thread safety
- Use `#[async_trait(?Send)]` to omit Send bound (WASM requirement)
- Return `CommandResult` (alias for `Result<CommandResponse>`)

### 3. **CommandContext** (`models` module)

Provides access to resolved interaction data.

```rust
pub struct CommandContext {
    pub data: InteractionData,
}

impl InteractionData {
    pub fn get_option_<type>(&self, name: &str) -> Option<T>
    pub fn get_resolved_<type>(&self, name: &str) -> Option<T>
}
```

#### Data Resolution Levels:

| Level | Method | Returns | Use Case |
|-------|--------|---------|----------|
| **Raw** | `get_option_user("name")` | `UserId` | Minimal data needed |
| **Resolved** | `get_resolved_user("name")` | `User` or `Member` | Full entity access |

### 4. **ResponseBuilder** (`bot` module)

Fluent API for constructing Discord messages.

```rust
pub struct CommandResponseBuilder {
    content: Option<String>,
    embeds: Vec<Embed>,
    ephemeral: bool,
}

impl CommandResponseBuilder {
    pub fn new() -> Self
    pub fn content(mut self, content: String) -> Self
    pub fn embed(mut self, embed: Embed) -> Self
    pub fn ephemeral(mut self) -> Self
    pub fn build(self) -> CommandResponse
}
```

#### Features:
- **Chaining**: Builder pattern for ergonomic API
- **Deferred Responses**: Support for long-running operations
- **Ephemeral Messages**: Only visible to the command invoker

### 5. **Error Handling** (`error` module)

Typed errors for type-safe error handling.

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Discord API error: {0}")]
    DiscordApi(String),

    #[error("Signature verification failed")]
    InvalidSignature,

    #[error("Invalid command options")]
    InvalidOptions,
    // ... more variants
}
```

---

## Module Structure

```
src/
├── lib.rs                    # Public API exports
├── prelude.rs              # Common imports
│
├── bot/                    # Bot orchestration
│   ├── mod.rs             # Bot struct and methods
│   ├── builder.rs         # BotBuilder
│   └── handler.rs         # Request handling
│
├── traits/                 # Core traits
│   ├── mod.rs
│   ├── command.rs         # Command trait
│   └── builders.rs        # Builder traits
│
├── models/                 # Data models
│   ├── mod.rs
│   ├── interaction.rs     # Interaction types
│   ├── command.rs         # Command options
│   └── response.rs        # Response types
│
├── error.rs               # Error types
│
├── crypto.rs              # ED25519 verification (internal)
│
└── services/              # Internal services
    ├── mod.rs
    └── discord.rs         # Discord API client
```

---

## Design Patterns

### 1. **Builder Pattern**

Used for flexible, readable object construction:

```rust
let response = CommandResponseBuilder::new()
    .content("Hello!".into())
    .ephemeral()
    .build();

let option = CommandOptionBuilder::user("target", "Target user")
    .required(true)
    .build()?;
```

**Benefits**:
- Readable API
- Optional fields without many constructor parameters
- Compile-time guarantee of valid objects

### 2. **Trait-Based Extensibility**

Commands are plugins via trait implementation:

```rust
#[async_trait(?Send)]
impl Command for MyCommand {
    // Custom logic here
}
```

**Benefits**:
- Open/closed principle - extend without modifying core
- Type-safe registration
- Dynamic command discovery

### 3. **Atomic Synchronization**

Commands sync once per worker lifecycle:

```rust
pub async fn sync_commands_once(&self, env: &Env) -> Result<()> {
    if self.sync_once.compare_exchange(
        false,
        true,
        Ordering::Release,
        Ordering::Acquire,
    ).is_ok() {
        // Perform sync
        self.sync_commands(env).await?;
    }
    Ok(())
}
```

**Benefits**:
- No race conditions in concurrent requests
- Single sync per worker cold start
- Minimal API overhead

### 4. **Singleton Pattern**

Use `LazyLock` or `OnceLock` for global bot instance:

```rust
static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    BotBuilder::new()
        .register_command(Hello)
        .register_command(Ping)
        .build()
});
```

**Benefits**:
- Lazy initialization only when needed
- Thread-safe access
- Efficient memory usage

### 5. **Type-Safe Option Resolution**

Two-level API for data access:

```rust
// Raw level - minimal data
if let Some(user_id) = ctx.data.get_option_user("target") {
    // Work with ID
}

// Resolved level - full entity
if let Some(user) = ctx.data.get_resolved_user("target") {
    // Work with full User/Member object
}
```

**Benefits**:
- Explicit intent at call site
- Compile-time type safety
- Clear separation of concerns

---

## Best Practices

### 1. **Response Time**

Discord enforces a 3-second response time limit:

```rust
// ✅ Fast response
async fn execute(&self, interaction: Interaction, ctx: CommandContext) -> CommandResult {
    Ok(CommandResponseBuilder::new()
        .content("Processing...".into())
        .build())
}

// ❌ Slow response (may timeout)
async fn execute(&self, interaction: Interaction, ctx: CommandContext) -> CommandResult {
    let result = expensive_operation().await; // Could exceed 3 seconds
    Ok(CommandResponseBuilder::new()
        .content(format!("Result: {:?}", result))
        .build())
}
```

**Solution**: Use deferred responses for long operations.

### 2. **Error Handling**

Always return `CommandResult` with proper error context:

```rust
// ✅ Good error context
let user = ctx.data.get_resolved_user("target")
    .ok_or(Error::Generic("Target user not found".into()))?;

// ❌ Vague error
let user = ctx.data.get_resolved_user("target")
    .ok_or(Error::Generic("Error".into()))?;
```

### 3. **Command Organization**

Keep commands modular and focused:

```rust
// ✅ Single responsibility
pub struct Greet;
pub struct Ping;
pub struct About;

// ❌ Mixed concerns
pub struct Util;  // Too many responsibilities
```

### 4. **Memory Efficiency**

Leverage WASM optimization for Cloudflare Workers:

```rust
// ✅ Shared command instances
let shared_cmd = Arc::new(MyCommand);

// ❌ Duplicate instances
BotBuilder::new()
    .register_command(MyCommand)
    .register_command(MyCommand)  // Separate instance
```

### 5. **Signature Verification**

Always verify Discord's signatures (handled automatically by Flarecord):

- ED25519 signature on every interaction
- Public key from Cloudflare secrets
- Prevents replay attacks

---

## Data Flow: A Complete Example

```
1. User types: "/hello @Alice"
   ↓
2. Discord sends HTTP POST with interaction JSON
   ↓
3. Flarecord verifies signature
   ↓
4. Bot::handle() parses interaction
   ↓
5. Command dispatch matches "hello"
   ↓
6. Hello::execute() called with:
   - interaction: User interaction details
   - ctx: Resolved command options (User Alice)
   ↓
7. Build response with CommandResponseBuilder
   ↓
8. Serialize to Discord format
   ↓
9. Return HTTP 200 with JSON response
   ↓
10. Discord displays message in channel
```

---

## Performance Considerations

### WASM Execution
- Cold start: ~50-100ms
- Warm execution: <10ms
- Memory footprint: ~1-5MB per instance

### Command Sync
- One-time cost per cold start
- Uses `sync_commands_once()` atomic flag
- API call to Discord (1-2 seconds)

### Interaction Processing
- Signature verification: <1ms
- Command dispatch: <1ms
- Handler execution: Depends on implementation (target: <2s)

---

## Future Enhancements

- 📋 Message component support (buttons, select menus)
- 🎮 Event listeners (reactions, message context menus)
- 💾 Built-in database integrations
- 🔄 Middleware pipeline for request/response
- 📊 Observability and metrics

---

**Related Documentation**:
- [API Reference](./API.md)
- [Getting Started](./GETTING_STARTED.md)
- [Troubleshooting](./TROUBLESHOOTING.md)
