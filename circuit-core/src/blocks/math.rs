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

/// Absolute value of a number
pub struct AbsBlock;

impl Block for AbsBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.abs".to_string(),
            name: "Absolute Value".to_string(),
            description: "Absolute value of a number".to_string(),
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

/// Negate a number
pub struct NegateBlock;

impl Block for NegateBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.negate".to_string(),
            name: "Negate".to_string(),
            description: "Negate a number".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(-value));
        Ok(outputs)
    }
}

/// Raise a base to an exponent
pub struct PowerBlock;

impl Block for PowerBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.power".to_string(),
            name: "Power".to_string(),
            description: "Raise a base to an exponent".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "base".to_string(),
                    name: "Base".to_string(),
                    data_type: "number".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "exponent".to_string(),
                    name: "Exponent".to_string(),
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
        let base = context
            .get_input("base")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'base'".to_string())
            })?;
        let exponent = context
            .get_input("exponent")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'exponent'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(base.powf(exponent)));
        Ok(outputs)
    }
}

/// Square root of a number
pub struct SqrtBlock;

impl Block for SqrtBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.sqrt".to_string(),
            name: "Square Root".to_string(),
            description: "Square root of a number".to_string(),
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

        if value < 0.0 {
            return Err(CircuitError::BlockExecution(
                "Cannot take square root of negative number".to_string(),
            ));
        }

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(value.sqrt()));
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
            description: "Minimum of two numbers".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(f64::min(a, b)));
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
            description: "Maximum of two numbers".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(f64::max(a, b)));
        Ok(outputs)
    }
}

/// Clamp a value between a minimum and maximum
pub struct ClampBlock;

impl Block for ClampBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.clamp".to_string(),
            name: "Clamp".to_string(),
            description: "Clamp a value between a minimum and maximum".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "number".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "min".to_string(),
                    name: "Min".to_string(),
                    data_type: "number".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "max".to_string(),
                    name: "Max".to_string(),
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
        let value = context
            .get_input("value")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;
        let min = context
            .get_input("min")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'min'".to_string())
            })?;
        let max = context
            .get_input("max")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'max'".to_string())
            })?;

        if min > max {
            return Err(CircuitError::BlockExecution(
                "Clamp: min must be <= max".to_string(),
            ));
        }

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(value.clamp(min, max)));
        Ok(outputs)
    }
}

/// Round a number to the nearest integer
pub struct RoundBlock;

impl Block for RoundBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.round".to_string(),
            name: "Round".to_string(),
            description: "Round a number to the nearest integer".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(value.round()));
        Ok(outputs)
    }
}

/// Floor of a number (round down)
pub struct FloorBlock;

impl Block for FloorBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.floor".to_string(),
            name: "Floor".to_string(),
            description: "Floor of a number (round down)".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(value.floor()));
        Ok(outputs)
    }
}

/// Ceiling of a number (round up)
pub struct CeilBlock;

impl Block for CeilBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.ceil".to_string(),
            name: "Ceiling".to_string(),
            description: "Ceiling of a number (round up)".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(value.ceil()));
        Ok(outputs)
    }
}

/// Sine function (radians)
pub struct SinBlock;

impl Block for SinBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.sin".to_string(),
            name: "Sine".to_string(),
            description: "Sine function (radians)".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(value.sin()));
        Ok(outputs)
    }
}

/// Cosine function (radians)
pub struct CosBlock;

impl Block for CosBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.cos".to_string(),
            name: "Cosine".to_string(),
            description: "Cosine function (radians)".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(value.cos()));
        Ok(outputs)
    }
}

/// Tangent function (radians)
pub struct TanBlock;

