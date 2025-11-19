use crate::block::{Block, BlockContext, BlockMetadata, PortDefinition};
use crate::error::{CircuitError, Result};
use crate::value::Value;
use std::collections::HashMap;

/// Output a constant value
pub struct ConstantBlock;

impl Block for ConstantBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "core.constant".to_string(),
            name: "Constant".to_string(),
            description: "Outputs a constant value".to_string(),
            inputs: vec![],
            outputs: vec![PortDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                data_type: "any".to_string(),
                required: true,
            }],
            config_schema: {
                let mut schema = HashMap::new();
                schema.insert("value".to_string(), "any".to_string());
                schema
            },
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

/// Debug block that prints values
pub struct DebugBlock;

impl Block for DebugBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "core.debug".to_string(),
            name: "Debug".to_string(),
            description: "Print debug information".to_string(),
            inputs: vec![PortDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                data_type: "any".to_string(),
                required: true,
            }],
            outputs: vec![PortDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                data_type: "any".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let value = context
            .get_input("value")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input 'value'".to_string()))?
            .clone();

        println!("DEBUG: {:?}", value);

        let mut outputs = HashMap::new();
        outputs.insert("value".to_string(), value);
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_block() {
        let block = ConstantBlock;
        let mut context = BlockContext::new();
        context
            .config
            .insert("value".to_string(), Value::String("Hello".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("value"),
            Some(&Value::String("Hello".to_string()))
        );
    }
}
