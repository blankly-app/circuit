use crate::block::{Block, BlockContext, BlockMetadata, PortDefinition};
use crate::error::{CircuitError, Result};
use crate::value::Value;
use std::collections::HashMap;

/// Convert a Value to its display string representation
fn value_to_display(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_display).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(_) => "[object]".to_string(),
        Value::Bytes(b) => format!("[{} bytes]", b.len()),
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

/// Get the length of a string
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
        outputs.insert("result".to_string(), Value::Int(value.len() as i64));
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
            description: "Convert string to uppercase".to_string(),
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
            description: "Convert string to lowercase".to_string(),
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

/// Remove leading and trailing whitespace
pub struct TrimBlock;

impl Block for TrimBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.trim".to_string(),
            name: "Trim".to_string(),
            description: "Remove leading and trailing whitespace".to_string(),
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
        outputs.insert(
            "result".to_string(),
            Value::String(value.trim().to_string()),
        );
        Ok(outputs)
    }
}

/// Check if a string contains a substring
pub struct ContainsBlock;

impl Block for ContainsBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.contains".to_string(),
            name: "Contains".to_string(),
            description: "Check if a string contains a substring".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "string".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "search".to_string(),
                    name: "Search".to_string(),
                    data_type: "string".to_string(),
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
        let value = context
            .get_input("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;
        let search = context
            .get_input("search")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'search'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Bool(value.contains(search)));
        Ok(outputs)
    }
}

/// Replace occurrences of a pattern in a string
pub struct ReplaceBlock;

impl Block for ReplaceBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.replace".to_string(),
            name: "Replace".to_string(),
            description: "Replace occurrences of a pattern in a string".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "string".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "pattern".to_string(),
                    name: "Pattern".to_string(),
                    data_type: "string".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "replacement".to_string(),
                    name: "Replacement".to_string(),
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
        let value = context
            .get_input("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;
        let pattern = context
            .get_input("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'pattern'".to_string())
            })?;
        let replacement = context
            .get_input("replacement")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'replacement'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert(
            "result".to_string(),
            Value::String(value.replace(pattern, replacement)),
        );
        Ok(outputs)
    }
}

/// Split a string by a delimiter
pub struct SplitBlock;

impl Block for SplitBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.split".to_string(),
            name: "Split".to_string(),
            description: "Split a string by a delimiter".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "string".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "delimiter".to_string(),
                    name: "Delimiter".to_string(),
                    data_type: "string".to_string(),
                    required: true,
                },
            ],
            outputs: vec![PortDefinition {
                id: "result".to_string(),
                name: "Result".to_string(),
                data_type: "array".to_string(),
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
        let delimiter = context
            .get_input("delimiter")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'delimiter'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert(
            "result".to_string(),
            Value::Array(
                value
                    .split(delimiter)
                    .map(|s| Value::String(s.to_string()))
                    .collect(),
            ),
        );
        Ok(outputs)
    }
}

/// Join an array into a string with a delimiter
pub struct JoinBlock;

impl Block for JoinBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.join".to_string(),
            name: "Join".to_string(),
            description: "Join an array into a string with a delimiter".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "array".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "delimiter".to_string(),
                    name: "Delimiter".to_string(),
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
        let arr = context
            .get_input("value")
            .and_then(|v| v.as_array())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?;
        let delimiter = context
            .get_input("delimiter")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'delimiter'".to_string())
            })?;

        let mut outputs = HashMap::new();
        outputs.insert(
            "result".to_string(),
            Value::String(
                arr.iter()
                    .map(value_to_display)
                    .collect::<Vec<_>>()
                    .join(delimiter),
            ),
        );
        Ok(outputs)
    }
}

/// Extract a substring by character position
pub struct SubstringBlock;

impl Block for SubstringBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.substring".to_string(),
            name: "Substring".to_string(),
            description: "Extract a substring by character position".to_string(),
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
            })?;
        let end = context
            .get_input("end")
            .and_then(|v| v.as_float())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'end'".to_string())
            })?;

        let chars: Vec<char> = value.chars().collect();
        let start = (start as usize).min(chars.len());
        let end = (end as usize).min(chars.len());

        let result = if start > end {
            String::new()
        } else {
            chars[start..end].iter().collect()
        };

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::String(result));
        Ok(outputs)
    }
}

/// Replace {} placeholders with a value
pub struct TemplateBlock;

