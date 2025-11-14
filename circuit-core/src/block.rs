use crate::{error::Result, value::Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata about a block type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    /// Unique identifier for the block type
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of what the block does
    pub description: String,
    /// Input port definitions
    pub inputs: Vec<PortDefinition>,
    /// Output port definitions
    pub outputs: Vec<PortDefinition>,
    /// Configuration schema
    pub config_schema: HashMap<String, String>,
}

/// Definition of an input or output port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDefinition {
    /// Port identifier
    pub id: String,
    /// Port name
    pub name: String,
    /// Expected data type
    pub data_type: String,
    /// Whether this port is required
    pub required: bool,
}

/// Context provided to a block during execution
pub struct BlockContext {
    /// Input values from connected nodes
    pub inputs: HashMap<String, Value>,
    /// Block-specific configuration
    pub config: HashMap<String, Value>,
}

impl BlockContext {
    /// Create a new block context
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            config: HashMap::new(),
        }
    }

    /// Get an input value by port ID
    pub fn get_input(&self, port_id: &str) -> Option<&Value> {
        self.inputs.get(port_id)
    }

    /// Get a config value by key
    pub fn get_config(&self, key: &str) -> Option<&Value> {
        self.config.get(key)
    }
}

impl Default for BlockContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait that all blocks must implement
pub trait Block: Send + Sync {
    /// Get metadata about this block
    fn metadata(&self) -> BlockMetadata;

    /// Execute the block with given context
    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>>;

    /// Validate the block configuration (optional)
    fn validate(&self, _config: &HashMap<String, Value>) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBlock;

    impl Block for TestBlock {
        fn metadata(&self) -> BlockMetadata {
            BlockMetadata {
                id: "test".to_string(),
                name: "Test Block".to_string(),
                description: "A test block".to_string(),
                inputs: vec![],
                outputs: vec![],
                config_schema: HashMap::new(),
            }
        }

        fn execute(&self, _context: BlockContext) -> Result<HashMap<String, Value>> {
            let mut outputs = HashMap::new();
            outputs.insert("result".to_string(), Value::Int(42));
            Ok(outputs)
        }
    }

    #[test]
    fn test_block_execution() {
        let block = TestBlock;
        let context = BlockContext::new();
        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Int(42)));
    }
}
