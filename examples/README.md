# Circuit Examples

This directory contains example applications demonstrating how to use Circuit.

## Available Examples

### Calculator (`calculator.rs`)

A simple calculator that demonstrates:
- Creating and configuring an engine
- Registering custom blocks
- Building a graph programmatically
- Connecting nodes together
- Executing a graph and reading results

**Run it:**
```bash
cargo run --example calculator
```

**What it does:**
Calculates `(5 + 3) * 2 = 16` using a node-based graph with:
- 3 Constant blocks (values: 5, 3, 2)
- 1 Add block
- 1 Multiply block
- 1 Debug block (prints the result)

## Creating Your Own Examples

1. Create a new file in the `examples/` directory (e.g., `my_example.rs`)

2. Add the example to `circuit-core/Cargo.toml`:
```toml
[[example]]
name = "my_example"
path = "../examples/my_example.rs"
```

3. Write your example using the Circuit API

4. Run it:
```bash
cargo run --example my_example
```

## Example Template

```rust
use circuit_core::{blocks::*, *};
use std::collections::HashMap;
use std::sync::Arc;

fn main() {
    // 1. Create engine
    let mut engine = Engine::new();

    // 2. Register blocks
    engine.register_block(Arc::new(AddBlock)).unwrap();
    engine.register_block(Arc::new(ConstantBlock)).unwrap();

    // 3. Create graph
    let mut graph = Graph::new("my-graph".to_string(), "My Graph".to_string());

    // 4. Add nodes
    let mut config = HashMap::new();
    config.insert("value".to_string(), Value::Float(42.0));
    
    let node = graph::Node {
        id: "const1".to_string(),
        block_type: "core.constant".to_string(),
        config,
        position: None,
    };
    graph.add_node(node).unwrap();

    // 5. Add connections (if multiple nodes)
    // graph.add_connection(graph::Connection { ... }).unwrap();

    // 6. Load and execute
    engine.load_graph(graph).unwrap();
    let results = engine.execute_graph("my-graph").unwrap();

    // 7. Process results
    println!("Results: {:?}", results);
}
```

## More Complex Examples

### String Processing

```rust
use circuit_core::{blocks::*, *};

fn string_example() {
    let mut engine = Engine::new();
    engine.register_block(Arc::new(ConstantBlock)).unwrap();
    engine.register_block(Arc::new(ConcatBlock)).unwrap();
    
    // Build graph that concatenates "Hello" + " " + "World"
    // ...
}
```

### Data Pipeline

```rust
use circuit_core::*;

// Create a multi-step data processing pipeline
fn pipeline_example() {
    // 1. Load data (Constant)
    // 2. Transform (custom blocks)
    // 3. Filter (custom blocks)
    // 4. Aggregate (custom blocks)
    // 5. Output (Debug)
}
```

## Platform-Specific Examples

### iOS/Swift

See [docs/SWIFT.md](../docs/SWIFT.md) for iOS/macOS integration examples.

### Android/Kotlin

See [docs/KOTLIN.md](../docs/KOTLIN.md) for Android integration examples.

### Web/React

See [docs/REACT.md](../docs/REACT.md) for web integration examples.

## Tips

1. **Start Simple**: Begin with the calculator example and modify it
2. **Debug Output**: Use the `DebugBlock` to inspect values
3. **Error Handling**: Always handle `Result` types properly
4. **Graph Validation**: The engine validates graphs automatically (cycle detection, etc.)
5. **Serialization**: You can save/load graphs as JSON for persistence

## Need Help?

- Check the [API documentation](../docs/API.md)
- Read the [main README](../README.md)
- Open an issue on GitHub
