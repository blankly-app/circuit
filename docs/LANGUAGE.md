# Circuit Language Specification

Circuit provides a declarative language for defining computational blocks and flow graphs through `.block` and `.flow` files.

## Table of Contents

1. [Overview](#overview)
2. [Block Files (.block)](#block-files-block)
3. [Flow Files (.flow)](#flow-files-flow)
4. [Data Types](#data-types)
5. [Examples](#examples)

## Overview

The Circuit Language consists of two main file types:

- **`.block` files**: Define reusable computational blocks (similar to function definitions)
- **`.flow` files**: Define complete graphs/workflows that connect blocks together (similar to programs)

Both file types use a simple, readable syntax designed to be easy to write and maintain.

## Block Files (.block)

Block files define reusable computational units with inputs, outputs, configuration parameters, and execution logic.

### Basic Syntax

```
block <qualified.name> {
    description "Human-readable description"

    input <name>: <Type> {
        description "Input description"
        default = <value>
    }

    output <name>: <Type> {
        description "Output description"
    }

    config <name>: <Type> {
        description "Config parameter description"
        default = <value>
    }

    execute {
        // Execution statements
    }
}
```

### Components

#### Block Name
- Must be a qualified name (e.g., `math.add`, `string.format`, `my.namespace.block`)
- Used to reference the block in flow files

#### Description
- Optional human-readable description
- Syntax: `description "text"`

#### Inputs
- Define input ports that receive data
- Required fields: `name` and `type`
- Optional: `description` and `default` value

#### Outputs
- Define output ports that produce data
- Required fields: `name` and `type`
- Optional: `description`

#### Config
- Define configuration parameters set at graph-design time
- Similar to inputs but values are static
- Optional: `default` value

#### Execute Block
- Contains the computation logic
- Uses a simple expression language
- Supports: assignment, arithmetic, conditionals, function calls

### Execute Block Syntax

```
execute {
    // Assignment
    output = input1 + input2

    // Conditionals
    if condition {
        output = value1
    }

    // Expressions
    result = (a + b) * c - d

    // Function calls
    result = pow(base, exponent)

    // Member access
    result = object.property.method()
}
```

### Complete Block Example

```
block math.power {
    description "Raises a base to an exponent (base^exponent)"

    input base: Number {
        description "The base number"
    }

    input exponent: Number {
        description "The exponent"
        default = 2
    }

    output result: Number {
        description "The result of base^exponent"
    }

    execute {
        result = base * base
    }
}
```

## Flow Files (.flow)

Flow files define complete computational graphs by instantiating blocks and connecting them together.

### Basic Syntax

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

### Components

#### Flow Name
- Simple identifier (not qualified)
- Used to reference the flow

#### Node Definitions
- Each node is an instance of a block
- `id`: Unique identifier for this node instance
- `block.type`: The qualified name of the block to instantiate
- Configuration values can be set inline
- Optional `position(x, y)` for visual layout

#### Connections
- Define data flow between nodes
- Format: `connect from.port -> to.port`
- Creates a directed edge in the graph

#### Outputs
- Specify which node outputs are exposed as flow outputs
- Format: `output node.port`

### Complete Flow Example

```
flow calculator {
    description "Simple calculator: (5 + 3) * 2 = 16"

    // Define constant values
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

    // Define operations
    node add: math.add {
        position(250, 150)
    }

    node multiply: math.multiply {
        position(550, 150)
    }

    // Connect the graph
    connect const5.value -> add.a
    connect const3.value -> add.b
    connect add.result -> multiply.a
    connect const2.value -> multiply.b

    // Expose final result
    output multiply.result
}
```

## Data Types

The Circuit Language supports the following data types:

| Type | Description | Example Values |
|------|-------------|----------------|
| `Number` | Floating-point numbers | `42`, `3.14`, `-10.5` |
| `String` | Text strings | `"hello"`, `"world"` |
| `Bool` | Boolean values | `true`, `false` |
| `Array` | Ordered lists | `[1, 2, 3]`, `["a", "b"]` |
| `Object` | Key-value maps | `{"key": "value", "count": 10}` |
| `Bytes` | Binary data | (not directly constructible in language) |
| `Any` | Any type | Any of the above |

### Value Literals

```
// Null
null

// Booleans
true
false

// Numbers
42
3.14159
-10.5

// Strings
"hello world"
"multi word string"

// Arrays
[1, 2, 3]
["a", "b", "c"]
[true, false, true]

// Objects
{"name": "Alice", "age": 30}
{"x": 10, "y": 20, "label": "point"}

// Nested structures
{
    "user": "bob",
    "scores": [10, 20, 30],
    "active": true
}
```

## Examples

### Example 1: String Formatting Block

```block
block string.format {
    description "Formats a string with prefix and suffix"

    input template: String {
        description "The template string"
    }

    config prefix: String {
        description "Prefix to add"
        default = ""
    }

    config suffix: String {
        description "Suffix to add"
        default = ""
    }

    output result: String {
        description "The formatted string"
    }

    execute {
        result = prefix + template + suffix
    }
}
```

### Example 2: Comparison Block

```block
block logic.compare {
    description "Compares two numbers"

    input a: Number
    input b: Number

    output equal: Bool
    output greater: Bool
    output less: Bool

    execute {
        equal = a == b
        greater = a > b
        less = a < b
    }
}
```

### Example 3: Data Pipeline Flow

```flow
flow data_pipeline {
    description "Process data through multiple stages"

    node input: core.constant {
        value = 10
    }

    node stage1: math.multiply {
        b = 2
    }

    node stage2: math.add {
        b = 5
    }

    node output: core.debug

    connect input.value -> stage1.a
    connect stage1.result -> stage2.a
    connect stage2.result -> output.input

    output output.output
}
```

### Example 4: String Processing Flow

```flow
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

## Usage

### Parsing Block Files

```rust
use circuit_lang::parse_block;

let source = std::fs::read_to_string("my_block.block")?;
let block = parse_block(&source)?;

println!("Block: {}", block.name);
println!("Inputs: {}", block.inputs.len());
println!("Outputs: {}", block.outputs.len());
```

### Parsing Flow Files

```rust
use circuit_lang::parse_flow;

let source = std::fs::read_to_string("my_flow.flow")?;
let flow = parse_flow(&source)?;

println!("Flow: {}", flow.name);
println!("Nodes: {}", flow.nodes.len());
println!("Connections: {}", flow.connections.len());
```

### Converting Flow to Graph

```rust
use circuit_lang::{parse_flow, flow_to_graph};

let source = std::fs::read_to_string("my_flow.flow")?;
let flow = parse_flow(&source)?;
let graph = flow_to_graph(&flow)?;

// Now you can use the graph with the Circuit engine
engine.load_graph(graph)?;
```

## Best Practices

1. **Use descriptive names**: Choose clear, self-documenting names for blocks, nodes, and ports
2. **Add descriptions**: Always include descriptions for blocks and ports
3. **Provide defaults**: Set sensible default values for optional inputs and config parameters
4. **Organize blocks**: Use qualified names to organize blocks into namespaces (e.g., `math.`, `string.`, `io.`)
5. **Position nodes**: Use position statements in flows for better visualization
6. **Comment your flows**: Use the description field to explain what your flow does

## Syntax Rules

- **Comments**: Use `//` for single-line comments
- **Whitespace**: Whitespace is flexible and ignored
- **Identifiers**: Must start with a letter, can contain letters, numbers, and underscores
- **Qualified names**: Use dots to create namespaces (e.g., `math.advanced.fft`)
- **Case sensitivity**: All identifiers are case-sensitive

## Error Handling

The parser provides detailed error messages:

```rust
match parse_block(source) {
    Ok(block) => println!("Successfully parsed: {}", block.name),
    Err(e) => eprintln!("Parse error: {}", e),
}
```

Common errors:
- Missing required fields (inputs, outputs must have types)
- Invalid syntax in execute blocks
- Malformed connections (must be `node.port -> node.port`)
- Type mismatches
- Invalid identifiers

## Future Extensions

Planned features for future versions:

- Type checking and validation
- Custom data types
- Async/streaming operations
- Error propagation in flows
- Conditional connections
- Loops and recursion
- Import/export of blocks
- Standard library of common blocks
