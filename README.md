# Circuit

**A node-based runtime engine for building apps with blocks written in Rust that runs anywhere**

Circuit is a flexible, cross-platform runtime engine that allows you to create applications using a visual node-based architecture. Write your blocks in Rust once, and run them on Swift (iOS/macOS), Kotlin (Android), and Web/React platforms.

## 🌟 Features

- **Universal Blocks**: Write computational blocks in Rust once, use everywhere
- **Cross-Platform**: Run on iOS, macOS, Android, and Web (via WebAssembly)
- **Type-Safe**: Strong typing with Rust's safety guarantees
- **Visual Flow**: Node-based execution graphs for clear data flow
- **Extensible**: Easy to add custom blocks for your use cases
- **Zero-Copy FFI**: Efficient cross-language communication
- **Cycle Detection**: Automatic validation of graph structures
- **Topological Execution**: Optimized execution order

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

Circuit consists of three main packages:

### 1. **circuit-core**
The core runtime engine written in Rust.

- **Blocks**: Reusable computational units
- **Graphs**: Visual flow definitions  
- **Engine**: Execution runtime with topological sorting
- **Values**: Type-safe data flow between blocks

### 2. **circuit-wasm**
WebAssembly bindings for running in browsers and JavaScript environments.

- Zero-copy data transfer
- JavaScript-friendly API
- React/Vue/Angular compatible

### 3. **circuit-ffi**
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

Blocks are the building units of Circuit applications. Here's how to create one:

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

## 🎯 Example: Calculator

See the [calculator example](examples/calculator.rs) for a complete working example:

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

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test -p circuit-core
cargo test -p circuit-ffi

# Run with output
cargo test -- --nocapture
```

## 📚 Documentation

- [Swift/iOS Integration](docs/SWIFT.md)
- [Kotlin/Android Integration](docs/KOTLIN.md)
- [React/Web Integration](docs/REACT.md)

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

- [ ] More built-in blocks (subtract, divide, modulo, etc.)
- [ ] Visual graph editor (web-based)
- [ ] Graph serialization to/from JSON
- [ ] Hot-reload support
- [ ] Performance optimizations
- [ ] Async block execution
- [ ] Streaming data support
- [ ] Graph versioning

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Circuit is designed to be a universal runtime for node-based applications, inspired by:
- Visual programming paradigms
- Dataflow programming
- Cross-platform development needs

