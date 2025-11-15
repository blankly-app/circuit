# Quick Start

Get up and running with Circuit in minutes!

## Prerequisites

Before you begin, ensure you have:

- **Rust** installed (1.70 or later)
- Basic familiarity with Rust syntax
- A text editor or IDE

## Installation

### 1. Install Rust

If you haven't already, install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Add Circuit to Your Project

Add Circuit to your `Cargo.toml`:

```toml
[dependencies]
circuit-core = { path = "../circuit-core" }
circuit-lang = { path = "../circuit-lang" }
```

Or clone the repository:

```bash
git clone https://github.com/blankly-app/circuit.git
cd circuit
```

## Your First Program

Let's create a simple calculator using Circuit's Rust API:

```rust
use circuit_core::{blocks::*, *};
use std::sync::Arc;

fn main() -> Result<()> {
    // Create the engine
    let mut engine = Engine::new();

    // Register built-in blocks
    engine.register_block(Arc::new(AddBlock))?;
    engine.register_block(Arc::new(MultiplyBlock))?;
    engine.register_block(Arc::new(ConstantBlock))?;

    // Create a graph
    let mut graph = Graph::new(
        "calculator".to_string(),
        "Simple Calculator".to_string()
    );

    // Add nodes
    let const5 = graph.add_node(Node::new(
        "const5".to_string(),
        "core.constant".to_string(),
        vec![("value".to_string(), Value::Int(5))].into_iter().collect(),
    ));

    let const3 = graph.add_node(Node::new(
        "const3".to_string(),
        "core.constant".to_string(),
        vec![("value".to_string(), Value::Int(3))].into_iter().collect(),
    ));

    let add = graph.add_node(Node::new(
        "add".to_string(),
        "math.add".to_string(),
        HashMap::new(),
    ));

    // Connect nodes
    graph.connect(&const5, "value", &add, "a")?;
    graph.connect(&const3, "value", &add, "b")?;

    // Load and execute
    engine.load_graph(graph)?;
    let results = engine.execute_graph("calculator")?;

    println!("Result: {:?}", results);

    Ok(())
}
```

Run it:

```bash
cargo run
```

## Using the Declarative Language

Circuit's declarative language makes it even easier. Create a file `calculator.flow`:

```flow
flow calculator {
    description "Simple calculator: (5 + 3) * 2 = 16"

    node const5: core.constant { value = 5 }
    node const3: core.constant { value = 3 }
    node const2: core.constant { value = 2 }

    node add: math.add
    node multiply: math.multiply

    connect const5.value -> add.a
    connect const3.value -> add.b
    connect add.result -> multiply.a
    connect const2.value -> multiply.b
}
```

Load and execute it:

```rust
use circuit_core::*;
use circuit_lang::{parse_flow, flow_to_graph};
use std::fs;

fn main() -> Result<()> {
    let mut engine = Engine::new();

    // Register blocks (as before)
    // ...

    // Load flow file
    let source = fs::read_to_string("calculator.flow")?;
    let flow = parse_flow(&source)?;
    let graph = flow_to_graph(&flow)?;

    engine.load_graph(graph)?;
    let results = engine.execute_graph("calculator")?;

    println!("Calculator result: {:?}", results);
    Ok(())
}
```

## Running the Examples

Circuit comes with several example flows in the `examples/` directory:

```bash
# Run the calculator example
cargo run --example calculator

# Run tests to see all examples in action
cargo test -p circuit-lang --test integration_tests
```

## Next Steps

Now that you've seen the basics, explore:

- [Your First Flow](./first-flow.md) - A detailed walkthrough of creating flows
- [Architecture Overview](./architecture.md) - Understand how Circuit works
- [Understanding Blocks](../guide/blocks.md) - Deep dive into blocks
- [The Declarative Language](../guide/language.md) - Complete language reference

## Common Issues

### Missing Blocks

If you see "Block not found" errors, make sure you've registered all the blocks you're using:

```rust
engine.register_block(Arc::new(AddBlock))?;
engine.register_block(Arc::new(MultiplyBlock))?;
// etc.
```

### Connection Errors

Port names must match exactly. Check your block definitions for the correct input/output port names.

### Parse Errors

When using `.flow` files, ensure:
- Block types are registered before loading
- Syntax matches the language specification
- All connections reference valid ports
