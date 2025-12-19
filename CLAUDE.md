# CLAUDE.md - AI Assistant Guide for Circuit

This document provides context and guidelines for AI assistants working with the Circuit codebase.

## Project Overview

**Circuit** is a node-based runtime engine for building applications with visual dataflow graphs. Written in Rust, it compiles to multiple platforms:
- **Native** (iOS/macOS via Swift, Android via Kotlin)
- **Web** (via WebAssembly)

The core idea: define computational "blocks" in Rust once, connect them in "flows", and run them anywhere.

## Repository Structure

```
circuit/
├── circuit-core/        # Core runtime engine (main Rust library)
│   └── src/
│       ├── lib.rs       # Public API exports
│       ├── block.rs     # Block trait and BlockContext
│       ├── blocks/      # Built-in block implementations
│       │   ├── core.rs  # ConstantBlock, DebugBlock
│       │   ├── math.rs  # AddBlock, MultiplyBlock
│       │   ├── logic.rs # Logic blocks
│       │   └── string.rs# StringConcatBlock
│       ├── engine.rs    # Engine - execution runtime
│       ├── graph.rs     # Graph, Node, Connection types
│       ├── value.rs     # Value enum (Int, Float, String, etc.)
│       └── error.rs     # CircuitError, Result types
│
├── circuit-lang/        # Parser for .block and .flow files
│   └── src/
│       ├── lib.rs       # Public API (parse_block, parse_flow)
│       ├── parser.rs    # Parser implementation
│       ├── ast.rs       # AST types (BlockDef, FlowDef, etc.)
│       ├── converter.rs # flow_to_graph conversion
│       └── grammar.pest # PEG grammar definition
│
├── circuit-wasm/        # WebAssembly bindings
│   └── src/lib.rs       # WasmEngine wrapper
│
├── circuit-ffi/         # C-compatible FFI layer
│   └── src/lib.rs       # C ABI functions for Swift/Kotlin
│
├── examples/            # Example files
│   ├── calculator.rs    # Rust example
│   ├── blocks/          # Example .block files
│   └── flows/           # Example .flow files
│
└── docs/                # Documentation (mdbook)
    ├── LANGUAGE.md      # .block/.flow syntax reference
    ├── API.md           # Core API documentation
    ├── SWIFT.md         # iOS/macOS integration
    ├── KOTLIN.md        # Android integration
    └── REACT.md         # Web integration
```

## Key Concepts

### Block
A computational unit that transforms inputs to outputs. Implements the `Block` trait:

```rust
pub trait Block: Send + Sync {
    fn metadata(&self) -> BlockMetadata;
    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>>;
    fn validate(&self, _config: &HashMap<String, Value>) -> Result<()> { Ok(()) }
}
```

### Graph
A directed acyclic graph (DAG) of connected nodes. Each node is an instance of a block type.

### Engine
The runtime that:
1. Registers block types
2. Loads graphs
3. Executes graphs in topological order
4. Collects results

### Value
Type-safe data flow between blocks:
```rust
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Bytes(Vec<u8>),
}
```

## Development Commands

### Build
```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo build -p circuit-core    # Build specific package
```

### Test
```bash
cargo test                     # Run all tests
cargo test -p circuit-core     # Test specific package
cargo test -p circuit-lang --test integration_tests  # Integration tests
cargo test -- --nocapture      # Show println! output
```

### Format & Lint
```bash
cargo fmt                      # Format code
cargo fmt --all -- --check     # Check formatting (CI)
cargo clippy --all-targets --all-features -- -D warnings  # Lint
```

### Run Examples
```bash
cargo run --example calculator
```

### Build for Platforms
```bash
# WebAssembly
cd circuit-wasm && wasm-pack build --target web

# iOS
cargo build --release --target aarch64-apple-ios -p circuit-ffi

# Android
cargo build --release --target aarch64-linux-android -p circuit-ffi
```

### Documentation
```bash
cargo doc --no-deps --all-features    # Generate Rustdoc
cd docs && mdbook serve --open         # Serve mdbook docs
```

## Rust Toolchain

The project uses Rust 1.90.0 (specified in `.rust-toolchain.toml`):
- Components: `rustfmt`, `clippy`, `rust-src`
- Targets: `wasm32-unknown-unknown`

Using `mise` is recommended for toolchain management:
```bash
mise use rust@1.90.0
mise install
```

## Code Conventions

### Naming
- Block IDs use qualified names: `math.add`, `core.constant`, `string.concat`
- Namespace blocks by category: `math.`, `core.`, `string.`, `logic.`, `io.`

