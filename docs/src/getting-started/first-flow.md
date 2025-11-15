# Your First Flow

This tutorial will walk you through creating your first Circuit flow from scratch.

## What You'll Build

We'll create a temperature converter that converts Celsius to Fahrenheit using the formula:
```
F = (C × 9/5) + 32
```

## Step 1: Create the Flow File

Create a new file called `temp_converter.flow`:

```flow
flow temp_converter {
    description "Converts Celsius to Fahrenheit"

    // Input temperature in Celsius
    node celsius: core.constant {
        value = 25
    }

    // Constants for the formula
    node nine: core.constant {
        value = 9
    }

    node five: core.constant {
        value = 5
    }

    node thirtytwo: core.constant {
        value = 32
    }

    // Calculations: (C × 9 / 5) + 32
    node multiply: math.multiply
    node divide: math.multiply  // We'll use multiply with 1/5 = 0.2
    node add: math.add

    // Connect the graph
    connect celsius.value -> multiply.a
    connect nine.value -> multiply.b
    connect multiply.result -> divide.a
    connect five.value -> divide.b
    connect divide.result -> add.a
    connect thirtytwo.value -> add.b

    // Output the final result
    output add.result
}
```

## Step 2: Understanding the Flow

Let's break down what this flow does:

1. **Constants**: We define constant values for:
   - `celsius` (25) - the input temperature
   - `nine` (9) - part of the conversion formula
   - `five` (5) - part of the conversion formula
   - `thirtytwo` (32) - the offset in the formula

2. **Operations**: We create three math operation nodes:
   - `multiply` - multiplies Celsius by 9
   - `divide` - divides by 5
   - `add` - adds 32

3. **Connections**: We wire the nodes together to form the calculation pipeline

4. **Output**: We expose the final result from the `add` node

## Step 3: Load and Execute

Create a Rust program to execute the flow:

```rust
use circuit_core::*;
use circuit_core::blocks::*;
use circuit_lang::{parse_flow, flow_to_graph};
use std::sync::Arc;
use std::fs;

fn main() -> Result<()> {
    // Create engine and register blocks
    let mut engine = Engine::new();
    engine.register_block(Arc::new(AddBlock))?;
    engine.register_block(Arc::new(MultiplyBlock))?;
    engine.register_block(Arc::new(ConstantBlock))?;

    // Load the flow file
    let source = fs::read_to_string("temp_converter.flow")?;
    let flow = parse_flow(&source)?;
    let graph = flow_to_graph(&flow)?;

    // Execute
    let graph_id = graph.id.clone();
    engine.load_graph(graph)?;
    let results = engine.execute_graph(&graph_id)?;

    // Print results
    for (node_id, outputs) in results {
        println!("Node {}: {:?}", node_id, outputs);
    }

    Ok(())
}
```

## Step 4: Run It

```bash
cargo run
```

You should see output showing the intermediate calculations and the final result (25°C = 77°F).

## Step 5: Make It Interactive

Now let's make it easier to change the input. Modify your Rust program:

```rust
use std::io::{self, Write};

fn main() -> Result<()> {
    // ... (setup code as before)

    // Get temperature from user
    print!("Enter temperature in Celsius: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let celsius: f64 = input.trim().parse()
        .map_err(|_| CircuitError::InvalidInput("Invalid number".to_string()))?;

    // Modify the constant node's configuration
    // (In a real app, you'd modify the graph before loading)

    // ... (execution code)

    Ok(())
}
```

## Next Steps

Congratulations! You've created your first Circuit flow. Here's what to explore next:

### Create Custom Blocks

Instead of using just constants and math operations, create custom blocks:

```block
block conversion.celsius_to_fahrenheit {
    description "Converts Celsius to Fahrenheit"

    input celsius: Number {
        description "Temperature in Celsius"
    }

    output fahrenheit: Number {
        description "Temperature in Fahrenheit"
    }

    execute {
        fahrenheit = (celsius * 9 / 5) + 32
    }
}
```

### Add More Features

Extend your temperature converter:

1. Support both directions (C→F and F→C)
2. Add Kelvin conversion
3. Add input validation
4. Create a visual display

### Learn More About Flows

- [Creating Flows](../guide/flows.md) - Detailed flow guide
- [The Declarative Language](../guide/language.md) - Complete syntax reference
- [Built-in Blocks](../guide/builtin-blocks.md) - Available blocks

### Try the Examples

Check out the example flows in `examples/flows/`:

```bash
cargo test -p circuit-lang --test integration_tests
```

## Troubleshooting

### "Block not found" error

Make sure all blocks used in your flow are registered:

```rust
engine.register_block(Arc::new(AddBlock))?;
engine.register_block(Arc::new(MultiplyBlock))?;
engine.register_block(Arc::new(ConstantBlock))?;
```

### Connection errors

Verify port names match exactly. Use:
- `.value` for constant block outputs
- `.a` and `.b` for math block inputs
- `.result` for math block outputs

### Parse errors

Check your `.flow` syntax:
- Node definitions need `node id: type { config }`
- Connections need `connect from.port -> to.port`
- All statements should be inside the `flow { }` block
