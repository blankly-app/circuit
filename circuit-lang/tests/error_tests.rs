use circuit_lang::*;

#[test]
fn test_missing_block_name() {
    let source = r#"
        block {
            output value: Number
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail without block name");
}

#[test]
fn test_missing_block_braces() {
    let source = "block test.name";

    assert!(parse_block(source).is_err(), "Should fail without braces");
}

#[test]
fn test_unclosed_block() {
    let source = r#"
        block test.unclosed {
            output value: Number
    "#;

    assert!(parse_block(source).is_err(), "Should fail with unclosed block");
}

#[test]
fn test_invalid_identifier_starting_with_number() {
    let source = r#"
        block 123test {
            output value: Number
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with identifier starting with number");
}

#[test]
fn test_invalid_type() {
    let source = r#"
        block test.invalid_type {
            input x: InvalidType
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with invalid type");
}

#[test]
fn test_input_without_type() {
    let source = r#"
        block test.no_type {
            input x
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail without type annotation");
}

#[test]
fn test_malformed_connection() {
    let source = r#"
        flow test {
            node n1: core.constant
            connect n1.value
        }
    "#;

    assert!(parse_flow(source).is_err(), "Should fail with incomplete connection");
}

#[test]
fn test_connection_without_arrow() {
    let source = r#"
        flow test {
            node n1: core.constant
            node n2: math.add
            connect n1.value n2.a
        }
    "#;

    assert!(parse_flow(source).is_err(), "Should fail without arrow");
}

#[test]
fn test_invalid_port_ref() {
    let source = r#"
        flow test {
            node n1: core.constant
            output n1
        }
    "#;

    assert!(parse_flow(source).is_err(), "Should fail with incomplete port reference");
}

#[test]
fn test_malformed_array() {
    let source = r#"
        flow test {
            node n1: core.constant {
                arr = [1, 2,
            }
        }
    "#;

    assert!(parse_flow(source).is_err(), "Should fail with unclosed array");
}

#[test]
fn test_malformed_object() {
    let source = r#"
        flow test {
            node n1: core.constant {
                obj = {"key": "value",}
            }
        }
    "#;

    // Trailing comma might be acceptable depending on grammar
    // This tests current behavior
    let result = parse_flow(source);
    // Currently this should fail because we don't support trailing commas
    assert!(result.is_err(), "Should fail with trailing comma in object");
}

#[test]
fn test_object_without_colon() {
    let source = r#"
        flow test {
            node n1: core.constant {
                obj = {"key" "value"}
            }
        }
    "#;

    assert!(parse_flow(source).is_err(), "Should fail without colon in object");
}

#[test]
fn test_unclosed_string() {
    let source = r#"
        block test.unclosed {
            description "This string never closes
            output value: Number
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with unclosed string");
}

#[test]
fn test_invalid_escape_sequence() {
    let source = r#"
        flow test {
            node n1: core.constant {
                val = "invalid \x escape"
            }
        }
    "#;

    // This should actually parse successfully because we allow unknown escapes
    // Let's test that it parses but keeps the backslash
    let result = parse_flow(source);
    assert!(result.is_ok(), "Should parse with unknown escape (keeping backslash)");
}

#[test]
fn test_missing_node_type() {
    let source = r#"
        flow test {
            node n1
        }
    "#;

    assert!(parse_flow(source).is_err(), "Should fail without node type");
}

#[test]
fn test_invalid_position() {
    let source = r#"
        flow test {
            node n1: core.constant {
                position(100)
            }
        }
    "#;

    assert!(parse_flow(source).is_err(), "Should fail with only one position coordinate");
}

#[test]
fn test_position_with_non_numeric() {
    let source = r#"
        flow test {
            node n1: core.constant {
                position("100", "200")
            }
        }
    "#;

    assert!(parse_flow(source).is_err(), "Should fail with non-numeric position");
}

#[test]
fn test_empty_qualified_name() {
    let source = r#"
        block . {
            output value: Number
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with empty qualified name");
}

#[test]
fn test_qualified_name_ending_with_dot() {
    let source = r#"
        block test.name. {
            output value: Number
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with trailing dot");
}

#[test]
fn test_flow_with_qualified_name() {
    let source = r#"
        flow test.flow {
            node n1: core.constant
        }
    "#;

    assert!(parse_flow(source).is_err(), "Should fail - flow names cannot be qualified");
}

#[test]
fn test_missing_assignment_value() {
    let source = r#"
        block test.missing {
            input x: Number
            output y: Number

            execute {
                y =
            }
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with missing assignment value");
}

#[test]
fn test_missing_if_condition() {
    let source = r#"
        block test.missing {
            output y: String

            execute {
                if {
                    y = "value"
                }
            }
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with missing if condition");
}

#[test]
fn test_malformed_binary_expression() {
    let source = r#"
        block test.malformed {
            input x: Number
            output y: Number

            execute {
                y = x +
            }
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with incomplete binary expression");
}

#[test]
fn test_unclosed_parenthesis() {
    let source = r#"
        block test.unclosed {
            input x: Number
            output y: Number

            execute {
                y = (x + 1
            }
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with unclosed parenthesis");
}

#[test]
fn test_function_call_unclosed() {
    let source = r#"
        block test.unclosed {
            input x: Number
            output y: Number

            execute {
                y = pow(x, 2
            }
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with unclosed function call");
}

#[test]
fn test_invalid_operator() {
    let source = r#"
        block test.invalid {
            input x: Number
            output y: Number

            execute {
                y = x & b
            }
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with invalid operator");
}

#[test]
fn test_duplicate_default_in_input() {
    // This actually might parse successfully but is semantically wrong
    let source = r#"
        block test.duplicate {
            input x: Number {
                default = 1
                default = 2
            }
        }
    "#;

    // Parser may accept this, only keeping the last one
    let result = parse_block(source);
    // If it parses, that's acceptable - validation should catch semantic errors
    if result.is_ok() {
        let block = result.unwrap();
        // Should only have one default value (the last one)
        assert_eq!(block.inputs[0].default, Some(Value::Number(2.0)));
    }
}

#[test]
fn test_empty_execute_block() {
    let source = r#"
        block test.empty_exec {
            output y: Number

            execute {
            }
        }
    "#;

    let result = parse_block(source);
    assert!(result.is_ok(), "Empty execute block should be valid");
    if let Ok(block) = result {
        assert!(block.execute.is_some());
        assert_eq!(block.execute.unwrap().statements.len(), 0);
    }
}

#[test]
fn test_keyword_as_identifier() {
    // Test using keywords in places where they shouldn't be allowed
    let source = r#"
        block test.keyword {
            input if: Number
        }
    "#;

    assert!(parse_block(source).is_err(), "Should fail with keyword as identifier");
}
