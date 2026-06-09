# 🔧 WebAssembly Considerations - Flarecord

Important information about running Flarecord as WebAssembly on Cloudflare Workers.

---

## Table of Contents

1. [WASM Basics](#wasm-basics)
2. [Cloudflare Workers Environment](#cloudflare-workers-environment)
3. [Building for WASM](#building-for-wasm)
4. [WASM Limitations](#wasm-limitations)
5. [Performance Tips](#performance-tips)
6. [Debugging WASM](#debugging-wasm)

---

## WASM Basics

**WASM** (WebAssembly) is a binary instruction format that runs at near-native speed. Flarecord compiles Rust to WASM to run on Cloudflare Workers.

### Why WASM?

| Benefit | Impact |
|---------|--------|
| **Speed** | Near-native performance |
| **Portability** | Runs anywhere (browsers, servers, edge) |
| **Security** | Sandboxed execution |
| **Size** | Compact binary (typically <1MB) |
| **Language Support** | Works with Rust, C++, Go, etc. |

### Compilation Pipeline

```
Rust Source Code
    ↓
Rust Compiler (rustc)
    ↓
WASM Binary (.wasm)
    ↓
Cloudflare Worker Runtime
    ↓
Executes on Cloudflare Edge Network
```

---

## Cloudflare Workers Environment

Cloudflare Workers is a serverless platform optimized for WASM.

### Key Features

**Global Distribution:**
- Runs on 200+ edge locations worldwide
- Sub-millisecond latency from users
- No server management needed

**Execution Model:**
- Runs WASM inside V8 JavaScript engine
- Event-driven (HTTP requests)
- Automatic scaling

**Limits:**
- Max execution time: 30 seconds (CPU-bound)
- Max memory: 128 MB
- Max binary size: 1 MB (core limit, 10 MB with modules)
- No file I/O (except KV storage)
- No persistent connections

### Worker Runtime

Flarecord uses the `worker` crate to interact with the Cloudflare runtime:

```rust
use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Your bot code here
    Ok(Response::ok("Hello!"))
}
```

---

## Building for WASM

### Prerequisites

Install the WASM target:

```bash
rustup target add wasm32-unknown-unknown
```

### Build Commands

**Debug build:**
```bash
cargo build --target wasm32-unknown-unknown
```

**Release build (optimized):**
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Verifying Build

Check the output binary size:

```bash
# On Linux/macOS
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# On Windows PowerShell
Get-Item target\wasm32-unknown-unknown\release\*.wasm | Select-Object Length
```

**Target:** < 1 MB for optimal cold starts

### Reducing Binary Size

If your binary is too large:

```bash
# 1. Use release profile
cargo build --target wasm32-unknown-unknown --release

# 2. Enable LTO (Link Time Optimization)
# Add to Cargo.toml:
[profile.release]
lto = true
codegen-units = 1

# 3. Use wasm-opt (optional)
npm install -g wasm-opt
wasm-opt -Oz target/wasm32-unknown-unknown/release/your_bot.wasm \
  -o target/wasm32-unknown-unknown/release/your_bot_optimized.wasm
```

---

## WASM Limitations

Important constraints when using WASM:

### 1. No Standard Library Features

Some Rust standard library features don't work in WASM:

**❌ Not available:**
- `std::thread` - No threading
- `std::fs` - No file system access
- `std::process` - No subprocess spawning
- System time (use `web_time`)
- DNS resolution

**✅ Available:**
- `std::collections` (HashMap, Vec, etc.)
- `std::sync` (Mutex, Arc, etc.)
- Async/await
- Serialization (serde)

### 2. No FFI to System Libraries

Cannot call system C libraries directly. Use web-friendly crates instead:

```rust
// ❌ Won't work in WASM
use std::time::SystemTime;

// ✅ Use web-compatible crates
use js_sys::Date;  // For current time
```

### 3. Async Runtime

Flarecord uses `tokio` but with WASM-compatible features:

```toml
# In Cargo.toml
tokio = { version = "1", features = ["sync", "time"] }
# Note: No "full" feature
```

### 4. Memory Constraints

Maximum 128 MB memory available:

```rust
// ✅ Be mindful of allocations
let data = Vec::with_capacity(1000);  // Pre-allocate

// ❌ Avoid large allocations
let huge = vec![0; 100_000_000];  // Likely OOM
```

### 5. No Persistent Storage

Workers can't store data locally. Options for persistence:

- **Cloudflare KV** - Key-value store
- **Cloudflare Durable Objects** - State and event streaming
- **External Database** - PostgreSQL, MongoDB, etc.

---

## Performance Tips

### 1. Use Lazy Initialization

Initialize expensive resources once:

```rust
use std::sync::LazyLock;

static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    Arc::new(BotBuilder::new()
        .register_command(MyCommand)
        .build())
});

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // BOT initialized only once
    BOT.handle(req, env).await
}
```

### 2. Avoid Cold Starts

Cold starts are slower but unavoidable. Minimize initialization work:

```rust
// ✅ Fast initialization
static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    Arc::new(BotBuilder::new()
        .register_command(HelloCommand)
        .build())
});

// ❌ Slow initialization
#[event(fetch)]
async fn fetch(...) -> Result<Response> {
    let bot = BotBuilder::new()
        .register_command(HelloCommand)
        .build();  // Created on every request
    // ...
}
```

### 3. Optimize Dependencies

Each dependency adds to binary size. Review `Cargo.toml`:

```bash
# Check which dependencies contribute most to size
cargo tree

# Remove unused dependencies
cargo tree --duplicates
```

**Heavy crates to avoid in WASM:**
- `tokio-full` (use features instead)
- Large regex engines
- Full cryptography libraries

### 4. Response Time

Discord requires responses within 3 seconds:

```rust
// ✅ Fast response
Ok(CommandResponseBuilder::new()
    .content("Processing...")
    .build())

// ❌ Slow (may timeout)
async fn execute(&self, ...) -> BotResult<CommandResponse> {
    let result = expensive_api_call().await;  // Might exceed 3s
    Ok(CommandResponseBuilder::new()
        .content(format!("{:?}", result))
        .build())
}
```

### 5. Use Arc for Shared State

Efficiently share data between commands:

```rust
// ✅ Shared reference
let config = Arc::new(config_data);

// ❌ Cloned data
let config = config_data.clone();  // Expensive
```

---

## Debugging WASM

### Enable Logging

Set up logging for WASM:

```rust
// In your entry point
use worker::console_log;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("Request: {:?}", req.method());
    
    // Your code
    
    Ok(Response::ok("Done"))
}
```

**View logs:**
```bash
wrangler tail
```

### Print Statements

Use `eprintln!` for debugging:

```rust
eprintln!("Debug info: {:?}", variable);
```

**View in logs:**
```bash
wrangler tail
```

### Local Testing

Test locally before deploying:

```bash
# Install Miniflare (local Cloudflare emulator)
npm install -g miniflare

# Run locally
wrangler dev

# Test bot in Discord (should connect to local worker)
```

### Panic Messages

Panics show up in logs:

```bash
wrangler tail --status error
```

---

## Cargo Configuration

### Essential Cargo.toml Settings

```toml
[package]
name = "my-bot"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]  # WebAssembly library

[dependencies]
worker = "^0.8"
flarecord = { git = "https://github.com/GitGinocchio/flarecord-rs" }
async-trait = "^0.1"
serde = { version = "^1.0", features = ["derive"] }
serde_json = "^1.0"

[profile.release]
opt-level = "z"          # Optimize for size
lto = true               # Link-time optimization
strip = true             # Strip symbols
codegen-units = 1        # Better optimization
```

### Build Target

Always specify the WASM target:

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

## See Also

- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - System architecture
- **[BOT.md](./BOT.md)** - Bot initialization
- **[GETTING_STARTED.md](../GETTING_STARTED.md)** - Setup guide
- **[Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)**
- **[Rust WASM Book](https://rustwasm.org/book/)**

---

**Next**: Check out the [Examples](../../examples/)! 🚀
