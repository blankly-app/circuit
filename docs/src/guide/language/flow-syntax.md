# Flow Syntax

Flow files define complete computational graphs by instantiating blocks and connecting them together.

## Basic Structure

```
flow <name> {
    description "Human-readable description"

    node <id>: <block.type> {
        <config_param> = <value>
        position(<x>, <y>)
    }

    connect <from_node>.<from_port> -> <to_node>.<to_port>

    output <node>.<port>
}
```

## Flow Name

Simple identifier (not qualified like blocks):

```
flow calculator { ... }
flow data_pipeline { ... }
flow my_workflow { ... }
```

## Description

Optional but recommended:

```
flow calculator {
    description "Simple calculator: (5 + 3) * 2 = 16"
    ...
}
```

## Node Definitions

Each node is an instance of a block:

```
node <id>: <block.type> {
    <config_param> = <value>
    position(<x>, <y>)
}
```

**Example:**
```
node const5: core.constant {
    value = 5
    position(100, 100)
}

node add: math.add {
    position(250, 150)
}
```

**Properties:**
- `id`: Unique identifier for this node instance
- `block.type`: The qualified name of the block to instantiate
- Configuration values are set inline
- `position(x, y)` is optional (for visual layout)

## Connections

Define data flow between nodes:

```
connect <from_node>.<from_port> -> <to_node>.<to_port>
```

**Examples:**
```
connect const5.value -> add.a
connect add.result -> multiply.a
connect input.data -> processor.input
```

**Rules:**
- Port names must match the block's input/output definitions
- Connections create directed edges in the graph
- A single output can connect to multiple inputs
- An input can only receive one connection

## Output Declarations

Specify which node outputs are exposed as flow outputs:

```
output <node>.<port>
```

**Example:**
```
output multiply.result
output final_node.computed_value
```

## Complete Examples

### Example 1: Simple Calculator

```
flow calculator {
    description "Simple calculator: (5 + 3) * 2 = 16"

    node const5: core.constant {
        value = 5
        position(100, 100)
    }

    node const3: core.constant {
        value = 3
        position(100, 200)
    }

    node const2: core.constant {
        value = 2
        position(400, 150)
    }

    node add: math.add {
        position(250, 150)
    }

    node multiply: math.multiply {
        position(550, 150)
    }

    connect const5.value -> add.a
    connect const3.value -> add.b
    connect add.result -> multiply.a
    connect const2.value -> multiply.b

    output multiply.result
}
```

### Example 2: Data Pipeline

```
flow data_pipeline {
    description "Process data through multiple stages"

    node input: core.constant {
        value = 10
    }

    node stage1: math.multiply {
        position(200, 100)
    }

    node stage2: math.add {
        position(400, 100)
    }

    node stage3: core.debug {
        position(600, 100)
    }

    connect input.value -> stage1.a
    connect stage1.result -> stage2.a
    connect stage2.result -> stage3.value

    output stage3.value
}
```

### Example 3: String Processing

```
flow string_processing {
    description "Concatenates strings together"

    node hello: core.constant {
        value = "Hello"
    }

    node space: core.constant {
        value = " "
    }

    node world: core.constant {
        value = "World"
    }

    node concat1: string.concat
    node concat2: string.concat

    connect hello.value -> concat1.a
    connect space.value -> concat1.b
    connect concat1.result -> concat2.a
    connect world.value -> concat2.b

    output concat2.result
}
```

## Loading and Executing Flows

Use the `circuit-lang` crate:

```rust
use circuit_core::*;
use circuit_lang::{parse_flow, flow_to_graph};
use std::fs;

// Parse flow file
let source = fs::read_to_string("calculator.flow")?;
let flow = parse_flow(&source)?;

// Convert to executable graph
let graph = flow_to_graph(&flow)?;

// Execute with engine
let graph_id = graph.id.clone();
engine.load_graph(graph)?;
let results = engine.execute_graph(&graph_id)?;
```

## Graph Validation

Circuit automatically validates flows:

- **Cycle Detection**: Ensures no circular dependencies
- **Port Validation**: Checks that connected ports exist
- **Type Checking**: Validates data type compatibility (future)

## Position Coordinates

The `position(x, y)` directive is optional and used for visual graph editors:

```
node add: math.add {
    position(250, 150)
}
```

- Coordinates are arbitrary (typically pixels)
- Not required for execution
- Helpful for visual tools and documentation

## Common Patterns

### Fan-out (One output → Many inputs)

```
node source: core.constant { value = 5 }
node consumer1: math.add
node consumer2: math.multiply

connect source.value -> consumer1.a
connect source.value -> consumer2.a
```

### Sequential Processing

```
node step1: processor.first
node step2: processor.second
node step3: processor.third

connect step1.output -> step2.input
connect step2.output -> step3.input
```

### Parallel Processing

```
node input: core.constant { value = 10 }
node pathA: math.add
node pathB: math.multiply

connect input.value -> pathA.a
connect input.value -> pathB.a
```

## See Also

- [Block Syntax](./block-syntax.md) - Define blocks to use in flows
- [Examples](../../examples/calculator.md) - Real-world flow examples
- [Creating Flows](../flows.md) - Practical guide
