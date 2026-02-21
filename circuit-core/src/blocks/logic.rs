use crate::block::{Block, BlockContext, BlockMetadata, PortDefinition};
use crate::error::{CircuitError, Result};
use crate::value::Value;
use std::collections::HashMap;

/// Logical AND of two booleans
pub struct AndBlock;

impl Block for AndBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.and".to_string(),
            name: "And".to_string(),
            description: "Logical AND of two booleans".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "a".to_string(),
                    name: "A".to_string(),
                    data_type: "bool".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "b".to_string(),
                    name: "B".to_string(),
                    data_type: "bool".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "bool".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let a = context
            .get_input("a")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'a'".to_string())
            })?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'b'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Bool(a && b));
        Ok(outputs)
    }
}

/// Logical OR of two booleans
pub struct OrBlock;

impl Block for OrBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.or".to_string(),
            name: "Or".to_string(),
            description: "Logical OR of two booleans".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "a".to_string(),
                    name: "A".to_string(),
                    data_type: "bool".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "b".to_string(),
                    name: "B".to_string(),
                    data_type: "bool".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "bool".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let a = context
            .get_input("a")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'a'".to_string())
            })?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'b'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Bool(a || b));
        Ok(outputs)
    }
}

/// Logical NOT of a boolean
pub struct NotBlock;

impl Block for NotBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.not".to_string(),
            name: "Not".to_string(),
            description: "Logical NOT of a boolean".to_string(),
            inputs: vec![PortDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                data_type: "bool".to_string(),
                required: true,
            }],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "bool".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let value = context
            .get_input("value")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Bool(!value));
        Ok(outputs)
    }
}

/// Check if two values are equal
pub struct EqualBlock;

impl Block for EqualBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.equal".to_string(),
            name: "Equal".to_string(),
            description: "Check if two values are equal".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "a".to_string(),
                    name: "A".to_string(),
                    data_type: "any".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "b".to_string(),
                    name: "B".to_string(),
                    data_type: "any".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "bool".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let a = context
            .get_input("a")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input 'a'".to_string()))?;
        let b = context
            .get_input("b")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input 'b'".to_string()))?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Bool(a == b));
        Ok(outputs)
    }
}

/// Check if A > B (numbers)
pub struct GreaterBlock;

impl Block for GreaterBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.greater".to_string(),
            name: "Greater Than".to_string(),
            description: "Check if A > B".to_string(),
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
                data_type: "bool".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let a = context
            .get_input("a")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'a'".to_string())
            })?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'b'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Bool(a > b));
        Ok(outputs)
    }
}

/// Check if A < B (numbers)
pub struct LessBlock;

impl Block for LessBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.less".to_string(),
            name: "Less Than".to_string(),
            description: "Check if A < B".to_string(),
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
                data_type: "bool".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let a = context
            .get_input("a")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'a'".to_string())
            })?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'b'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Bool(a < b));
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_block() {
        let block = AndBlock;
        let mut context = BlockContext::new();

        // True && True = True
        context.inputs.insert("a".to_string(), Value::Bool(true));
        context.inputs.insert("b".to_string(), Value::Bool(true));
        let result = block.execute(context.clone()).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(true)));

        // True && False = False
        context.inputs.insert("b".to_string(), Value::Bool(false));
        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_or_block() {
        let block = OrBlock;
        let mut context = BlockContext::new();

        // False || False = False
        context.inputs.insert("a".to_string(), Value::Bool(false));
        context.inputs.insert("b".to_string(), Value::Bool(false));
        let result = block.execute(context.clone()).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(false)));

        // False || True = True
        context.inputs.insert("b".to_string(), Value::Bool(true));
        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_not_block() {
        let block = NotBlock;
        let mut context = BlockContext::new();

        // !True = False
        context
            .inputs
            .insert("value".to_string(), Value::Bool(true));
        let result = block.execute(context.clone()).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(false)));

        // !False = True
        context
            .inputs
            .insert("value".to_string(), Value::Bool(false));
        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_equal_block() {
        let block = EqualBlock;
        let mut context = BlockContext::new();

        // 5 == 5
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(5.0));
        let result = block.execute(context.clone()).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(true)));

        // 5 != 3
        context.inputs.insert("b".to_string(), Value::Float(3.0));
        let result = block.execute(context.clone()).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(false)));

        // "hello" == "hello"
        context
            .inputs
            .insert("a".to_string(), Value::String("hello".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("hello".to_string()));
        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_greater_block() {
        let block = GreaterBlock;
        let mut context = BlockContext::new();

        // 5 > 3 = True
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));
        let result = block.execute(context.clone()).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(true)));

        // 3 > 5 = False
        context.inputs.insert("a".to_string(), Value::Float(3.0));
        context.inputs.insert("b".to_string(), Value::Float(5.0));
        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_less_block() {
        let block = LessBlock;
        let mut context = BlockContext::new();

        // 3 < 5 = True
        context.inputs.insert("a".to_string(), Value::Float(3.0));
        context.inputs.insert("b".to_string(), Value::Float(5.0));
        let result = block.execute(context.clone()).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(true)));

        // 5 < 3 = False
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));
        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(false)));
    }
}
