# Circuit

**A node-based runtime engine for building apps with blocks written in Rust that runs anywhere**

Circuit is a flexible, cross-platform runtime engine that allows you to create applications using a visual node-based architecture. Write your blocks in Rust once, and run them on Swift (iOS/macOS), Kotlin (Android), and Web/React platforms.

## 🌟 Features

- **Universal Blocks**: Write computational blocks in Rust once, use everywhere
- **Declarative Language**: Define blocks and flows using `.block` and `.flow` files
- **Cross-Platform**: Run on iOS, macOS, Android, and Web (via WebAssembly)
- **Type-Safe**: Strong typing with Rust's safety guarantees
- **Visual Flow**: Node-based execution graphs for clear data flow
- **Extensible**: Easy to add custom blocks for your use cases
- **Zero-Copy FFI**: Efficient cross-language communication
- **Cycle Detection**: Automatic validation of graph structures
- **Topological Execution**: Optimized execution order
- **Comprehensive Testing**: Unit, integration, and WASM tests included

## 🚀 Quick Start

### Core Library (Rust)

```rust
use circuit_core::{blocks::*, *};
use std::sync::Arc;

// Create engine
let mut engine = Engine::new();

// Register blocks
engine.register_block(Arc::new(AddBlock)).unwrap();
engine.register_block(Arc::new(ConstantBlock)).unwrap();

// Create and execute graph
let mut graph = Graph::new("calc".to_string(), "Calculator".to_string());
// ... add nodes and connections ...
engine.load_graph(graph).unwrap();
let results = engine.execute_graph("calc").unwrap();
```

### Swift (iOS/macOS)

```swift
let engine = CircuitEngine()
try engine.loadGraph(json: graphJson)
let result = try engine.executeGraph(id: "calculator")
```

See [Swift Documentation](docs/SWIFT.md) for detailed integration.

### Kotlin (Android)

```kotlin
val engine = CircuitEngine()
engine.loadGraph(graphJson)
val result = engine.executeGraph("calculator")
```

See [Kotlin Documentation](docs/KOTLIN.md) for detailed integration.

### React (Web)

```typescript
const { loadGraph, executeGraph } = useCircuit();
await loadGraph(graphJson);
const results = await executeGraph('calculator');
```

See [React Documentation](docs/REACT.md) for detailed integration.

## 📦 Architecture

Circuit consists of four main packages:

### 1. **circuit-core**
The core runtime engine written in Rust.

- **Blocks**: Reusable computational units
- **Graphs**: Visual flow definitions
- **Engine**: Execution runtime with topological sorting
- **Values**: Type-safe data flow between blocks

### 2. **circuit-lang**
Language parser for `.block` and `.flow` files.

- **Declarative Syntax**: Define blocks and flows in a simple, readable format
- **Type System**: Support for Number, String, Bool, Array, Object, and more
- **Converter**: Convert flow definitions to executable graphs
- **Examples**: Pre-built example blocks and flows

### 3. **circuit-wasm**
WebAssembly bindings for running in browsers and JavaScript environments.

- Zero-copy data transfer
- JavaScript-friendly API
- React/Vue/Angular compatible

### 4. **circuit-ffi**
C-compatible FFI layer for native platform integration.

- Swift bindings (iOS/macOS)
- Kotlin bindings (Android)
- Static and dynamic library support

## 🏗️ Building

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install platform targets (optional, for cross-compilation)
rustup target add aarch64-apple-ios      # iOS ARM64
rustup target add x86_64-apple-ios       # iOS Simulator
rustup target add aarch64-linux-android  # Android ARM64
rustup target add wasm32-unknown-unknown # WebAssembly
```

### Build All Packages

```bash
# Build all packages
cargo build --release

# Run tests
cargo test

# Run example
cargo run --example calculator
```

### Build for Specific Platforms

```bash
# iOS
cargo build --release --target aarch64-apple-ios -p circuit-ffi

# Android
cargo build --release --target aarch64-linux-android -p circuit-ffi

# WebAssembly
cd circuit-wasm
wasm-pack build --target web
```

## 📖 Creating Custom Blocks

### Using .block Files (Recommended)

The easiest way to create blocks is using `.block` files:

```
block math.square {
    description "Squares a number (x²)"

    input x: Number {
        description "The number to square"
    }

    output result: Number {
        description "The squared result"
    }

    execute {
        result = x * x
    }
}
```

Load and use it:

```rust
use circuit_lang::{parse_block, flow_to_graph};

