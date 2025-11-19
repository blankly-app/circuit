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
        outputs.insert("result".to_string(), Value::Float(a + b));
        Ok(outputs)
    }
}

/// Subtract two numbers (a - b)
pub struct SubtractBlock;

impl Block for SubtractBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.subtract".to_string(),
            name: "Subtract".to_string(),
            description: "Subtract two numbers (a - b)".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(a - b));
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
        outputs.insert("result".to_string(), Value::Float(a * b));
        Ok(outputs)
    }
}

/// Divide two numbers (a / b)
pub struct DivideBlock;

impl Block for DivideBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.divide".to_string(),
            name: "Divide".to_string(),
            description: "Divide two numbers (a / b)".to_string(),
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
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'a'".to_string())
            })?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'b'".to_string())
            })?;

        if b == 0.0 {
            return Err(CircuitError::BlockExecution("Division by zero".to_string()));
        }

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(a / b));
        Ok(outputs)
    }
}

/// Modulo of two numbers (a % b)
pub struct ModuloBlock;

impl Block for ModuloBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.modulo".to_string(),
            name: "Modulo".to_string(),
            description: "Modulo of two numbers (a % b)".to_string(),
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
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'a'".to_string())
            })?;
        let b = context
            .get_input("b")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'b'".to_string())
            })?;

        if b == 0.0 {
            return Err(CircuitError::BlockExecution("Modulo by zero".to_string()));
        }

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(a % b));
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
    fn test_subtract_block() {
        let block = SubtractBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(7.0)));
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
    fn test_divide_block() {
        let block = DivideBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context.inputs.insert("b".to_string(), Value::Float(2.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_divide_by_zero() {
        let block = DivideBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context.inputs.insert("b".to_string(), Value::Float(0.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_block() {
        let block = ModuloBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(1.0)));
    }
}