impl Block for TanBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "math.tan".to_string(),
            name: "Tangent".to_string(),
            description: "Tangent function (radians)".to_string(),
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
        outputs.insert("result".to_string(), Value::Float(value.tan()));
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // AddBlock tests
    // ========================================================================

    #[test]
    fn test_add_happy_path() {
        let block = AddBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(8.0)));
    }

    #[test]
    fn test_add_int_coercion() {
        let block = AddBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Int(5));
        context.inputs.insert("b".to_string(), Value::Int(3));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(8.0)));
    }

    #[test]
    fn test_add_missing_input() {
        let block = AddBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_wrong_type() {
        let block = AddBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("a".to_string(), Value::String("hello".to_string()));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_negative_numbers() {
        let block = AddBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(-5.0));
        context.inputs.insert("b".to_string(), Value::Float(-3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-8.0)));
    }

    #[test]
    fn test_add_zeros() {
        let block = AddBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(0.0));
        context.inputs.insert("b".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    // ========================================================================
    // SubtractBlock tests
    // ========================================================================

    #[test]
    fn test_subtract_happy_path() {
        let block = SubtractBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(7.0)));
    }

    #[test]
    fn test_subtract_int_coercion() {
        let block = SubtractBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Int(10));
        context.inputs.insert("b".to_string(), Value::Int(3));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(7.0)));
    }

    #[test]
    fn test_subtract_missing_input() {
        let block = SubtractBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_subtract_wrong_type() {
        let block = SubtractBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context
            .inputs
            .insert("b".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_subtract_negative_result() {
        let block = SubtractBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(3.0));
        context.inputs.insert("b".to_string(), Value::Float(10.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-7.0)));
    }

    // ========================================================================
    // MultiplyBlock tests
    // ========================================================================

    #[test]
    fn test_multiply_happy_path() {
        let block = MultiplyBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(15.0)));
    }

    #[test]
    fn test_multiply_int_coercion() {
        let block = MultiplyBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Int(5));
        context.inputs.insert("b".to_string(), Value::Int(3));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(15.0)));
    }

    #[test]
    fn test_multiply_missing_input() {
        let block = MultiplyBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiply_wrong_type() {
        let block = MultiplyBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("a".to_string(), Value::String("hello".to_string()));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiply_by_zero() {
        let block = MultiplyBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    #[test]
    fn test_multiply_negative() {
        let block = MultiplyBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(-5.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-15.0)));
    }

    // ========================================================================
    // DivideBlock tests
    // ========================================================================

    #[test]
    fn test_divide_happy_path() {
        let block = DivideBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context.inputs.insert("b".to_string(), Value::Float(2.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_divide_int_coercion() {
        let block = DivideBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Int(10));
        context.inputs.insert("b".to_string(), Value::Int(2));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_divide_missing_input() {
        let block = DivideBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_divide_wrong_type() {
        let block = DivideBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context
            .inputs
            .insert("b".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
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
    fn test_divide_fractional_result() {
        let block = DivideBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(7.0));
        context.inputs.insert("b".to_string(), Value::Float(2.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(3.5)));
    }

    // ========================================================================
    // ModuloBlock tests
    // ========================================================================

    #[test]
    fn test_modulo_happy_path() {
        let block = ModuloBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(1.0)));
    }

    #[test]
    fn test_modulo_int_coercion() {
        let block = ModuloBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Int(10));
        context.inputs.insert("b".to_string(), Value::Int(3));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(1.0)));
    }

    #[test]
    fn test_modulo_missing_input() {
        let block = ModuloBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_wrong_type() {
        let block = ModuloBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("a".to_string(), Value::String("hello".to_string()));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_by_zero() {
        let block = ModuloBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(10.0));
        context.inputs.insert("b".to_string(), Value::Float(0.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_even_division() {
        let block = ModuloBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(9.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    // ========================================================================
    // AbsBlock tests
    // ========================================================================

    #[test]
    fn test_abs_happy_path() {
        let block = AbsBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_abs_int_coercion() {
        let block = AbsBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(-5));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_abs_missing_input() {
        let block = AbsBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_abs_wrong_type() {
        let block = AbsBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_abs_negative() {
        let block = AbsBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(-42.5));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(42.5)));
    }

    #[test]
    fn test_abs_zero() {
        let block = AbsBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    // ========================================================================
    // NegateBlock tests
    // ========================================================================

    #[test]
    fn test_negate_happy_path() {
        let block = NegateBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-5.0)));
    }

    #[test]
    fn test_negate_int_coercion() {
        let block = NegateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(5));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-5.0)));
    }

    #[test]
    fn test_negate_missing_input() {
        let block = NegateBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_negate_wrong_type() {
        let block = NegateBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_negate_negative_becomes_positive() {
        let block = NegateBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(-7.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(7.0)));
    }

    #[test]
    fn test_negate_zero() {
        let block = NegateBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        // -0.0 == 0.0 in floating point
        assert_eq!(result.get("result"), Some(&Value::Float(-0.0)));
    }

    // ========================================================================
    // PowerBlock tests
    // ========================================================================

    #[test]
    fn test_power_happy_path() {
        let block = PowerBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("base".to_string(), Value::Float(2.0));
        context
            .inputs
            .insert("exponent".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(8.0)));
    }

    #[test]
    fn test_power_int_coercion() {
        let block = PowerBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("base".to_string(), Value::Int(2));
        context.inputs.insert("exponent".to_string(), Value::Int(3));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(8.0)));
    }

    #[test]
    fn test_power_missing_input() {
        let block = PowerBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("base".to_string(), Value::Float(2.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_power_wrong_type() {
        let block = PowerBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("base".to_string(), Value::String("hello".to_string()));
        context
            .inputs
            .insert("exponent".to_string(), Value::Float(3.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_power_zero_exponent() {
        let block = PowerBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("base".to_string(), Value::Float(5.0));
        context
            .inputs
            .insert("exponent".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(1.0)));
    }

    #[test]
    fn test_power_negative_exponent() {
        let block = PowerBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("base".to_string(), Value::Float(2.0));
        context
            .inputs
            .insert("exponent".to_string(), Value::Float(-1.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.5)));
    }

    #[test]
    fn test_power_fractional_exponent() {
        let block = PowerBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("base".to_string(), Value::Float(4.0));
        context
            .inputs
            .insert("exponent".to_string(), Value::Float(0.5));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(2.0)));
    }

    // ========================================================================
    // SqrtBlock tests
    // ========================================================================

    #[test]
    fn test_sqrt_happy_path() {
        let block = SqrtBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(25.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_sqrt_int_coercion() {
        let block = SqrtBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(16));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(4.0)));
    }

    #[test]
    fn test_sqrt_missing_input() {
        let block = SqrtBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_sqrt_wrong_type() {
        let block = SqrtBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_sqrt_negative() {
        let block = SqrtBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(-4.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_sqrt_zero() {
        let block = SqrtBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    // ========================================================================
    // MinBlock tests
    // ========================================================================

    #[test]
    fn test_min_happy_path() {
        let block = MinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn test_min_int_coercion() {
        let block = MinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Int(5));
        context.inputs.insert("b".to_string(), Value::Int(3));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn test_min_missing_input() {
        let block = MinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_min_wrong_type() {
        let block = MinBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("a".to_string(), Value::String("hello".to_string()));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_min_equal_values() {
        let block = MinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(5.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_min_negative_numbers() {
        let block = MinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(-5.0));
        context.inputs.insert("b".to_string(), Value::Float(-3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-5.0)));
    }

    // ========================================================================
    // MaxBlock tests
    // ========================================================================

    #[test]
    fn test_max_happy_path() {
        let block = MaxBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_max_int_coercion() {
        let block = MaxBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Int(5));
        context.inputs.insert("b".to_string(), Value::Int(3));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_max_missing_input() {
        let block = MaxBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_max_wrong_type() {
        let block = MaxBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context
            .inputs
            .insert("b".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_max_equal_values() {
        let block = MaxBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(5.0));
        context.inputs.insert("b".to_string(), Value::Float(5.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_max_negative_numbers() {
        let block = MaxBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Float(-5.0));
        context.inputs.insert("b".to_string(), Value::Float(-3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-3.0)));
    }

    // ========================================================================
    // ClampBlock tests
    // ========================================================================

    #[test]
    fn test_clamp_happy_path() {
        let block = ClampBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));
        context.inputs.insert("min".to_string(), Value::Float(0.0));
        context.inputs.insert("max".to_string(), Value::Float(10.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_clamp_int_coercion() {
        let block = ClampBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(5));
        context.inputs.insert("min".to_string(), Value::Int(0));
        context.inputs.insert("max".to_string(), Value::Int(10));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_clamp_missing_input() {
        let block = ClampBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));
        context.inputs.insert("min".to_string(), Value::Float(0.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_clamp_wrong_type() {
        let block = ClampBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));
        context.inputs.insert("min".to_string(), Value::Float(0.0));
        context.inputs.insert("max".to_string(), Value::Float(10.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_clamp_below_min() {
        let block = ClampBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(-5.0));
        context.inputs.insert("min".to_string(), Value::Float(0.0));
        context.inputs.insert("max".to_string(), Value::Float(10.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    #[test]
    fn test_clamp_above_max() {
        let block = ClampBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(15.0));
        context.inputs.insert("min".to_string(), Value::Float(0.0));
        context.inputs.insert("max".to_string(), Value::Float(10.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(10.0)));
    }

    #[test]
    fn test_clamp_min_greater_than_max() {
        let block = ClampBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));
        context.inputs.insert("min".to_string(), Value::Float(10.0));
        context.inputs.insert("max".to_string(), Value::Float(0.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_clamp_equal_min_max() {
        let block = ClampBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));
        context.inputs.insert("min".to_string(), Value::Float(3.0));
        context.inputs.insert("max".to_string(), Value::Float(3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(3.0)));
    }

    // ========================================================================
    // RoundBlock tests
    // ========================================================================

    #[test]
    fn test_round_happy_path() {
        let block = RoundBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(3.7));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(4.0)));
    }

    #[test]
    fn test_round_int_coercion() {
        let block = RoundBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(5));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_round_missing_input() {
        let block = RoundBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_round_wrong_type() {
        let block = RoundBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_round_down() {
        let block = RoundBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(3.2));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn test_round_half() {
        let block = RoundBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(2.5));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn test_round_negative() {
        let block = RoundBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(-3.7));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-4.0)));
    }

    // ========================================================================
    // FloorBlock tests
    // ========================================================================

    #[test]
    fn test_floor_happy_path() {
        let block = FloorBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(3.7));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn test_floor_int_coercion() {
        let block = FloorBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(5));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_floor_missing_input() {
        let block = FloorBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_floor_wrong_type() {
        let block = FloorBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_floor_already_integer() {
        let block = FloorBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_floor_negative() {
        let block = FloorBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(-3.2));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-4.0)));
    }

    // ========================================================================
    // CeilBlock tests
    // ========================================================================

    #[test]
    fn test_ceil_happy_path() {
        let block = CeilBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(3.2));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(4.0)));
    }

    #[test]
    fn test_ceil_int_coercion() {
        let block = CeilBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(5));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_ceil_missing_input() {
        let block = CeilBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_ceil_wrong_type() {
        let block = CeilBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_ceil_already_integer() {
        let block = CeilBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_ceil_negative() {
        let block = CeilBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(-3.7));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(-3.0)));
    }

    // ========================================================================
    // SinBlock tests
    // ========================================================================

    #[test]
    fn test_sin_happy_path() {
        let block = SinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::Float(std::f64::consts::FRAC_PI_2),
        );

        let result = block.execute(context).unwrap();
        let value = result.get("result").unwrap();
        if let Value::Float(f) = value {
            assert!((f - 1.0).abs() < 1e-10);
        } else {
            panic!("Expected Float value");
        }
    }

    #[test]
    fn test_sin_int_coercion() {
        let block = SinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    #[test]
    fn test_sin_missing_input() {
        let block = SinBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_sin_wrong_type() {
        let block = SinBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_sin_zero() {
        let block = SinBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    #[test]
    fn test_sin_pi() {
        let block = SinBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(std::f64::consts::PI));

        let result = block.execute(context).unwrap();
        let value = result.get("result").unwrap();
        if let Value::Float(f) = value {
            assert!(f.abs() < 1e-10);
        } else {
            panic!("Expected Float value");
        }
    }

    // ========================================================================
    // CosBlock tests
    // ========================================================================

    #[test]
    fn test_cos_happy_path() {
        let block = CosBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(1.0)));
    }

    #[test]
    fn test_cos_int_coercion() {
        let block = CosBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(1.0)));
    }

    #[test]
    fn test_cos_missing_input() {
        let block = CosBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_cos_wrong_type() {
        let block = CosBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_cos_pi() {
        let block = CosBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(std::f64::consts::PI));

        let result = block.execute(context).unwrap();
        let value = result.get("result").unwrap();
        if let Value::Float(f) = value {
            assert!((f - (-1.0)).abs() < 1e-10);
        } else {
            panic!("Expected Float value");
        }
    }

    #[test]
    fn test_cos_half_pi() {
        let block = CosBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::Float(std::f64::consts::FRAC_PI_2),
        );

        let result = block.execute(context).unwrap();
        let value = result.get("result").unwrap();
        if let Value::Float(f) = value {
            assert!(f.abs() < 1e-10);
        } else {
            panic!("Expected Float value");
        }
    }

    // ========================================================================
    // TanBlock tests
    // ========================================================================

    #[test]
    fn test_tan_happy_path() {
        let block = TanBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::Float(std::f64::consts::FRAC_PI_4),
        );

        let result = block.execute(context).unwrap();
        let value = result.get("result").unwrap();
        if let Value::Float(f) = value {
            assert!((f - 1.0).abs() < 1e-10);
        } else {
            panic!("Expected Float value");
        }
    }

    #[test]
    fn test_tan_int_coercion() {
        let block = TanBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    #[test]
    fn test_tan_missing_input() {
        let block = TanBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_tan_wrong_type() {
        let block = TanBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_tan_zero() {
        let block = TanBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(0.0)));
    }

    #[test]
    fn test_tan_pi() {
        let block = TanBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(std::f64::consts::PI));

        let result = block.execute(context).unwrap();
        let value = result.get("result").unwrap();
        if let Value::Float(f) = value {
            assert!(f.abs() < 1e-10);
        } else {
            panic!("Expected Float value");
        }
    }

    // ========================================================================
    // Metadata tests
    // ========================================================================

    #[test]
    fn test_metadata_ids() {
        assert_eq!(AddBlock.metadata().id, "math.add");
        assert_eq!(SubtractBlock.metadata().id, "math.subtract");
        assert_eq!(MultiplyBlock.metadata().id, "math.multiply");
        assert_eq!(DivideBlock.metadata().id, "math.divide");
        assert_eq!(ModuloBlock.metadata().id, "math.modulo");
        assert_eq!(AbsBlock.metadata().id, "math.abs");
        assert_eq!(NegateBlock.metadata().id, "math.negate");
        assert_eq!(PowerBlock.metadata().id, "math.power");
        assert_eq!(SqrtBlock.metadata().id, "math.sqrt");
        assert_eq!(MinBlock.metadata().id, "math.min");
        assert_eq!(MaxBlock.metadata().id, "math.max");
        assert_eq!(ClampBlock.metadata().id, "math.clamp");
        assert_eq!(RoundBlock.metadata().id, "math.round");
        assert_eq!(FloorBlock.metadata().id, "math.floor");
        assert_eq!(CeilBlock.metadata().id, "math.ceil");
        assert_eq!(SinBlock.metadata().id, "math.sin");
        assert_eq!(CosBlock.metadata().id, "math.cos");
        assert_eq!(TanBlock.metadata().id, "math.tan");
    }

    #[test]
    fn test_metadata_names() {
        assert_eq!(AbsBlock.metadata().name, "Absolute Value");
        assert_eq!(NegateBlock.metadata().name, "Negate");
        assert_eq!(PowerBlock.metadata().name, "Power");
        assert_eq!(SqrtBlock.metadata().name, "Square Root");
        assert_eq!(MinBlock.metadata().name, "Minimum");
        assert_eq!(MaxBlock.metadata().name, "Maximum");
        assert_eq!(ClampBlock.metadata().name, "Clamp");
        assert_eq!(RoundBlock.metadata().name, "Round");
        assert_eq!(FloorBlock.metadata().name, "Floor");
        assert_eq!(CeilBlock.metadata().name, "Ceiling");
        assert_eq!(SinBlock.metadata().name, "Sine");
        assert_eq!(CosBlock.metadata().name, "Cosine");
        assert_eq!(TanBlock.metadata().name, "Tangent");
    }

    #[test]
    fn test_trig_descriptions() {
        assert_eq!(SinBlock.metadata().description, "Sine function (radians)");
        assert_eq!(CosBlock.metadata().description, "Cosine function (radians)");
        assert_eq!(
            TanBlock.metadata().description,
            "Tangent function (radians)"
        );
    }
}
