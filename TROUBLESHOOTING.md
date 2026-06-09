# 🔧 Troubleshooting Guide

Common issues and their solutions for Flarecord bots.

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [Deployment Issues](#deployment-issues)
3. [Bot Not Responding](#bot-not-responding)
4. [Commands Not Appearing](#commands-not-appearing)
5. [Signature Verification Errors](#signature-verification-errors)
6. [Performance Issues](#performance-issues)
7. [WASM Build Issues](#wasm-build-issues)

---

## Installation Issues

### "error: linker not found" during cargo build

**Problem**: Rust can't find the C linker for WASM compilation.

**Solutions**:

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Verify installation
rustc --target wasm32-unknown-unknown --version
```

### "Command not found: wrangler"

**Problem**: Wrangler CLI is not installed or not in PATH.

**Solutions**:

```bash
# Install globally via npm
npm install -g @cloudflare/wrangler

# Or install locally
npm install --save-dev @cloudflare/wrangler

# Verify
wrangler --version
```

### Cargo.lock conflicts

**Problem**: Dependencies won't resolve correctly.

**Solutions**:

```bash
# Clean and rebuild
cargo clean
cargo build --target wasm32-unknown-unknown

# Update dependencies
cargo update
```

---

## Deployment Issues

### "No account ID configured"

**Problem**: Wrangler can't find your Cloudflare account.

**Solutions**:

```bash
# Configure via wrangler login
wrangler login

# Or set in wrangler.toml
[env.default]
account_id = "YOUR_ACCOUNT_ID"
```

Get your account ID from [Cloudflare Dashboard](https://dash.cloudflare.com).

### "Error: Workers product is not available"

**Problem**: Your Cloudflare account doesn't have Workers enabled.

**Solutions**:

- Upgrade to Cloudflare Workers plan
- Check account permissions
- Verify email is confirmed

### "Worker script too large"

**Problem**: Compiled WASM binary exceeds 1MB limit.

**Solutions**:

```bash
# Build with optimizations
cargo build --target wasm32-unknown-unknown --release

# Check binary size
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# Remove debug symbols
wasm-opt -Oz input.wasm -o output.wasm
```

Install `wasm-opt`:
```bash
npm install -g wasm-opt
```

---

## Bot Not Responding

### "Interaction Failed" in Discord

**Problem**: Bot isn't responding to commands.

**Checklist**:

- [ ] Worker URL is configured in Discord settings
- [ ] Worker is deployed and running
- [ ] Bot has permission to send messages
- [ ] Check worker logs

**Debugging**:

```bash
# Stream logs
wrangler tail

# View with filtering
wrangler tail --status error
wrangler tail --format json | grep "error"
```

### Command Works Locally but Not in Discord

**Problem**: Bot works with `wrangler dev` but not in production.

**Solutions**:

```bash
# Verify production deployment
wrangler deployments list

# Check environment variables
wrangler env list

# Verify secrets are set
wrangler secret list
```

---

## Commands Not Appearing

### Command Registered but Not Visible

**Problem**: Command doesn't show up in Discord command list.

**Checklist**:

- [ ] Wait 1-2 minutes after deployment
- [ ] Restart Discord client
- [ ] Verify bot has "applications.commands" scope
- [ ] Check command name (lowercase, no spaces)

**Solutions**:

```bash
# Force command sync
curl -X POST https://discord.com/api/v10/applications/{app_id}/commands \
  -H "Authorization: Bot {token}" \
  -H "Content-Type: application/json" \
  -d '{"name":"hello","description":"Say hello","type":1}'
```

Replace `{app_id}` and `{token}` with your values.

### Command Name Conflicts

**Problem**: Multiple commands with the same name.

**Solutions**:

- Ensure unique command names in `BotBuilder`
- Check for conflicts with global Discord commands
- Use `wrangler tail` to see registration attempts

---

## Signature Verification Errors

### "Signature verification failed"

**Problem**: Discord requests are being rejected.

**Checklist**:

- [ ] Public Key is correct in Cloudflare secrets
- [ ] `DISCORD_PUBLIC_KEY` env var is set
- [ ] Public Key matches Discord Developer Portal

**Debugging**:

```bash
# Verify secret is set
wrangler secret list

# Check Public Key format (hex string, 64 chars)
echo $DISCORD_PUBLIC_KEY | wc -c

# Should output: 65 (64 chars + newline)
```

**Solution**:

1. Go to Discord Developer Portal
2. General Information → Copy Public Key
3. Update secret:
   ```bash
   wrangler secret put DISCORD_PUBLIC_KEY
   # Paste key at prompt
   ```

---

## Performance Issues

### "Command timed out"

**Problem**: Bot takes >3 seconds to respond.

**Causes**:

- Slow external API calls
- Heavy computation in execute()
- Network latency
- WASM cold start

**Solutions**:

1. **Use Deferred Responses**:
   ```rust
   Ok(CommandResponseBuilder::new()
       .deferred()
       .build())
   // Then call edit_response later
   ```

2. **Optimize Command Logic**:
   ```rust
   // ❌ Slow - blocking work in handler
   let data = fetch_data().await; // 2+ seconds
   
   // ✅ Fast - defer and handle async
   Ok(CommandResponseBuilder::new()
       .deferred()
       .build())
   // Follow up asynchronously
   ```

3. **Minimize Cold Starts**:
   - Use `LazyLock` for bot initialization
   - Keep command handlers small
   - Optimize WASM binary size

### High Memory Usage

**Problem**: Worker crashes due to memory limits.

**Solutions**:

```bash
# Check binary size
cargo build --target wasm32-unknown-unknown --release
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# Profile memory usage
wasm-bindgen-cli --version

# Reduce dependencies
# Review Cargo.toml for unused crates
```

---

## WASM Build Issues

### "error: module did not export memory"

**Problem**: WASM module is missing required exports.

**Solution**:

```toml
# In Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
worker = "^0.8"
```

### "error: unknown format"

**Problem**: Build target is incorrect.

**Solution**:

```bash
# Use correct WASM target
cargo build --target wasm32-unknown-unknown

# Not:
cargo build --target wasm32-wasi
```

### "error: can't find crate for 'std'"

**Problem**: Standard library features not available for WASM.

**Solutions**:

```toml
# Use wasm-compatible crates
[dependencies]
async-trait = "^0.1"  # Works with WASM
tokio = { version = "^1", features = ["rt"] }  # Enable features
```

Avoid:
- `std::thread` (no threading in WASM)
- System-dependent crates
- File I/O operations

---

## Discord API Issues

### "Invalid token"

**Problem**: Bot token is invalid or expired.

**Solution**:

1. Generate new token in Developer Portal
2. Update secret:
   ```bash
   wrangler secret put DISCORD_BOT_TOKEN
   ```

### "Forbidden - Invalid permissions"

**Problem**: Bot lacks required permissions.

**Solution**:

1. In Developer Portal → OAuth2 → URL Generator
2. Select required scopes and permissions:
   - `bot`
   - `applications.commands`
   - `chat.write`
   - Others as needed
3. Re-invite bot with new permissions

### "Rate limited"

**Problem**: Too many API requests to Discord.

**Solution**:

- Implement request throttling
- Cache command data
- Use Discord's rate limit headers:

```rust
if let Some(retry_after) = response.headers().get("retry-after") {
    // Wait before retrying
}
```

---

## Getting Help

### Debug Information to Include

When reporting issues, include:

```bash
# Rust version
rustc --version

# Wrangler version
wrangler --version

# Worker logs
wrangler tail

# Binary size
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# Error messages (full stack trace)
```

### Resources

- [Flarecord Issues](https://github.com/GitGinocchio/flarecord-rs/issues)
- [Discord Developer Docs](https://discord.com/developers/docs)
- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers)
- [Rust WASM Book](https://rustwasm.org/book/)

---

**Still stuck?** [Open an issue](https://github.com/GitGinocchio/flarecord-rs/issues) with your debug information.
