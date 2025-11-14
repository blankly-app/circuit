# Circuit API Reference

## Core Concepts

### Engine

The `Engine` is the main runtime that manages blocks and executes graphs.

```rust
pub struct Engine {
    blocks: BlockRegistry,
    pub graphs: HashMap<String, Graph>,
}
```

#### Methods

- `new() -> Self` - Create a new engine instance
- `register_block(&mut self, block: Arc<dyn Block>) -> Result<()>` - Register a block type
- `load_graph(&mut self, graph: Graph) -> Result<()>` - Load a graph
- `execute_graph(&self, graph_id: &str) -> Result<HashMap<NodeId, HashMap<String, Value>>>` - Execute a graph
- `execute(&self, graph: &Graph) -> Result<HashMap<NodeId, HashMap<String, Value>>>` - Execute a graph directly
- `list_blocks(&self) -> Vec<String>` - Get registered block types
- `list_graphs(&self) -> Vec<String>` - Get loaded graphs

### Block

The `Block` trait defines a computational unit.

```rust
pub trait Block: Send + Sync {
    fn metadata(&self) -> BlockMetadata;
    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>>;
    fn validate(&self, _config: &HashMap<String, Value>) -> Result<()>;
}
```

#### BlockMetadata

```rust
pub struct BlockMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub inputs: Vec<PortDefinition>,
    pub outputs: Vec<PortDefinition>,
    pub config_schema: HashMap<String, String>,
}
```

#### BlockContext

```rust
pub struct BlockContext {
    pub inputs: HashMap<String, Value>,
    pub config: HashMap<String, Value>,
}
```

Methods:
- `new() -> Self`
- `get_input(&self, port_id: &str) -> Option<&Value>`
- `get_config(&self, key: &str) -> Option<&Value>`

### Graph

A directed graph of nodes and connections.

```rust
pub struct Graph {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub nodes: HashMap<NodeId, Node>,
    pub connections: Vec<Connection>,
}
```

#### Methods

- `new(id: String, name: String) -> Self`
- `add_node(&mut self, node: Node) -> Result<()>`
- `remove_node(&mut self, node_id: &str) -> Result<()>`
- `add_connection(&mut self, connection: Connection) -> Result<()>`
- `topological_sort(&self) -> Result<Vec<NodeId>>` - Get execution order
- `get_incoming_connections(&self, node_id: &str) -> Vec<&Connection>`

#### Node

```rust
pub struct Node {
    pub id: NodeId,
    pub block_type: String,
    pub config: HashMap<String, Value>,
    pub position: Option<(f64, f64)>,
}
```

#### Connection

```rust
pub struct Connection {
    pub from_node: NodeId,
    pub from_port: String,
    pub to_node: NodeId,
    pub to_port: String,
}
```

### Value

Type-safe values that flow through the graph.

```rust
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Bytes(Vec<u8>),
}
```

#### Methods

- `is_null(&self) -> bool`
- `as_bool(&self) -> Option<bool>`
- `as_int(&self) -> Option<i64>`
- `as_float(&self) -> Option<f64>`
- `as_str(&self) -> Option<&str>`
- `as_array(&self) -> Option<&Vec<Value>>`
- `as_object(&self) -> Option<&HashMap<String, Value>>`

### Error Types

```rust
pub enum CircuitError {
    BlockExecution(String),
    Graph(String),
    NodeNotFound(String),
    InvalidConnection(String),
    Serialization(serde_json::Error),
    CycleDetected,
    InvalidInput(String),
    TypeMismatch { expected: String, actual: String },
    Other(anyhow::Error),
}
```

## Built-in Blocks

### Math Blocks

#### AddBlock (`math.add`)

Adds two numbers together.

**Inputs:**
- `a` (number, required)
- `b` (number, required)

**Outputs:**
- `result` (number)

#### MultiplyBlock (`math.multiply`)

Multiplies two numbers together.

**Inputs:**
- `a` (number, required)
- `b` (number, required)

**Outputs:**
- `result` (number)

### Core Blocks

#### ConstantBlock (`core.constant`)

Outputs a constant value from configuration.

**Configuration:**
- `value` (any)

**Outputs:**
- `value` (any)

#### DebugBlock (`core.debug`)

Prints debug information and passes through the value.

**Inputs:**
- `value` (any, required)

**Outputs:**
- `value` (any)

### String Blocks

#### ConcatBlock (`string.concat`)

Concatenates two strings.

**Inputs:**
- `a` (string, required)
- `b` (string, required)

**Outputs:**
- `result` (string)

## Platform APIs

### WebAssembly (JavaScript/TypeScript)

```typescript
class WasmEngine {
    constructor();
    loadGraph(graphJson: string): void;
    executeGraph(graphId: string): string;
    listBlocks(): string[];
    listGraphs(): string[];
}
```

### FFI (C/Swift/Kotlin)

```c
uint64_t circuit_engine_create();
void circuit_engine_destroy(uint64_t handle);
int32_t circuit_load_graph(uint64_t handle, const char* json, char** error);
char* circuit_execute_graph(uint64_t handle, const char* graph_id, char** error);
void circuit_free_string(char* s);
```

## Usage Examples

### Creating a Simple Graph

```rust
use circuit_core::*;
use std::sync::Arc;

let mut engine = Engine::new();
engine.register_block(Arc::new(blocks::AddBlock)).unwrap();
engine.register_block(Arc::new(blocks::ConstantBlock)).unwrap();

let mut graph = Graph::new("calc".to_string(), "Calculator".to_string());

// Add nodes
let node1 = graph::Node {
    id: "const1".to_string(),
    block_type: "core.constant".to_string(),
    config: {
        let mut cfg = HashMap::new();
        cfg.insert("value".to_string(), Value::Float(5.0));
        cfg
    },
    position: None,
};
graph.add_node(node1).unwrap();

// Add connections and execute
```

### Serializing Graphs

Graphs can be serialized to/from JSON:

```rust
use serde_json;

// Serialize
let json = serde_json::to_string(&graph)?;

// Deserialize
let graph: Graph = serde_json::from_str(&json)?;
```

### Error Handling

```rust
match engine.execute_graph("my-graph") {
    Ok(results) => {
        // Process results
    }
    Err(CircuitError::NodeNotFound(id)) => {
        eprintln!("Node {} not found", id);
    }
    Err(CircuitError::CycleDetected) => {
        eprintln!("Graph contains a cycle");
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Thread Safety

- `Engine` can be shared across threads using `Arc<Mutex<Engine>>`
- Blocks must be `Send + Sync`
- Graph execution is stateless (no mutation during execution)

## Performance Considerations

1. **Graph Validation**: Validation happens when adding connections, not during execution
2. **Topological Sort**: Computed once per execution
3. **Value Cloning**: Values are cloned when passing between nodes
4. **Block Registration**: Blocks are stored as `Arc<dyn Block>` for efficient sharing

## Extending Circuit

### Custom Block Example

```rust
use circuit_core::*;

struct CustomBlock;

impl Block for CustomBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "custom.block".to_string(),
            name: "Custom Block".to_string(),
            description: "Does custom processing".to_string(),
            inputs: vec![/* ... */],
            outputs: vec![/* ... */],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        // Custom logic here
        Ok(HashMap::new())
    }

    fn validate(&self, config: &HashMap<String, Value>) -> Result<()> {
        // Optional validation
        Ok(())
    }
}
```

### Platform Integration

See platform-specific documentation:
- [Swift/iOS](SWIFT.md)
- [Kotlin/Android](KOTLIN.md)
- [React/Web](REACT.md)