### Error Handling
- Use `CircuitError` enum for domain errors
- Use `Result<T>` type alias (`std::result::Result<T, CircuitError>`)
- Use `thiserror` for error derivation

### Testing
- Unit tests go in the same file as the code (`#[cfg(test)] mod tests`)
- Integration tests go in `tests/` directory
- Each block should have execution tests
- Test files: parser_tests.rs, integration_tests.rs, wasm_tests.rs

### Block Implementation Pattern
```rust
pub struct MyBlock;

impl Block for MyBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "namespace.my_block".to_string(),
            name: "My Block".to_string(),
            description: "What this block does".to_string(),
            inputs: vec![PortDefinition { /* ... */ }],
            outputs: vec![PortDefinition { /* ... */ }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let input = context.get_input("input_name")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input".to_string()))?;

        let mut outputs = HashMap::new();
        outputs.insert("output_name".to_string(), /* computed value */);
        Ok(outputs)
    }
}
```

## Workspace Dependencies

Shared dependencies are defined in root `Cargo.toml`:
- `serde` + `serde_json`: Serialization
- `thiserror`: Error derivation
- `anyhow`: Error context

Additional per-crate:
- `circuit-lang`: `pest` (PEG parser generator)
- `circuit-wasm`: `wasm-bindgen`, `wasm-bindgen-test`
- `circuit-ffi`: `lazy_static`

## CI/CD Pipeline

GitHub Actions workflows (`.github/workflows/`):

### ci.yml
- **test**: Format check, clippy, build, test, run example
- **build-wasm**: Build WebAssembly target
- **check-docs**: Verify documentation builds

### claude.yml
- Integrates Claude Code for AI-assisted development
- Triggers on @claude mentions in issues/PRs

### deploy-docs.yml
- Deploys documentation to GitHub Pages

## Language Files

### .block Files
Define reusable computational blocks:
```
block math.square {
    description "Squares a number"

    input x: Number {
        description "Input value"
    }

    output result: Number {
        description "Squared result"
    }

    execute {
        result = x * x
    }
}
```

### .flow Files
Define complete graphs:
```
flow calculator {
    description "Adds two numbers"

    node a: core.constant { value = 5 }
    node b: core.constant { value = 3 }
    node sum: math.add

    connect a.value -> sum.a
    connect b.value -> sum.b

    output sum.result
}
```

See `docs/LANGUAGE.md` for complete syntax reference.

## Common Tasks

### Adding a New Block (Rust)
1. Add implementation in `circuit-core/src/blocks/<category>.rs`
2. Export in `circuit-core/src/blocks/mod.rs`
3. Add tests in the same file
4. Run `cargo test -p circuit-core`

### Adding a New Block (.block file)
1. Create file in `examples/blocks/` or your project
2. Parse with `circuit_lang::parse_block()`
3. The execute block defines computation logic

### Creating a Flow
1. Create `.flow` file or build programmatically
2. Parse with `circuit_lang::parse_flow()`
3. Convert with `circuit_lang::flow_to_graph()`
4. Load with `engine.load_graph(graph)`
5. Execute with `engine.execute_graph(id)`

### Cross-Platform Integration
- **Swift**: Use `circuit-ffi` C functions via Swift wrapper
- **Kotlin**: Use `circuit-ffi` via JNI
- **React/Web**: Use `circuit-wasm` via wasm-bindgen

## Architecture Notes

### Execution Model
1. Engine validates all block types are registered
2. Graph undergoes cycle detection via topological sort
3. Nodes execute in dependency order
4. Outputs flow along connections to downstream inputs
5. Final results are collected per-node

### Thread Safety
- `Block` trait requires `Send + Sync`
- Engine uses `Arc<dyn Block>` for block storage
- FFI/WASM use `Arc<Mutex<Engine>>` for safe sharing

### Graph Validation
- Cycle detection prevents infinite loops
- Unknown block types are rejected at load time
- Connections reference existing nodes and ports

## Quick Reference

| Task | Command |
|------|---------|
| Build all | `cargo build` |
| Test all | `cargo test` |
| Format | `cargo fmt` |
| Lint | `cargo clippy` |
| Run example | `cargo run --example calculator` |
| Build WASM | `cd circuit-wasm && wasm-pack build --target web` |
| Build docs | `cargo doc --no-deps` |

## Files of Interest

- `circuit-core/src/block.rs` - Block trait definition
- `circuit-core/src/engine.rs` - Execution engine
- `circuit-core/src/graph.rs` - Graph structure
- `circuit-lang/src/grammar.pest` - Language grammar
- `examples/calculator.rs` - Complete working example
- `docs/LANGUAGE.md` - Language specification
