use circuit_lang::*;
use pretty_assertions::assert_eq;

#[test]
fn test_multiline_comment() {
    let source = r#"
        /* This is a multiline comment
           spanning multiple lines */
        block test.multiline {
            /* Another comment */
            output value: Number
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse block with multiline comment");
    assert_eq!(block_def.name, "test.multiline");
}

#[test]
fn test_nested_multiline_comment() {
    let source = r#"
        block test.nested {
            /* Outer comment
               /* This looks nested but isn't in Pest */
               Still part of comment */
            output value: Number
        }
    "#;

    // This should fail because Pest doesn't support nested comments by default
    let result = parse_block(source);
    // It actually works because /* inside comment is just text
    assert!(result.is_ok());
}

#[test]
fn test_escape_sequences_in_strings() {
    let source = r#"
        flow test_escapes {
            node n1: core.constant {
                newline = "line1\nline2"
                tab = "col1\tcol2"
                quote = "say \"hello\""
                backslash = "path\\to\\file"
            }
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse flow with escape sequences");
    let config = &flow_def.nodes[0].config;

    assert_eq!(config.get("newline"), Some(&Value::String("line1\nline2".to_string())));
    assert_eq!(config.get("tab"), Some(&Value::String("col1\tcol2".to_string())));
    assert_eq!(config.get("quote"), Some(&Value::String("say \"hello\"".to_string())));
    assert_eq!(config.get("backslash"), Some(&Value::String("path\\to\\file".to_string())));
}

#[test]
fn test_empty_block() {
    let source = r#"
        block test.empty {
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse empty block");
    assert_eq!(block_def.name, "test.empty");
    assert!(block_def.inputs.is_empty());
    assert!(block_def.outputs.is_empty());
    assert!(block_def.config.is_empty());
    assert!(block_def.execute.is_none());
}

#[test]
fn test_empty_flow() {
    let source = r#"
        flow empty {
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse empty flow");
    assert_eq!(flow_def.name, "empty");
    assert!(flow_def.nodes.is_empty());
    assert!(flow_def.connections.is_empty());
}

#[test]
fn test_deeply_nested_qualified_name() {
    let source = r#"
        block very.deeply.nested.namespace.block.name {
            output value: Number
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse deeply nested name");
    assert_eq!(block_def.name, "very.deeply.nested.namespace.block.name");
}

#[test]
fn test_negative_numbers() {
    let source = r#"
        flow negatives {
            node n1: core.constant {
                neg_int = -42
                neg_float = -3.14159
                neg_zero = -0
            }
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse negative numbers");
    let config = &flow_def.nodes[0].config;

    assert_eq!(config.get("neg_int"), Some(&Value::Number(-42.0)));
    assert_eq!(config.get("neg_float"), Some(&Value::Number(-3.14159)));
    assert_eq!(config.get("neg_zero"), Some(&Value::Number(-0.0)));
}

#[test]
fn test_empty_arrays_and_objects() {
    let source = r#"
        flow empty_collections {
            node n1: core.constant {
                empty_array = []
                empty_object = {}
            }
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse empty collections");
    let config = &flow_def.nodes[0].config;

    assert_eq!(config.get("empty_array"), Some(&Value::Array(vec![])));
    assert_eq!(config.get("empty_object"), Some(&Value::Object(std::collections::HashMap::new())));
}

#[test]
fn test_nested_arrays() {
    let source = r#"
        flow nested_arrays {
            node n1: core.constant {
                nested = [[1, 2], [3, 4], [5, 6]]
            }
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse nested arrays");
    let config = &flow_def.nodes[0].config;

    let expected = Value::Array(vec![
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::Array(vec![Value::Number(3.0), Value::Number(4.0)]),
        Value::Array(vec![Value::Number(5.0), Value::Number(6.0)]),
    ]);
    assert_eq!(config.get("nested"), Some(&expected));
}

#[test]
fn test_nested_objects() {
    let source = r#"
        flow nested_objects {
            node n1: core.constant {
                nested = {"outer": {"inner": "value"}}
            }
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse nested objects");
    let config = &flow_def.nodes[0].config;

    let mut inner = std::collections::HashMap::new();
    inner.insert("inner".to_string(), Value::String("value".to_string()));

    let mut outer = std::collections::HashMap::new();
    outer.insert("outer".to_string(), Value::Object(inner));

    assert_eq!(config.get("nested"), Some(&Value::Object(outer)));
}

#[test]
fn test_mixed_value_types_in_array() {
    let source = r#"
        flow mixed_array {
            node n1: core.constant {
                mixed = [1, "two", true, null, 4.5]
            }
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse mixed array");
    let config = &flow_def.nodes[0].config;

    let expected = Value::Array(vec![
        Value::Number(1.0),
        Value::String("two".to_string()),
        Value::Bool(true),
        Value::Null,
        Value::Number(4.5),
    ]);
    assert_eq!(config.get("mixed"), Some(&expected));
}

#[test]
fn test_string_object_keys() {
    let source = r#"
        flow string_keys {
            node n1: core.constant {
                obj = {"key with spaces": 1, "another-key": 2}
            }
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse string object keys");
    let config = &flow_def.nodes[0].config;

    if let Some(Value::Object(obj)) = config.get("obj") {
        assert_eq!(obj.get("key with spaces"), Some(&Value::Number(1.0)));
        assert_eq!(obj.get("another-key"), Some(&Value::Number(2.0)));
    } else {
        panic!("Expected object value");
    }
}

#[test]
fn test_whitespace_handling() {
    // Test with various whitespace: tabs, spaces, newlines
    let source = "block\ttest.whitespace\t{\n\n\n    output\t\tvalue:\t\tNumber\n\n}";

    let block_def = parse_block(source).expect("Failed to parse with mixed whitespace");
    assert_eq!(block_def.name, "test.whitespace");
    assert_eq!(block_def.outputs.len(), 1);
}

#[test]
fn test_comments_in_various_places() {
    let source = r#"
        // Top level comment
        block test.comments { // inline comment after brace
            // Comment before description
            description "A test block" // inline after description

            // Comment before input
            input x: Number // inline after input

            output y: Number // inline after output
            // Comment at end of block
        }
        // Comment after block
    "#;

    let block_def = parse_block(source).expect("Failed to parse with many comments");
    assert_eq!(block_def.name, "test.comments");
    assert_eq!(block_def.inputs.len(), 1);
    assert_eq!(block_def.outputs.len(), 1);
}

#[test]
fn test_port_without_braces() {
    let source = r#"
        block test.no_braces {
            input x: Number
            output y: Number
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse ports without braces");
    assert_eq!(block_def.inputs.len(), 1);
    assert_eq!(block_def.outputs.len(), 1);
    assert_eq!(block_def.inputs[0].description, None);
    assert_eq!(block_def.inputs[0].default, None);
}

#[test]
fn test_node_without_config() {
    let source = r#"
        flow test {
            node n1: core.constant
            node n2: math.add {}
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse nodes without config");
    assert_eq!(flow_def.nodes.len(), 2);
    assert!(flow_def.nodes[0].config.is_empty());
    assert!(flow_def.nodes[1].config.is_empty());
}

#[test]
fn test_multiple_outputs() {
    let source = r#"
        flow multi_output {
            node n1: test.block
            node n2: test.block

            output n1.result
            output n2.result
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse multiple outputs");
    assert_eq!(flow_def.outputs.len(), 2);
}

#[test]
fn test_all_operators() {
    let source = r#"
        block test.operators {
            input a: Number
            input b: Number
            output result: Bool

            execute {
                sum = a + b
                diff = a - b
                prod = a * b
                quot = a / b
                mod = a % b
                eq = a == b
                ne = a != b
                lt = a < b
                gt = a > b
                le = a <= b
                ge = a >= b
                and = true && false
                or = true || false
                not = !true
                neg = -a
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse all operators");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_complex_expressions() {
    let source = r#"
        block test.complex_expr {
            input x: Number
            output result: Number

            execute {
                result = (x + 1) * (x - 1) / 2
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse complex expression");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_if_with_else() {
    let source = r#"
        block test.if_else {
            input x: Number
            output result: String

            execute {
                if x > 0 {
                    result = "positive"
                } else {
                    result = "negative"
                }
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse if-else");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_return_statement() {
    let source = r#"
        block test.return {
            input x: Number
            output result: Number

            execute {
                return x * 2
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse return statement");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_member_access() {
    let source = r#"
        block test.member {
            input obj: Object
            output result: Any

            execute {
                result = obj.property.nested
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse member access");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_function_call() {
    let source = r#"
        block test.call {
            input x: Number
            output result: Number

            execute {
                result = pow(x, 2)
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse function call");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_function_call_multiple_args() {
    let source = r#"
        block test.call_multi {
            input a: Number
            input b: Number
            input c: Number
            output result: Number

            execute {
                result = max(a, b, c)
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse function with multiple args");
    assert!(block_def.execute.is_some());
}
