use crate::{
    block::{Block, BlockContext},
    error::{CircuitError, Result},
    graph::{Graph, NodeId},
    value::Value,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Block registry that maps block type IDs to block implementations
pub type BlockRegistry = HashMap<String, Arc<dyn Block>>;

/// The main execution engine for running graphs
pub struct Engine {
    /// Registered block types
    blocks: BlockRegistry,
    /// Loaded graphs
    pub graphs: HashMap<String, Graph>,
}

impl Engine {
    /// Create a new engine instance
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            graphs: HashMap::new(),
        }
    }

    /// Register a block type with the engine
    pub fn register_block(&mut self, block: Arc<dyn Block>) -> Result<()> {
        let metadata = block.metadata();
        if self.blocks.contains_key(&metadata.id) {
            return Err(CircuitError::Graph(format!(
                "Block type '{}' is already registered",
                metadata.id
            )));
        }
        self.blocks.insert(metadata.id, block);
        Ok(())
    }

    /// Load a graph into the engine
    pub fn load_graph(&mut self, graph: Graph) -> Result<()> {
        // Validate that all block types are registered
        for node in graph.nodes.values() {
            if !self.blocks.contains_key(&node.block_type) {
                return Err(CircuitError::Graph(format!(
                    "Unknown block type: {}",
                    node.block_type
                )));
            }
        }

        self.graphs.insert(graph.id.clone(), graph);
        Ok(())
    }

    /// Execute a graph by ID
    pub fn execute_graph(&self, graph_id: &str) -> Result<HashMap<NodeId, HashMap<String, Value>>> {
        let graph = self
            .graphs
            .get(graph_id)
            .ok_or_else(|| CircuitError::Graph(format!("Graph '{}' not found", graph_id)))?;

        self.execute(graph)
    }

    /// Execute a graph
    pub fn execute(&self, graph: &Graph) -> Result<HashMap<NodeId, HashMap<String, Value>>> {
        // Get execution order
        let execution_order = graph.topological_sort()?;

        // Store outputs from each node
        let mut node_outputs: HashMap<NodeId, HashMap<String, Value>> = HashMap::new();

        // Execute nodes in topological order
        for node_id in execution_order {
            let node = graph
                .nodes
                .get(&node_id)
                .ok_or_else(|| CircuitError::NodeNotFound(node_id.clone()))?;

            let block = self.blocks.get(&node.block_type).ok_or_else(|| {
                CircuitError::Graph(format!("Block type '{}' not found", node.block_type))
            })?;

            // Build context for this node
            let mut context = BlockContext::new();
            context.config = node.config.clone();

            // Gather inputs from connected nodes
            for connection in graph.get_incoming_connections(&node_id) {
                if let Some(source_outputs) = node_outputs.get(&connection.from_node) {
                    if let Some(value) = source_outputs.get(&connection.from_port) {
                        context
                            .inputs
                            .insert(connection.to_port.clone(), value.clone());
                    }
                }
            }

            // Execute the block
            let outputs = block
                .execute(context)
                .map_err(|e| CircuitError::BlockExecution(format!("Node '{}': {}", node_id, e)))?;

            node_outputs.insert(node_id.clone(), outputs);
        }

        Ok(node_outputs)
    }

    /// Get list of registered block types
    pub fn list_blocks(&self) -> Vec<String> {
        self.blocks.keys().cloned().collect()
    }

    /// Get list of loaded graphs
    pub fn list_graphs(&self) -> Vec<String> {
        self.graphs.keys().cloned().collect()
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{BlockMetadata, PortDefinition};
    use crate::graph::{Connection, Node};

    struct AddBlock;
    impl Block for AddBlock {
        fn metadata(&self) -> BlockMetadata {
            BlockMetadata {
                id: "add".to_string(),
                name: "Add".to_string(),
                description: "Adds two numbers".to_string(),
                inputs: vec![
                    PortDefinition {
                        id: "a".to_string(),
                        name: "A".to_string(),
                        data_type: "number".to_string(),
                        required: true,
                    },
                    PortDefinition {
                        id: "b".to_string(),
                        name: "B".to_string(),
                        data_type: "number".to_string(),
                        required: true,
                    },
                ],
                outputs: vec![PortDefinition {
                    id: "result".to_string(),
                    name: "Result".to_string(),
                    data_type: "number".to_string(),
                    required: true,
                }],
                config_schema: HashMap::new(),
            }
        }

        fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
            let a = context
                .get_input("a")
                .and_then(|v| v.as_float())
                .ok_or_else(|| CircuitError::InvalidInput("Missing input 'a'".to_string()))?;
            let b = context
                .get_input("b")
                .and_then(|v| v.as_float())
                .ok_or_else(|| CircuitError::InvalidInput("Missing input 'b'".to_string()))?;

            let mut outputs = HashMap::new();
            outputs.insert("result".to_string(), Value::Float(a + b));
            Ok(outputs)
        }
    }

    struct ConstantBlock;
    impl Block for ConstantBlock {
        fn metadata(&self) -> BlockMetadata {
            BlockMetadata {
                id: "constant".to_string(),
                name: "Constant".to_string(),
                description: "Outputs a constant value".to_string(),
                inputs: vec![],
                outputs: vec![PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "number".to_string(),
                    required: true,
                }],
                config_schema: HashMap::new(),
            }
        }

        fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
            let value = context
                .get_config("value")
                .ok_or_else(|| CircuitError::InvalidInput("Missing config 'value'".to_string()))?
                .clone();

            let mut outputs = HashMap::new();
            outputs.insert("value".to_string(), value);
            Ok(outputs)
        }
    }

    #[test]
    fn test_engine_registration() {
        let mut engine = Engine::new();
        engine.register_block(Arc::new(AddBlock)).unwrap();
        assert_eq!(engine.list_blocks().len(), 1);
    }

    #[test]
    fn test_simple_execution() {
        let mut engine = Engine::new();
        engine.register_block(Arc::new(ConstantBlock)).unwrap();
        engine.register_block(Arc::new(AddBlock)).unwrap();

        // Create a simple graph: const1(5) + const2(3) = add(8)
        let mut graph = Graph::new("test".to_string(), "Test Graph".to_string());

        // Create constant nodes
        let mut config1 = HashMap::new();
        config1.insert("value".to_string(), Value::Float(5.0));
        let node1 = Node {
            id: "const1".to_string(),
            block_type: "constant".to_string(),
            config: config1,
            position: None,
        };

        let mut config2 = HashMap::new();
        config2.insert("value".to_string(), Value::Float(3.0));
        let node2 = Node {
            id: "const2".to_string(),
            block_type: "constant".to_string(),
            config: config2,
            position: None,
        };

        // Create add node
        let node3 = Node {
            id: "add".to_string(),
            block_type: "add".to_string(),
            config: HashMap::new(),
            position: None,
        };

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();
        graph.add_node(node3).unwrap();

        // Connect nodes
        graph
            .add_connection(Connection {
                from_node: "const1".to_string(),
                from_port: "value".to_string(),
                to_node: "add".to_string(),
                to_port: "a".to_string(),
            })
            .unwrap();

        graph
            .add_connection(Connection {
                from_node: "const2".to_string(),
                from_port: "value".to_string(),
                to_node: "add".to_string(),
                to_port: "b".to_string(),
            })
            .unwrap();

        engine.load_graph(graph).unwrap();

        // Execute
        let results = engine.execute_graph("test").unwrap();

        // Verify result
        let add_output = results.get("add").unwrap();
        let result = add_output.get("result").unwrap();
        assert_eq!(result.as_float(), Some(8.0));
    }
}