let source = std::fs::read_to_string("math.square.block")?;
let block_def = parse_block(&source)?;
```

See the [Language Documentation](docs/LANGUAGE.md) for complete syntax reference.

### Using Rust (Advanced)

You can also create blocks programmatically in Rust:

```rust
use circuit_core::*;
use std::collections::HashMap;

pub struct MyBlock;

impl Block for MyBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "my.block".to_string(),
            name: "My Block".to_string(),
            description: "Does something useful".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "input".to_string(),
                    name: "Input".to_string(),
                    data_type: "any".to_string(),
                    required: true,
                }
            ],
            outputs: vec![
                PortDefinition {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    data_type: "any".to_string(),
                    required: true,
                }
            ],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let input = context.get_input("input")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input".to_string()))?;
        
        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), input.clone());
        Ok(outputs)
    }
}
```

## 🎯 Examples

### Example 1: Calculator Flow

The calculator flow demonstrates basic arithmetic operations:

```flow
flow calculator {
    description "Simple calculator: (5 + 3) * 2 = 16"

    node const5: core.constant {
        value = 5
    }

    node const3: core.constant {
        value = 3
    }

    node add: math.add
    node multiply: math.multiply

    connect const5.value -> add.a
    connect const3.value -> add.b
    connect add.result -> multiply.a
}
```

See [examples/flows/calculator.flow](examples/flows/calculator.flow) and other examples in the `examples/` directory.

### Example 2: Programmatic (Rust)

See the [calculator example](examples/calculator.rs) for a complete Rust example:

```bash
cargo run --example calculator
```

This demonstrates:
- Creating an engine
- Registering blocks
- Building a graph programmatically
- Connecting nodes
- Executing the graph
- Reading results

## 🧪 Testing

Circuit includes comprehensive testing across all packages:

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test -p circuit-core      # Core engine tests
cargo test -p circuit-lang      # Language parser tests
cargo test -p circuit-ffi       # FFI layer tests
cargo test -p circuit-wasm      # WASM tests

# Run integration tests (tests example files)
cargo test -p circuit-lang --test integration_tests

# Run with output
cargo test -- --nocapture
```

### Test Coverage

- **circuit-core**: 15 unit tests covering blocks, graphs, engine, and values
- **circuit-lang**: 20+ tests for parser and converter
- **circuit-lang integration**: Tests for all example .block and .flow files
- **circuit-ffi**: Engine lifecycle tests
- **circuit-wasm**: 10+ WASM-specific tests

All tests are run automatically via GitHub Actions CI/CD on every commit.

## 📚 Documentation

**[📖 Read the full documentation →](https://blankly-app.github.io/circuit/)**

The comprehensive documentation website includes:

- **Getting Started** - Quick start, installation, your first flow
- **User Guide** - Blocks, flows, declarative language, custom blocks
- **Platform Integration** - Swift, Kotlin, React, WebAssembly guides
- **Advanced Topics** - Building, FFI, performance, testing
- **API Reference** - Complete Rust API documentation
- **Examples** - Practical tutorials and walkthroughs

### Quick Links

- [Language Specification](docs/LANGUAGE.md) - `.block` and `.flow` syntax
- [API Reference](docs/API.md) - Core API documentation
- [Swift/iOS Integration](docs/SWIFT.md)
- [Kotlin/Android Integration](docs/KOTLIN.md)
- [React/Web Integration](docs/REACT.md)

### Building Documentation Locally

```bash
cd docs
./build.sh
mdbook serve --open
```

## 🔧 Built-in Blocks

Circuit includes several built-in blocks:

### Math
- `math.add` - Add two numbers
- `math.multiply` - Multiply two numbers

### Core
- `core.constant` - Output a constant value
- `core.debug` - Print debug information

### String
- `string.concat` - Concatenate strings

## 🛣️ Roadmap

- [x] Declarative language for blocks and flows
- [x] Comprehensive testing infrastructure
- [x] Parser for `.block` and `.flow` files
- [x] Documentation website with guides and API reference
- [ ] More built-in blocks (subtract, divide, modulo, etc.)
- [ ] Visual graph editor (web-based)
- [ ] Block execution interpreter (for `.block` execute blocks)
- [ ] Hot-reload support
- [ ] Performance optimizations and benchmarks
- [ ] Async block execution
- [ ] Streaming data support
- [ ] Graph versioning
- [ ] Standard library of common blocks

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Circuit is designed to be a universal runtime for node-based applications, inspired by:
- Visual programming paradigms
- Dataflow programming
- Cross-platform development needs