impl Block for TemplateBlock {
    fn metadata(&self) -> BlockMetadata {
        BlockMetadata {
            id: "string.template".to_string(),
            name: "Template".to_string(),
            description: "Replace {} placeholders with a value".to_string(),
            inputs: vec![
                PortDefinition {
                    id: "template".to_string(),
                    name: "Template".to_string(),
                    data_type: "string".to_string(),
                    required: true,
                },
                PortDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    data_type: "any".to_string(),
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
        let template = context
            .get_input("template")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'template'".to_string())
            })?;
        let value = context
            .get_input("value")
            .ok_or_else(|| {
                CircuitError::InvalidInput("Missing or invalid input 'value'".to_string())
            })?
            .clone();

        let mut outputs = HashMap::new();
        outputs.insert(
            "result".to_string(),
            Value::String(template.replace("{}", &value_to_display(&value))),
        );
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── ConcatBlock ──────────────────────────────────────────────────────

    #[test]
    fn test_concat_happy_path() {
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

    #[test]
    fn test_concat_empty_strings() {
        let block = ConcatBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("a".to_string(), Value::String("".to_string()));
        context
            .inputs
            .insert("b".to_string(), Value::String("".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("".to_string())));
    }

    #[test]
    fn test_concat_missing_input() {
        let block = ConcatBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("a".to_string(), Value::String("Hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_concat_wrong_type() {
        let block = ConcatBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("a".to_string(), Value::Int(42));
        context
            .inputs
            .insert("b".to_string(), Value::String("World".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── LengthBlock ─────────────────────────────────────────────────────

    #[test]
    fn test_length_happy_path() {
        let block = LengthBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Int(5)));
    }

    #[test]
    fn test_length_empty_string() {
        let block = LengthBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Int(0)));
    }

    #[test]
    fn test_length_unicode() {
        let block = LengthBlock;
        let mut context = BlockContext::new();
        // "cafe\u0301" is 5 bytes in UTF-8, str::len() returns byte length
        context.inputs.insert(
            "value".to_string(),
            Value::String("caf\u{00e9}".to_string()),
        );

        let result = block.execute(context).unwrap();
        // "caf\u{00e9}" is 5 bytes (c=1, a=1, f=1, \u{00e9}=2)
        assert_eq!(result.get("result"), Some(&Value::Int(5)));
    }

    #[test]
    fn test_length_missing_input() {
        let block = LengthBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_length_wrong_type() {
        let block = LengthBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(123));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── UppercaseBlock ──────────────────────────────────────────────────

    #[test]
    fn test_uppercase_happy_path() {
        let block = UppercaseBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("HELLO".to_string()))
        );
    }

    #[test]
    fn test_uppercase_already_uppercase() {
        let block = UppercaseBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("HELLO".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("HELLO".to_string()))
        );
    }

    #[test]
    fn test_uppercase_empty() {
        let block = UppercaseBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("".to_string())));
    }

    #[test]
    fn test_uppercase_unicode() {
        let block = UppercaseBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::String("caf\u{00e9}".to_string()),
        );

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("CAF\u{00c9}".to_string()))
        );
    }

    #[test]
    fn test_uppercase_missing_input() {
        let block = UppercaseBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_uppercase_wrong_type() {
        let block = UppercaseBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Bool(true));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── LowercaseBlock ─────────────────────────────────────────────────

    #[test]
    fn test_lowercase_happy_path() {
        let block = LowercaseBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("HELLO".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_lowercase_already_lowercase() {
        let block = LowercaseBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_lowercase_empty() {
        let block = LowercaseBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("".to_string())));
    }

    #[test]
    fn test_lowercase_missing_input() {
        let block = LowercaseBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_lowercase_wrong_type() {
        let block = LowercaseBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Float(99.9));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── TrimBlock ───────────────────────────────────────────────────────

    #[test]
    fn test_trim_both_sides() {
        let block = TrimBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("  hello  ".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_trim_no_whitespace() {
        let block = TrimBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_trim_only_leading() {
        let block = TrimBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("  hello".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_trim_only_trailing() {
        let block = TrimBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello  ".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_trim_missing_input() {
        let block = TrimBlock;
        let context = BlockContext::new();

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_trim_wrong_type() {
        let block = TrimBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(42));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── ContainsBlock ───────────────────────────────────────────────────

    #[test]
    fn test_contains_found() {
        let block = ContainsBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::String("hello world".to_string()),
        );
        context
            .inputs
            .insert("search".to_string(), Value::String("world".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_contains_not_found() {
        let block = ContainsBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::String("hello world".to_string()),
        );
        context
            .inputs
            .insert("search".to_string(), Value::String("xyz".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_contains_empty_search() {
        let block = ContainsBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));
        context
            .inputs
            .insert("search".to_string(), Value::String("".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_contains_empty_value() {
        let block = ContainsBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("".to_string()));
        context
            .inputs
            .insert("search".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_contains_missing_input() {
        let block = ContainsBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_wrong_type() {
        let block = ContainsBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(42));
        context
            .inputs
            .insert("search".to_string(), Value::String("4".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── ReplaceBlock ────────────────────────────────────────────────────

    #[test]
    fn test_replace_single() {
        let block = ReplaceBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::String("hello world".to_string()),
        );
        context
            .inputs
            .insert("pattern".to_string(), Value::String("world".to_string()));
        context
            .inputs
            .insert("replacement".to_string(), Value::String("rust".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello rust".to_string()))
        );
    }

    #[test]
    fn test_replace_multiple() {
        let block = ReplaceBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("aaa".to_string()));
        context
            .inputs
            .insert("pattern".to_string(), Value::String("a".to_string()));
        context
            .inputs
            .insert("replacement".to_string(), Value::String("b".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("bbb".to_string()))
        );
    }

    #[test]
    fn test_replace_no_match() {
        let block = ReplaceBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));
        context
            .inputs
            .insert("pattern".to_string(), Value::String("xyz".to_string()));
        context
            .inputs
            .insert("replacement".to_string(), Value::String("abc".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_replace_empty_pattern() {
        let block = ReplaceBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hi".to_string()));
        context
            .inputs
            .insert("pattern".to_string(), Value::String("".to_string()));
        context
            .inputs
            .insert("replacement".to_string(), Value::String("-".to_string()));

        // Rust's str::replace with empty pattern inserts between each char and at boundaries
        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("-h-i-".to_string()))
        );
    }

    #[test]
    fn test_replace_missing_input() {
        let block = ReplaceBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));
        context
            .inputs
            .insert("pattern".to_string(), Value::String("l".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_replace_wrong_type() {
        let block = ReplaceBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(42));
        context
            .inputs
            .insert("pattern".to_string(), Value::String("4".to_string()));
        context
            .inputs
            .insert("replacement".to_string(), Value::String("x".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── SplitBlock ──────────────────────────────────────────────────────

    #[test]
    fn test_split_happy_path() {
        let block = SplitBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("a,b,c".to_string()));
        context
            .inputs
            .insert("delimiter".to_string(), Value::String(",".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string()),
            ]))
        );
    }

    #[test]
    fn test_split_delimiter_not_found() {
        let block = SplitBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));
        context
            .inputs
            .insert("delimiter".to_string(), Value::String(",".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::Array(vec![Value::String("hello".to_string())]))
        );
    }

    #[test]
    fn test_split_empty_delimiter() {
        let block = SplitBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hi".to_string()));
        context
            .inputs
            .insert("delimiter".to_string(), Value::String("".to_string()));

        // Rust's str::split with empty string splits between each char
        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::Array(vec![
                Value::String("".to_string()),
                Value::String("h".to_string()),
                Value::String("i".to_string()),
                Value::String("".to_string()),
            ]))
        );
    }

    #[test]
    fn test_split_missing_input() {
        let block = SplitBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_split_wrong_type() {
        let block = SplitBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(123));
        context
            .inputs
            .insert("delimiter".to_string(), Value::String(",".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── JoinBlock ───────────────────────────────────────────────────────

    #[test]
    fn test_join_happy_path() {
        let block = JoinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string()),
            ]),
        );
        context
            .inputs
            .insert("delimiter".to_string(), Value::String(", ".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("a, b, c".to_string()))
        );
    }

    #[test]
    fn test_join_single_element() {
        let block = JoinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::Array(vec![Value::String("only".to_string())]),
        );
        context
            .inputs
            .insert("delimiter".to_string(), Value::String(",".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("only".to_string()))
        );
    }

    #[test]
    fn test_join_empty_array() {
        let block = JoinBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::Array(vec![]));
        context
            .inputs
            .insert("delimiter".to_string(), Value::String(",".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("".to_string())));
    }

    #[test]
    fn test_join_non_string_elements() {
        let block = JoinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::Array(vec![Value::Int(1), Value::Bool(true), Value::Null]),
        );
        context
            .inputs
            .insert("delimiter".to_string(), Value::String("-".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("1-true-null".to_string()))
        );
    }

    #[test]
    fn test_join_missing_input() {
        let block = JoinBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("delimiter".to_string(), Value::String(",".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_join_wrong_type() {
        let block = JoinBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::String("not an array".to_string()),
        );
        context
            .inputs
            .insert("delimiter".to_string(), Value::String(",".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── SubstringBlock ──────────────────────────────────────────────────

    #[test]
    fn test_substring_happy_path() {
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "value".to_string(),
            Value::String("hello world".to_string()),
        );
        context
            .inputs
            .insert("start".to_string(), Value::Float(0.0));
        context.inputs.insert("end".to_string(), Value::Float(5.0));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_substring_start_zero() {
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("abc".to_string()));
        context.inputs.insert("start".to_string(), Value::Int(0));
        context.inputs.insert("end".to_string(), Value::Int(2));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("ab".to_string())));
    }

    #[test]
    fn test_substring_end_beyond_length() {
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hi".to_string()));
        context
            .inputs
            .insert("start".to_string(), Value::Float(0.0));
        context
            .inputs
            .insert("end".to_string(), Value::Float(100.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("hi".to_string())));
    }

    #[test]
    fn test_substring_start_beyond_length() {
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hi".to_string()));
        context
            .inputs
            .insert("start".to_string(), Value::Float(50.0));
        context
            .inputs
            .insert("end".to_string(), Value::Float(100.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("".to_string())));
    }

    #[test]
    fn test_substring_start_greater_than_end() {
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));
        context
            .inputs
            .insert("start".to_string(), Value::Float(3.0));
        context.inputs.insert("end".to_string(), Value::Float(1.0));

        let result = block.execute(context).unwrap();
        assert_eq!(result.get("result"), Some(&Value::String("".to_string())));
    }

    #[test]
    fn test_substring_missing_input() {
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));
        context
            .inputs
            .insert("start".to_string(), Value::Float(0.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_substring_wrong_type() {
        let block = SubstringBlock;
        let mut context = BlockContext::new();
        context.inputs.insert("value".to_string(), Value::Int(123));
        context
            .inputs
            .insert("start".to_string(), Value::Float(0.0));
        context.inputs.insert("end".to_string(), Value::Float(1.0));

        let result = block.execute(context);
        assert!(result.is_err());
    }

    // ── TemplateBlock ───────────────────────────────────────────────────

    #[test]
    fn test_template_single_placeholder() {
        let block = TemplateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "template".to_string(),
            Value::String("Hello, {}!".to_string()),
        );
        context
            .inputs
            .insert("value".to_string(), Value::String("world".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("Hello, world!".to_string()))
        );
    }

    #[test]
    fn test_template_no_placeholder() {
        let block = TemplateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "template".to_string(),
            Value::String("No placeholders here".to_string()),
        );
        context
            .inputs
            .insert("value".to_string(), Value::String("ignored".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("No placeholders here".to_string()))
        );
    }

    #[test]
    fn test_template_multiple_placeholders() {
        let block = TemplateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "template".to_string(),
            Value::String("{} and {}".to_string()),
        );
        context
            .inputs
            .insert("value".to_string(), Value::String("x".to_string()));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("x and x".to_string()))
        );
    }

    #[test]
    fn test_template_numeric_value() {
        let block = TemplateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "template".to_string(),
            Value::String("Count: {}".to_string()),
        );
        context.inputs.insert("value".to_string(), Value::Int(42));

        let result = block.execute(context).unwrap();
        assert_eq!(
            result.get("result"),
            Some(&Value::String("Count: 42".to_string()))
        );
    }

    #[test]
    fn test_template_missing_input() {
        let block = TemplateBlock;
        let mut context = BlockContext::new();
        context.inputs.insert(
            "template".to_string(),
            Value::String("Hello, {}!".to_string()),
        );

        let result = block.execute(context);
        assert!(result.is_err());
    }

    #[test]
    fn test_template_wrong_type_for_template() {
        let block = TemplateBlock;
        let mut context = BlockContext::new();
        context
            .inputs
            .insert("template".to_string(), Value::Int(42));
        context
            .inputs
            .insert("value".to_string(), Value::String("hello".to_string()));

        let result = block.execute(context);
        assert!(result.is_err());
    }
}
