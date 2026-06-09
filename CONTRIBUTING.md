# 🤝 Contributing to Flarecord

Thank you for your interest in contributing to Flarecord! This guide will help you get started.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Making Changes](#making-changes)
5. [Submitting Pull Requests](#submitting-pull-requests)
6. [Coding Standards](#coding-standards)
7. [Testing](#testing)

---

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors. This includes:

- **Respect** - Treat all contributors with kindness and respect
- **Inclusivity** - Welcome contributions from people of all backgrounds
- **Professionalism** - Keep discussions focused and constructive
- **Safety** - Report violations to the maintainers

### Unacceptable Behavior

The following are not tolerated in any form:
- Harassment or discrimination based on identity
- Threats or intentional harm
- Spam or off-topic content
- Sharing private information without consent

---

## Getting Started

### 1. Fork the Repository

```bash
# Fork on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/flarecord-rs.git
cd flarecord-rs
```

### 2. Create a Feature Branch

```bash
# Create a new branch for your feature
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/bug-description
```

### 3. Set Up Your Environment

See [Development Setup](#development-setup) below.

---

## Development Setup

### Prerequisites

- Rust 1.85+ (MSRV)
- Wrangler CLI
- Node.js 18+

### Install Dependencies

```bash
# Install Rust toolchain for WASM
rustup target add wasm32-unknown-unknown

# Verify installation
cargo --version
wrangler --version
```

### Build the Project

```bash
# Build for WASM
cargo build --target wasm32-unknown-unknown

# Build with optimizations
cargo build --target wasm32-unknown-unknown --release
```

### Run Tests

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

---

## Making Changes

### Code Organization

- **One feature per branch** - Keep changes focused and reviewable
- **Commit often** - Use atomic commits with clear messages
- **Update tests** - Any new feature must include tests
- **Update documentation** - Keep docs in sync with code

### Example Workflow

```bash
# Start with a clean main branch
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/add-message-components

# Make changes
# ... edit files ...

# Run tests
cargo test

# Commit
git commit -m "feat: add support for message components"

# Push to your fork
git push origin feature/add-message-components
```

---

## Submitting Pull Requests

### Before Submitting

- [ ] Tests pass: `cargo test`
- [ ] Code builds: `cargo build --target wasm32-unknown-unknown`
- [ ] Documentation is updated (if relevant)
- [ ] Commit messages follow conventions (see below)
- [ ] No merge conflicts with `main`

### Pull Request Format

```markdown
## Description

Brief description of what this PR does.

## Related Issues

Closes #123

## Changes

- Change 1
- Change 2
- Change 3

## Testing

How you tested this change:
- [ ] Unit tests added/updated
- [ ] Manual testing completed
- [ ] Tested on Cloudflare Workers

## Screenshots (if applicable)

Include screenshots of Discord bot responses, etc.
```

### Commit Message Conventions

Follow conventional commits format:

```
<type>(<scope>): <description>

<body>

<footer>
```

#### Types

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `refactor` - Code refactoring
- `perf` - Performance improvements
- `test` - Testing
- `chore` - Maintenance

#### Examples

```bash
git commit -m "feat(bot): add ephemeral responses"
git commit -m "fix(error): improve signature verification errors"
git commit -m "docs(getting-started): clarify WASM setup"
git commit -m "refactor(models): simplify command option handling"
```

---

## Coding Standards

### Rust Style

We follow standard Rust conventions with `rustfmt`:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

Check for common issues with `clippy`:

```bash
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

### File Structure

```
src/
├── lib.rs              # Public exports
├── prelude.rs          # Common re-exports
├── module/
│   ├── mod.rs          # Module definition
│   ├── types.rs        # Type definitions
│   ├── impl.rs         # Implementations
│   └── tests.rs        # Tests (if applicable)
```

### Documentation

- Add doc comments to public items:

```rust
/// Represents a Discord command handler.
///
/// # Example
///
/// ```rust
/// # use flarecord::prelude::*;
/// # use async_trait::async_trait;
/// pub struct MyCommand;
///
/// #[async_trait(?Send)]
/// impl Command for MyCommand {
///     // ... implementation
/// }
/// ```
pub trait Command: Send + Sync {
    // ...
}
```

### Error Handling

- Use `thiserror` for error types
- Provide context with error messages
- Never use `.unwrap()` or `.panic!()`

### Testing

- Write tests next to the code
- Use `#[test]` attribute
- Test both success and error paths

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_registration() {
        let bot = BotBuilder::new()
            .register_command(Hello)
            .build();

        assert!(bot.has_command("hello"));
    }

    #[test]
    fn test_invalid_signature() {
        // Test error handling
    }
}
```

---

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_command_registration

# Run with verbose output
cargo test -- --nocapture --test-threads=1
```

### Integration Tests

Place integration tests in `tests/` directory:

```bash
tests/
├── integration_test.rs
└── fixtures/
    └── sample_interaction.json
```

### Testing Guidelines

- Test both happy path and error cases
- Use fixtures for test data
- Mock external services (Discord API calls)
- Keep tests fast and independent
- Use descriptive test names

---

## Review Process

### What to Expect

1. **Automated Checks** - CI pipeline runs tests and lint checks
2. **Code Review** - Maintainers review your changes
3. **Feedback** - You may be asked to make adjustments
4. **Approval** - Once approved, your PR is merged

### Response Time

- Initial review: Within 1-2 business days
- Follow-up feedback: Within 24-48 hours

### Getting Help

- **Questions?** Comment on your PR
- **Need guidance?** Open a discussion issue
- **Found a bug?** Open an issue with reproduction steps

---

## Reporting Bugs

### Bug Report Template

```markdown
## Description

Brief description of the bug.

## Reproduction Steps

1. Step 1
2. Step 2
3. Step 3

## Expected Behavior

What should happen.

## Actual Behavior

What actually happens.

## Environment

- Rust version: `rustc --version`
- Wrangler version: `wrangler --version`
- OS: Windows/macOS/Linux

## Logs

Include any error messages or logs (truncate if very long).

```

---

## Feature Requests

### Enhancement Template

```markdown
## Description

What feature should be added?

## Use Case

Why is this feature needed?

## Proposed Solution

How should this feature work?

## Alternatives

Are there alternative solutions?

## Additional Context

Any other relevant information.
```

---

## Documentation Contributions

### Improving Docs

Documentation lives in `/docs`. To contribute:

1. Fork and create a branch
2. Edit `.md` files
3. Test formatting locally
4. Submit a PR

### Documentation Standards

- Use clear, concise language
- Include code examples
- Link to related docs
- Keep formatting consistent

---

## Community

- **Discord Server**: [Coming soon]
- **Discussions**: Use GitHub Discussions
- **Issues**: Use GitHub Issues for bugs/features
- **Releases**: Subscribe to releases for updates

---

## Recognition

Contributors are recognized in:
- Git commit history
- Release notes (for significant contributions)
- Contributors list (coming soon)

---

## Development Tips

### Local Testing

```bash
# Test locally with Miniflare
cargo install miniflare
wrangler dev

# Then test your bot in Discord
```

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug wrangler dev

# In code:
eprintln!("Debug info: {:?}", variable);
```

### Performance Testing

```bash
# Build release binary
cargo build --target wasm32-unknown-unknown --release

# Check binary size
ls -lh target/wasm32-unknown-unknown/release/*.wasm
```

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Async Rust](https://rust-lang.github.io/async-book/)
- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)
- [Discord Developer Docs](https://discord.com/developers/docs)
- [Twilight Docs](https://twilight.rs/)

---

**Thank you for contributing to Flarecord!** 🎉

Questions? Open an issue or reach out to the maintainers.
