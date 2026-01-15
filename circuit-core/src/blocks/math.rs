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

/// Subtract two numbers
pub struct SubtractBlock;

impl Block for SubtractBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.subtract".to_string(),
            name: "Subtract".to_string(),
            description: "Subtract b from a".to_string(),
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

/// Divide two numbers
pub struct DivideBlock;

impl Block for DivideBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.divide".to_string(),
            name: "Divide".to_string(),
            description: "Divide a by b".to_string(),
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
            return Err(CircuitError::InvalidInput("Division by zero".to_string()));
        }

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(a / b));
        Ok(outputs)
    }
}

/// Modulo operation
pub struct ModuloBlock;

impl Block for ModuloBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.modulo".to_string(),
            name: "Modulo".to_string(),
            description: "Calculate a modulo b (remainder)".to_string(),
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
            return Err(CircuitError::InvalidInput("Modulo by zero".to_string()));
        }

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(a % b));
        Ok(outputs)
    }
}

/// Absolute value
pub struct AbsoluteBlock;

impl Block for AbsoluteBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.absolute".to_string(),
            name: "Absolute".to_string(),
            description: "Calculate absolute value of a number".to_string(),
            inputs: vec![PortDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                data_type: "number".to_string(),
                required: true,
            }],
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
        let value = context
            .get_input("value")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(value.abs()));
        Ok(outputs)
    }
}

/// Minimum of two numbers
pub struct MinBlock;

impl Block for MinBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.min".to_string(),
            name: "Minimum".to_string(),
            description: "Return the smaller of two numbers".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(a.min(b)));
        Ok(outputs)
    }
}

/// Maximum of two numbers
pub struct MaxBlock;

impl Block for MaxBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.max".to_string(),
            name: "Maximum".to_string(),
            description: "Return the larger of two numbers".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(a.max(b)));
        Ok(outputs)
    }
}

/// Check if two values are equal
pub struct EqualsBlock;

impl Block for EqualsBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.equals".to_string(),
            name: "Equals".to_string(),
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
                data_type: "boolean".to_string(),
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

/// Check if a is greater than b
pub struct GreaterThanBlock;

impl Block for GreaterThanBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.greater_than".to_string(),
            name: "Greater Than".to_string(),
            description: "Check if a is greater than b".to_string(),
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
                data_type: "boolean".to_string(),
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

/// Check if a is less than b
pub struct LessThanBlock;

impl Block for LessThanBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.less_than".to_string(),
            name: "Less Than".to_string(),
            description: "Check if a is less than b".to_string(),
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
                data_type: "boolean".to_string(),
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

/// Logical AND operation
pub struct AndBlock;

impl Block for AndBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.and".to_string(),
            name: "And".to_string(),
            description: "Logical AND of two boolean values".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "a".to_string(),
                    name: "A".to_string(),
                    data_type: "boolean".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "b".to_string(),
                    name: "B".to_string(),
                    data_type: "boolean".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "boolean".to_string(),
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

/// Logical OR operation
pub struct OrBlock;

impl Block for OrBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.or".to_string(),
            name: "Or".to_string(),
            description: "Logical OR of two boolean values".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "a".to_string(),
                    name: "A".to_string(),
                    data_type: "boolean".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "b".to_string(),
                    name: "B".to_string(),
                    data_type: "boolean".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "boolean".to_string(),
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

/// Logical NOT operation
pub struct NotBlock;

impl Block for NotBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "logic.not".to_string(),
            name: "Not".to_string(),
            description: "Logical NOT of a boolean value".to_string(),
            inputs: vec![PortDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                data_type: "boolean".to_string(),
                required: true,
            }],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "boolean".to_string(),
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

/// Convert string to uppercase
pub struct UppercaseBlock;

impl Block for UppercaseBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.uppercase".to_string(),
            name: "Uppercase".to_string(),
            description: "Convert a string to uppercase".to_string(),
            inputs: vec![PortDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                data_type: "string".to_string(),
                required: true,
            }],
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
        let value = context
            .get_input("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::String(value.to_uppercase()));
        Ok(outputs)
    }
}

/// Convert string to lowercase
pub struct LowercaseBlock;

impl Block for LowercaseBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.lowercase".to_string(),
            name: "Lowercase".to_string(),
            description: "Convert a string to lowercase".to_string(),
            inputs: vec![PortDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                data_type: "string".to_string(),
                required: true,
            }],
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
        let value = context
            .get_input("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::String(value.to_lowercase()));
        Ok(outputs)
    }
}

/// Get length of a string
pub struct LengthBlock;

impl Block for LengthBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.length".to_string(),
            name: "Length".to_string(),
            description: "Get the length of a string".to_string(),
            inputs: vec![PortDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                data_type: "string".to_string(),
                required: true,
            }],
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
        let value = context
            .get_input("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(value.len() as f64));
        Ok(outputs)
    }
}

/// Extract substring
pub struct SubstringBlock;

impl Block for SubstringBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.substring".to_string(),
            name: "Substring".to_string(),
            description: "Extract a substring from start to end index".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "string".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "start".to_string(),
                    name: "Start".to_string(),
                    data_type: "number".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "end".to_string(),
                    name: "End".to_string(),
                    data_type: "number".to_string(),
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
        let value = context
            .get_input("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;
        let start = context
            .get_input("start")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'start'".to_string())
            })? as usize;
        let end = context
            .get_input("end")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'end'".to_string())
            })? as usize;

        // Use character-based indexing to avoid panic on multi-byte UTF-8 characters
        let char_count = value.chars().count();
        if start > end || end > char_count {
            return Err(CircuitError::InvalidInput(
                "Invalid substring range".to_string(),
            ));
        }

        let substring: String = value.chars().skip(start).take(end - start).collect();

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::String(substring));
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

    #[test]
    fn test_substring_block_ascii() {
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("Hello World".to_string()));
        context.inputs.insert("start".to_string(), Value::Float(0.0));
        context.inputs.insert("end".to_string(), Value::Float(5.0));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("Hello".to_string()))
        );
    }

    #[test]
    fn test_substring_block_utf8() {
        // Test with multi-byte UTF-8 characters to ensure we use character-based indexing
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        // Chinese characters: each character is 3 bytes, but should count as 1 char
        context
            .inputs
            .insert("value".to_string(), Value::String("你好世界".to_string()));
        context.inputs.insert("start".to_string(), Value::Float(0.0));
        context.inputs.insert("end".to_string(), Value::Float(2.0));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("你好".to_string()))
        );
    }

    #[test]
    fn test_substring_block_invalid_range() {
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("Hello".to_string()));
        context.inputs.insert("start".to_string(), Value::Float(3.0));
        context.inputs.insert("end".to_string(), Value::Float(10.0)); // end > char_count

        let result = block.execute(context);
        assert!(result.is_err());
    }
}
