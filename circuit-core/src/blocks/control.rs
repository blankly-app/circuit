use crate::block::{Block, BlockContext, BlockMetadata, PortDefinition};
use crate::error::{CircuitError, Result};
use crate::value::Value;
use std::collections::HashMap;

/// Select between two values based on a condition
pub struct IfBlock;

impl Block for IfBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "control.if".to_string(),
            name: "If".to_string(),
            description: "Select between two values based on a condition".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "condition".to_string(),
                    name: "Condition".to_string(),
                    data_type: "bool".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "then_value".to_string(),
                    name: "Then Value".to_string(),
                    data_type: "any".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "else_value".to_string(),
                    name: "Else Value".to_string(),
                    data_type: "any".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "any".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let condition = context
            .get_input("condition")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'condition'".to_string())
            })?;
        let then_value = context
            .get_input("then_value")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input 'then_value'".to_string()))?
            .clone();
        let else_value = context
            .get_input("else_value")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input 'else_value'".to_string()))?
            .clone();

        let mut outputs = HashMap::new();
        outputs.insert(
            "result".to_string(),
            if condition { then_value } else { else_value },
        );
        Ok(outputs)
    }
}

/// Select from multiple values based on a numeric selector
pub struct SwitchBlock;

impl Block for SwitchBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "control.switch".to_string(),
            name: "Switch".to_string(),
            description: "Select from multiple values based on a numeric selector".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "selector".to_string(),
                    name: "Selector".to_string(),
                    data_type: "number".to_string(),
                    required: true,
                },
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
                PortDefinition {
                    id: "default".to_string(),
                    name: "Default".to_string(),
                    data_type: "any".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "any".to_string(),
                required: true,
            }],
            config_schema: HashMap::new(),
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let selector_f = context
            .get_input("selector")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'selector'".to_string())
            })?;
        if !selector_f.is_finite() {
            return Err(CircuitError::BlockExecution(
                "Switch: selector must be finite".to_string(),
            ));
        }
        let selector = selector_f.round() as i64;
        let a = context
            .get_input("a")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input 'a'".to_string()))?
            .clone();
        let b = context
            .get_input("b")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input 'b'".to_string()))?
            .clone();
        let default = context
            .get_input("default")
            .ok_or_else(|| CircuitError::InvalidInput("Missing input 'default'".to_string()))?
            .clone();

        let selected = match selector {
            0 => a,
            1 => b,
            _ => default,
        };

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), selected);
        Ok(outputs)
    }
}

/// Pass through a value when gate is open, otherwise output Null
pub struct GateBlock;

impl Block for GateBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "control.gate".to_string(),
            name: "Gate".to_string(),
            description: "Pass through a value when gate is open, otherwise output Null"
                .to_string(),
            inputs: vec![
                PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "any".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "open".to_string(),
                    name: "Open".to_string(),
                    data_type: "bool".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
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
        let open = context
            .get_input("open")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'open'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), if open { value } else { Value::Null });
        Ok(outputs)
    }
}

/// Add a step value to the input
pub struct CounterBlock;

impl Block for CounterBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "control.counter".to_string(),
            name: "Counter".to_string(),
            description: "Add a step value to the input".to_string(),
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
            config_schema: {
                let mut schema = HashMap::new();
                schema.insert("step".to_string(), "number".to_string());
                schema
            },
        }
    }

    fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
        let value = context
            .get_input("value")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;
        let step = context
            .get_config("step")
            .and_then(|v| v.as_float())
            .unwrap_or(1.0);

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(value + step));
        Ok(outputs)
    }
}

/// Add a value to an initial value
pub struct AccumulatorBlock;

