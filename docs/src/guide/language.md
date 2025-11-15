# The Declarative Language

Circuit provides a declarative language for defining computational blocks and flow graphs through `.block` and `.flow` files.

## Overview

The Circuit Language consists of two main file types:

- **`.block` files**: Define reusable computational blocks (similar to function definitions)
- **`.flow` files**: Define complete graphs/workflows that connect blocks together (similar to programs)

Both file types use a simple, readable syntax designed to be easy to write and maintain.

## Why Use the Declarative Language?

While you can create blocks and graphs programmatically in Rust, the declarative language offers several advantages:

1. **Readability**: Clear, self-documenting syntax
2. **Accessibility**: Non-programmers can create and modify flows
3. **Rapid Prototyping**: Quickly iterate on designs without recompiling
4. **Visual Mapping**: Easy to visualize from the textual representation
5. **Separation of Concerns**: Logic (blocks) separate from composition (flows)

## File Structure

### Block Files

```
block <qualified.name> {
    description "Human-readable description"

    input <name>: <Type> { ... }
    output <name>: <Type> { ... }
    config <name>: <Type> { ... }

    execute { ... }
}
```

### Flow Files

```
flow <name> {
    description "Human-readable description"

    node <id>: <block.type> { ... }
    connect <from>.<port> -> <to>.<port>
    output <node>.<port>
}
```

## Syntax Rules

- **Comments**: Use `//` for single-line comments
- **Whitespace**: Whitespace is flexible and ignored
- **Identifiers**: Must start with a letter, can contain letters, numbers, and underscores
- **Qualified names**: Use dots to create namespaces (e.g., `math.advanced.fft`)
- **Case sensitivity**: All identifiers are case-sensitive

## Best Practices

1. **Use descriptive names**: Choose clear, self-documenting names for blocks, nodes, and ports
2. **Add descriptions**: Always include descriptions for blocks and ports
3. **Provide defaults**: Set sensible default values for optional inputs and config parameters
4. **Organize blocks**: Use qualified names to organize blocks into namespaces (e.g., `math.`, `string.`, `io.`)
5. **Position nodes**: Use position statements in flows for better visualization
6. **Comment your flows**: Use the description field to explain what your flow does

## Next Steps

- [Block Syntax](./language/block-syntax.md) - Detailed block file syntax
- [Flow Syntax](./language/flow-syntax.md) - Detailed flow file syntax
- [Type System](./language/types.md) - Data types and values
- [Creating Custom Blocks](../custom-blocks.md) - Hands-on guide

## Examples

Check out the example files in the repository:

- `examples/blocks/` - Example block definitions
- `examples/flows/` - Example flow definitions

Run the integration tests to see them in action:

```bash
cargo test -p circuit-lang --test integration_tests
```
