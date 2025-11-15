# Block Syntax

Block files define reusable computational units with inputs, outputs, configuration parameters, and execution logic.

## Basic Structure

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

## Block Name

The block name must be a qualified identifier using dot notation:

```
block math.add { ... }
block string.format { ... }
block myapp.custom.processor { ... }
```

**Best practices:**
- Use namespaces to organize related blocks
- Keep names descriptive but concise
- Use lowercase with dots for namespacing

## Description

Optional but highly recommended:

```
block math.square {
    description "Squares a number (x²)"
    ...
}
```

## Inputs

Define input ports that receive data:

```
input x: Number {
    description "The number to square"
}

input optional_value: Number {
    description "An optional input"
    default = 0
}
```

**Properties:**
- `name`: Port identifier (required)
- `type`: Data type (required)
- `description`: Human-readable description (optional)
- `default`: Default value if not connected (optional)

## Outputs

Define output ports that produce data:

```
output result: Number {
    description "The squared result"
}
```

**Properties:**
- `name`: Port identifier (required)
- `type`: Data type (required)
- `description`: Human-readable description (optional)

## Config Parameters

Configuration values set at design-time:

```
config multiplier: Number {
    description "Multiplication factor"
    default = 1
}
```

**Difference from inputs:**
- Config values are static (set when creating the node)
- Inputs are dynamic (data flowing through the graph)

## Execute Block

Contains the computation logic:

```
execute {
    result = x * x
}
```

### Supported Operations

**Assignment:**
```
output = input
result = calculation
```

**Arithmetic:**
```
result = a + b
result = a - b
result = a * b
result = a / b
```

**Comparisons:**
```
equal = a == b
not_equal = a != b
greater = a > b
less = a < b
```

**Logical:**
```
and_result = a && b
or_result = a || b
not_result = !value
```

**Conditionals:**
```
if condition {
    output = value1
}
```

**Function Calls:**
```
result = pow(base, exponent)
result = sqrt(value)
```

**Member Access:**
```
result = object.property
result = object.method()
```

## Complete Examples

### Example 1: Simple Math Block

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

### Example 2: Block with Config

```
block math.power {
    description "Raises a base to an exponent"

    input base: Number {
        description "The base number"
    }

    config exponent: Number {
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

### Example 3: String Processing

```
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

### Example 4: Multiple Outputs

```
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

## Loading Block Definitions

Use the `circuit-lang` crate to parse block files:

```rust
use circuit_lang::parse_block;

let source = std::fs::read_to_string("math.square.block")?;
let block = parse_block(&source)?;

println!("Block: {}", block.name);
println!("Inputs: {}", block.inputs.len());
println!("Outputs: {}", block.outputs.len());
```

## See Also

- [Flow Syntax](./flow-syntax.md) - How to use blocks in flows
- [Type System](./types.md) - Available data types
- [Creating Custom Blocks](../custom-blocks/block-files.md) - Practical guide
