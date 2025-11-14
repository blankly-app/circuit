//! Example blocks showcasing how to create custom blocks for the Circuit engine
//!
//! This module demonstrates various types of blocks:
//! - Math operations (Add, Multiply, etc.)
//! - String operations (Concat, Format, etc.)
//! - Control flow (If, Switch, etc.)
//! - Data transformation (Map, Filter, etc.)

use crate::block::{Block, BlockContext, BlockMetadata, PortDefinition};
use crate::error::{CircuitError, Result};
use crate::value::Value;
use std::collections::HashMap;

/// Add two numbers together
pub struct AddBlock;

impl Block for AddBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.add".to_string(),
            name: "Add".to_string(),
            description: "Add two numbers together".to_string(),
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
            .ok_or_else(|| CircuitError::InvalidInput("Missing or invalid input 'a'".to_string()))?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_float())
            .ok_or_else(|| CircuitError::InvalidInput("Missing or invalid input 'b'".to_string()))?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(a + b));
        Ok(outputs)
    }
}

/// Multiply two numbers together
pub struct MultiplyBlock;

impl Block for MultiplyBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.multiply".to_string(),
            name: "Multiply".to_string(),
            description: "Multiply two numbers together".to_string(),
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
            .ok_or_else(|| CircuitError::InvalidInput("Missing or invalid input 'a'".to_string()))?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_float())
            .ok_or_else(|| CircuitError::InvalidInput("Missing or invalid input 'b'".to_string()))?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(a * b));
        Ok(outputs)
    }
}

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
            .ok_or_else(|| CircuitError::InvalidInput("Missing or invalid input 'a'".to_string()))?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_str())
            .ok_or_else(|| CircuitError::InvalidInput("Missing or invalid input 'b'".to_string()))?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::String(format!("{}{}", a, b)));
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
    fn test_add_block() {
        let block = AddBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(8.0)));
    }

    #[test]
    fn test_multiply_block() {
        let block = MultiplyBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(15.0)));
    }

    #[test]
    fn test_constant_block() {
        let block = ConstantBlock;
        let mut context = BlockContext::new();
        context.config.insert("value".to_string(), Value::String("Hello".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("value"), Some(&Value::String("Hello".to_string())));
    }

    #[test]
    fn test_concat_block() {
        let block = ConcatBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::String("Hello".to_string()));
        context.inputs.insert("b".to_string(), Value::String(" World".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("Hello World".to_string())));
    }
}
