use crate::block::{Block, BlockContext, BlockMetadata, PortDefinition};
use crate::error::{CircuitError, Result};
use crate::value::Value;
use std::collections::HashMap;

/// Concatenate two strings
pub struct ConcatBlock;

impl Block for ConcatBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.concat".to_string(),
            name: "Concatenate".to_string(),
            description: "Concatenate two strings".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "a".to_string(),
                    name: "String A".to_string(),
                    data_type: "string".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "b".to_string(),
                    name: "String B".to_string(),
                    data_type: "string".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "string".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let a = context
            .get_input("a")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'a'".to_string())
            })?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'b'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::String(format!("{}{}", a, b)));
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_block() {
        let block = ConcatBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("a".to_string(), Value::String("Hello".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String(" World".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("Hello World".to_string()))
        );
    }
}
