# Contributing to Circuit

We love your input! We want to make contributing to Circuit as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## Development Process

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code follows the existing style.
6. Issue that pull request!

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/circuit.git
cd circuit

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and test
cargo build
cargo test

# Run example
cargo run --example calculator
```

## Adding New Blocks

To add a new block type:

1. Create your block implementation in `circuit-core/src/blocks.rs`
2. Implement the `Block` trait
3. Add tests for your block
4. Update documentation

Example:

```rust
pub struct MyBlock;

impl Block for MyBlock {
    fn metadata(&self) -> BlockMetadata {
        // Define metadata
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        // Implement logic
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_my_block() {
        // Add tests
    }
}
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Follow Rust naming conventions
- Add documentation comments for public APIs
- Keep functions focused and small

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests for specific package
cargo test -p circuit-core
```

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update documentation in the `docs/` directory
3. The PR will be merged once you have the sign-off of a maintainer

## Bug Reports

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening)

## Feature Requests

Feature requests are welcome! Please provide:

- Clear description of the feature
- Use case / motivation
- Example of how it would work
- Any implementation ideas (optional)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to open an issue for any questions or concerns!
