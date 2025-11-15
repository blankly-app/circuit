# Introduction

Welcome to **Circuit** - a node-based runtime engine for building applications with computational blocks written in Rust that runs anywhere.

## What is Circuit?

Circuit is a flexible, cross-platform runtime engine that allows you to create applications using a visual node-based architecture. Write your computational blocks in Rust once, and run them on Swift (iOS/macOS), Kotlin (Android), and Web/React platforms.

## Key Features

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

## Why Circuit?

Traditional cross-platform development often requires writing the same logic multiple times for different platforms. Circuit solves this by:

1. **Write Once, Run Anywhere**: Write your computational blocks in Rust and automatically use them across all platforms
2. **Visual Programming**: Define complex data flows using an intuitive node-based system
3. **Type Safety**: Leverage Rust's type system for runtime safety across all platforms
4. **Performance**: Compiled Rust code ensures optimal performance on every platform
5. **Maintainability**: Update logic in one place and deploy everywhere

## Use Cases

Circuit is perfect for:

- **Data Processing Pipelines**: Transform and process data through reusable blocks
- **Business Logic**: Share complex business rules across mobile and web
- **Machine Learning**: Build ML inference pipelines that work everywhere
- **Game Logic**: Create game mechanics once, deploy to all platforms
- **Computational Apps**: Any application with complex computational requirements

## Quick Example

Here's a simple calculator flow that demonstrates Circuit's declarative syntax:

```flow
flow calculator {
    description "Simple calculator: (5 + 3) * 2 = 16"

    node const5: core.constant { value = 5 }
    node const3: core.constant { value = 3 }
    node add: math.add
    node multiply: math.multiply

    connect const5.value -> add.a
    connect const3.value -> add.b
    connect add.result -> multiply.a
}
```

This flow creates a calculation graph that:
1. Creates two constant values (5 and 3)
2. Adds them together
3. Multiplies the result by another value

## Next Steps

- **New to Circuit?** Start with the [Quick Start](./getting-started/quick-start.md) guide
- **Want to understand the architecture?** Read the [Architecture Overview](./getting-started/architecture.md)
- **Ready to build?** Jump to [Creating Custom Blocks](./guide/custom-blocks.md)
- **Integrating with a platform?** Check out the [Platform Integration](./platforms/overview.md) guides

## Community and Support

Circuit is actively developed and welcomes contributions. If you encounter issues or have questions:

- **GitHub Issues**: Report bugs or request features
- **Documentation**: This comprehensive guide covers all aspects of Circuit
- **Examples**: Check the `examples/` directory for working code samples

Let's get started building with Circuit!