impl Block for AccumulatorBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "control.accumulator".to_string(),
            name: "Accumulator".to_string(),
            description: "Add a value to an initial value".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "number".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "initial".to_string(),
                    name: "Initial".to_string(),
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
        let initial = context
            .get_input("initial")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'initial'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Float(initial + value));
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── IfBlock tests ──────────────────────────────────────────────

    #[test]
    fn test_if_condition_true() {
        let block = IfBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("condition".to_string(), Value::Bool(true));
        context
            .inputs
            .insert("then_value".to_string(), Value::String("yes".to_string()));
        context
            .inputs
            .insert("else_value".to_string(), Value::String("no".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("yes".to_string()))
        );
    }

    #[test]
    fn test_if_condition_false() {
        let block = IfBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("condition".to_string(), Value::Bool(false));
        context
            .inputs
            .insert("then_value".to_string(), Value::String("yes".to_string()));
        context
            .inputs
            .insert("else_value".to_string(), Value::String("no".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("no".to_string())));
    }

    #[test]
    fn test_if_with_numeric_values() {
        let block = IfBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("condition".to_string(), Value::Bool(true));
        context
            .inputs
            .insert("then_value".to_string(), Value::Float(42.0));
        context
            .inputs
            .insert("else_value".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(42.0)));
    }

    #[test]
    fn test_if_missing_condition() {
        let block = IfBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("then_value".to_string(), Value::String("yes".to_string()));
        context
            .inputs
            .insert("else_value".to_string(), Value::String("no".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_if_missing_then_value() {
        let block = IfBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("condition".to_string(), Value::Bool(true));
        context
            .inputs
            .insert("else_value".to_string(), Value::String("no".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_if_missing_else_value() {
        let block = IfBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("condition".to_string(), Value::Bool(false));
        context
            .inputs
            .insert("then_value".to_string(), Value::String("yes".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_if_wrong_type_condition() {
        let block = IfBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "condition".to_string(),
            Value::String("not a bool".to_string()),
        );
        context
            .inputs
            .insert("then_value".to_string(), Value::String("yes".to_string()));
        context
            .inputs
            .insert("else_value".to_string(), Value::String("no".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── SwitchBlock tests ──────────────────────────────────────────

    #[test]
    fn test_switch_select_a() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("selector".to_string(), Value::Float(0.0));
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("first".to_string()))
        );
    }

    #[test]
    fn test_switch_select_b() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("selector".to_string(), Value::Float(1.0));
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("second".to_string()))
        );
    }

    #[test]
    fn test_switch_select_default() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("selector".to_string(), Value::Float(2.0));
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("fallback".to_string()))
        );
    }

    #[test]
    fn test_switch_negative_selector() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("selector".to_string(), Value::Float(-1.0));
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("fallback".to_string()))
        );
    }

    #[test]
    fn test_switch_fractional_selector_rounds() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("selector".to_string(), Value::Float(0.9));
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        // 0.9 rounds to 1, so should select b
        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("second".to_string()))
        );
    }

    #[test]
    fn test_switch_small_negative_rounds_to_zero() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("selector".to_string(), Value::Float(-0.1));
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        // -0.1 rounds to 0, so should select a
        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("first".to_string()))
        );
    }

    #[test]
    fn test_switch_nan_selector() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("selector".to_string(), Value::Float(f64::NAN));
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_infinity_selector() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("selector".to_string(), Value::Float(f64::INFINITY));
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_int_coercion() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("selector".to_string(), Value::Int(0));
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("first".to_string()))
        );
    }

    #[test]
    fn test_switch_missing_selector() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_switch_wrong_type() {
        let block = SwitchBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "selector".to_string(),
            Value::String("not a number".to_string()),
        );
        context
            .inputs
            .insert("a".to_string(), Value::String("first".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("second".to_string()));
        context
            .inputs
            .insert("default".to_string(), Value::String("fallback".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── GateBlock tests ────────────────────────────────────────────

    #[test]
    fn test_gate_open() {
        let block = GateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(42));
        context.inputs.insert("open".to_string(), Value::Bool(true));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Int(42)));
    }

    #[test]
    fn test_gate_closed() {
        let block = GateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(42));
        context
            .inputs
            .insert("open".to_string(), Value::Bool(false));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Null));
    }

    #[test]
    fn test_gate_open_with_string() {
        let block = GateBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));
        context.inputs.insert("open".to_string(), Value::Bool(true));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_gate_missing_value() {
        let block = GateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("open".to_string(), Value::Bool(true));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_gate_missing_open() {
        let block = GateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(42));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_gate_wrong_type_open() {
        let block = GateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(42));
        context.inputs.insert("open".to_string(), Value::Int(1));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── CounterBlock tests ─────────────────────────────────────────

    #[test]
    fn test_counter_default_step() {
        let block = CounterBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(10.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(11.0)));
    }

    #[test]
    fn test_counter_custom_step() {
        let block = CounterBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(10.0));
        context.config.insert("step".to_string(), Value::Float(5.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(15.0)));
    }

    #[test]
    fn test_counter_negative_step() {
        let block = CounterBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(10.0));
        context
            .config
            .insert("step".to_string(), Value::Float(-3.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(7.0)));
    }

    #[test]
    fn test_counter_zero() {
        let block = CounterBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(0.0));
        context.config.insert("step".to_string(), Value::Float(1.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(1.0)));
    }

    #[test]
    fn test_counter_int_coercion() {
        let block = CounterBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(10));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(11.0)));
    }

    #[test]
    fn test_counter_missing_value() {
        let block = CounterBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_counter_wrong_type() {
        let block = CounterBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::String("not a number".to_string()),
        );

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── AccumulatorBlock tests ─────────────────────────────────────

    #[test]
    fn test_accumulator_happy_path() {
        let block = AccumulatorBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));
        context
            .inputs
            .insert("initial".to_string(), Value::Float(10.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(15.0)));
    }

    #[test]
    fn test_accumulator_zero_initial() {
        let block = AccumulatorBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));
        context
            .inputs
            .insert("initial".to_string(), Value::Float(0.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn test_accumulator_negative() {
        let block = AccumulatorBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(-3.0));
        context
            .inputs
            .insert("initial".to_string(), Value::Float(10.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(7.0)));
    }

    #[test]
    fn test_accumulator_int_coercion() {
        let block = AccumulatorBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(5));
        context.inputs.insert("initial".to_string(), Value::Int(10));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Float(15.0)));
    }

    #[test]
    fn test_accumulator_missing_value() {
        let block = AccumulatorBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("initial".to_string(), Value::Float(10.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_accumulator_missing_initial() {
        let block = AccumulatorBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(5.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_accumulator_wrong_type() {
        let block = AccumulatorBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::String("not a number".to_string()),
        );
        context
            .inputs
            .insert("initial".to_string(), Value::Float(10.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }
}
