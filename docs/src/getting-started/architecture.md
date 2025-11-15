# Architecture Overview

Understanding Circuit's architecture will help you build better applications and debug issues more effectively.

## High-Level Architecture

Circuit consists of four main layers:

```
┌─────────────────────────────────────────────────────┐
│         Platform Layer (Swift/Kotlin/JS)            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐  │
│  │     iOS      │  │   Android    │  │   Web    │  │
│  └──────────────┘  └──────────────┘  └──────────┘  │
└─────────────────────────────────────────────────────┘
                         │
┌─────────────────────────────────────────────────────┐
│          FFI/WASM Bindings Layer                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐  │
│  │ circuit-ffi  │  │ circuit-ffi  │  │circuit-  │  │
│  │   (Swift)    │  │  (Kotlin)    │  │  wasm    │  │
│  └──────────────┘  └──────────────┘  └──────────┘  │
└─────────────────────────────────────────────────────┘
                         │
┌─────────────────────────────────────────────────────┐
│              Language Layer                         │
│              ┌──────────────┐                       │
│              │ circuit-lang │                       │
│              │   Parser &   │                       │
│              │  Converter   │                       │
│              └──────────────┘                       │
└─────────────────────────────────────────────────────┘
                         │
┌─────────────────────────────────────────────────────┐
│               Core Runtime Engine                   │
│              ┌──────────────┐                       │
│              │ circuit-core │                       │
│              │   Engine,    │                       │
│              │  Blocks,     │                       │
│              │   Graphs     │                       │
│              └──────────────┘                       │
└─────────────────────────────────────────────────────┘
```

## Core Components

### 1. circuit-core: The Runtime Engine

The heart of Circuit, `circuit-core` provides:

#### Blocks

The fundamental unit of computation. Each block:

- Has **inputs** (data it receives)
- Has **outputs** (data it produces)
- Has **configuration** (parameters that customize behavior)
- Implements an **execute** method (the actual computation)

```rust
pub trait Block: Send + Sync {
    fn metadata(&self) -> BlockMetadata;
    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>>;
    fn validate(&self, _config: &HashMap<String, Value>) -> Result<()> { Ok(()) }
}
```

#### Graphs

A directed acyclic graph (DAG) representing data flow:

- **Nodes**: Instances of blocks with unique IDs
- **Connections**: Data paths between node ports
- **Cycle Detection**: Ensures no circular dependencies
- **Validation**: Checks that all connections are valid

#### Engine

The execution runtime that:

1. **Registers** blocks (makes them available)
2. **Loads** graphs (prepares them for execution)
3. **Executes** graphs in topological order
4. **Routes** data between connected nodes
5. **Returns** final output values

#### Values

Type-safe data containers supporting:

- `Null`, `Bool`, `Int`, `Float`, `String`
- `Array`, `Object` (nested structures)
- `Bytes` (binary data)
- Conversion and serialization via serde

### 2. circuit-lang: The Declarative Language

Provides a human-friendly way to define blocks and flows:

#### Parser

Uses Pest grammar to parse `.block` and `.flow` files into AST (Abstract Syntax Tree).

#### Converter

Transforms AST into executable `Graph` objects that the engine can run.

#### Benefits

- **Declarative**: Describe what you want, not how to build it
- **Readable**: Clear syntax for non-programmers
- **Maintainable**: Easy to modify flows without recompiling

### 3. circuit-ffi: Native Platform Bridge

C-compatible FFI layer for iOS and Android:

- **C API**: Simple functions for create, load, execute, destroy
- **Memory Management**: Safe handling of strings and pointers
- **Error Handling**: Proper error propagation across FFI boundary
- **Thread Safety**: Uses lazy_static for global engine registry

Swift and Kotlin wrappers provide idiomatic APIs on top of the C layer.

### 4. circuit-wasm: Web Assembly Bridge

WebAssembly bindings for browser and Node.js:

- **wasm-bindgen**: Automatic JavaScript bindings
- **Zero-Copy**: Efficient data transfer between JS and Rust
- **TypeScript**: Full type definitions included
- **Promise-based**: Async/await friendly API

## Data Flow

Here's how data flows through a Circuit application:

```
1. Define Flow (.flow file or Rust code)
   │
   ├─> Defines nodes (block instances)
   ├─> Defines connections (data paths)
   └─> Defines configuration (block parameters)

2. Parse/Convert (circuit-lang)
   │
   └─> Converts to Graph object

3. Load Graph (Engine)
   │
   ├─> Validates graph structure
   ├─> Checks for cycles
   └─> Computes topological order

4. Execute Graph (Engine)
   │
   ├─> Executes nodes in topological order
   ├─> Routes data through connections
   └─> Collects final outputs

5. Return Results
   │
   └─> HashMap<String, HashMap<String, Value>>
```

## Topological Execution

Circuit executes nodes in **topological order**, ensuring:

1. A node only executes after all its dependencies
2. Data flows in the correct direction
3. No deadlocks or circular dependencies

### Example

For this flow:

```flow
flow example {
    node a: core.constant { value = 1 }
    node b: core.constant { value = 2 }
    node c: math.add
    node d: math.multiply

    connect a.value -> c.a
    connect b.value -> c.b
    connect c.result -> d.a
}
```

Execution order:

1. `a` and `b` execute first (no dependencies)
2. `c` executes next (depends on `a` and `b`)
3. `d` executes last (depends on `c`)

## Memory Model

### Rust Side

- **Blocks**: `Arc<dyn Block>` - Thread-safe, shared references
- **Graphs**: Owned by Engine, stored in HashMap
- **Values**: Cloned when passed between nodes (cheap for small values)

### FFI Boundary

- **Strings**: Converted between Rust String and C char*
- **Pointers**: Engine handles stored in global registry
- **Memory Safety**: Rust manages all allocations

### WASM Boundary

- **Values**: Serialized as JSON across boundary
- **Optimization**: Uses wasm-bindgen for efficient marshaling
- **Memory**: WASM linear memory managed by Rust

## Error Handling

Circuit uses Rust's `Result` type throughout:

```rust
pub type Result<T> = std::result::Result<T, CircuitError>;

pub enum CircuitError {
    BlockNotFound(String),
    GraphNotFound(String),
    InvalidInput(String),
    InvalidConnection(String),
    CycleDetected,
    ExecutionError(String),
    ParseError(String),
}
```

Errors propagate through the stack and can be caught at the platform layer.

## Performance Characteristics

- **Graph Loading**: O(N + E) where N = nodes, E = edges
- **Topological Sort**: O(N + E)
- **Execution**: O(N × B) where B = average block execution time
- **Data Flow**: Cloning overhead for Values (typically small)

### Optimization Opportunities

1. **Lazy Evaluation**: Only execute nodes needed for requested outputs
2. **Parallelization**: Execute independent nodes concurrently
3. **Caching**: Memoize block results for repeated inputs
4. **Streaming**: Support streaming data for large datasets

These are future roadmap items.

## Thread Safety

- **Engine**: Not thread-safe; use one per thread or protect with Mutex
- **Blocks**: Must be `Send + Sync` (thread-safe)
- **Graphs**: Immutable after loading (safe to share)
- **Values**: Cloned between nodes (no shared mutation)

## Next Steps

- [Understanding Blocks](../guide/blocks.md) - Deep dive into blocks
- [The Graph Engine](../guide/engine.md) - Engine internals
- [Creating Custom Blocks](../guide/custom-blocks.md) - Build your own blocks
