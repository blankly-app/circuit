use circuit_lang::*;
use pretty_assertions::assert_eq;

#[test]
fn test_parse_simple_block() {
    let source = r#"
        block math.square {
            description "Squares a number"

            input x: Number {
                description "Input value"
            }

            output result: Number {
                description "x squared"
            }

            execute {
                result = x * x
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse block");
    assert_eq!(block_def.name, "math.square");
    assert_eq!(block_def.description, Some("Squares a number".to_string()));
    assert_eq!(block_def.inputs.len(), 1);
    assert_eq!(block_def.inputs[0].name, "x");
    assert_eq!(block_def.outputs.len(), 1);
    assert_eq!(block_def.outputs[0].name, "result");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_parse_block_with_config() {
    let source = r#"
        block math.power {
            description "Raises base to exponent"

            input base: Number

            config exponent: Number {
                description "The power to raise to"
                default = 2
            }

            output result: Number

            execute {
                result = base * base
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse block");
    assert_eq!(block_def.config.len(), 1);
    assert_eq!(block_def.config[0].name, "exponent");
    assert_eq!(block_def.config[0].default, Some(Value::Number(2.0)));
}

#[test]
fn test_parse_block_with_multiple_ports() {
    let source = r#"
        block math.complex {
            input a: Number
            input b: Number
            input c: Number

            output sum: Number
            output product: Number

            execute {
                sum = a + b + c
                product = a * b * c
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse block");
    assert_eq!(block_def.inputs.len(), 3);
    assert_eq!(block_def.outputs.len(), 2);
}

#[test]
fn test_parse_flow_simple() {
    let source = r#"
        flow calculator {
            description "Simple calculator"

            node n1: core.constant {
                value = 5
            }

            node n2: core.constant {
                value = 3
            }

            node add: math.add

            connect n1.value -> add.a
            connect n2.value -> add.b

            output add.result
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse flow");
    assert_eq!(flow_def.name, "calculator");
    assert_eq!(flow_def.description, Some("Simple calculator".to_string()));
    assert_eq!(flow_def.nodes.len(), 3);
    assert_eq!(flow_def.connections.len(), 2);
    assert_eq!(flow_def.outputs.len(), 1);
}

#[test]
fn test_parse_flow_with_positions() {
    let source = r#"
        flow positioned {
            node n1: core.constant {
                value = 42
                position(100, 200)
            }

            node n2: core.constant {
                value = 10
                position(300, 200)
            }

            connect n1.value -> n2.value
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse flow");
    assert_eq!(flow_def.nodes[0].position, Some((100.0, 200.0)));
    assert_eq!(flow_def.nodes[1].position, Some((300.0, 200.0)));
}

#[test]
fn test_parse_values() {
    let source = r#"
        flow values_test {
            node n1: test.block {
                null_val = null
                bool_val = true
                num_val = 42.5
                str_val = "hello"
                array_val = [1, 2, 3]
                obj_val = {"key": "value", "count": 10}
            }
        }
    "#;

    let flow_def = parse_flow(source).expect("Failed to parse flow");
    let config = &flow_def.nodes[0].config;

    assert_eq!(config.get("null_val"), Some(&Value::Null));
    assert_eq!(config.get("bool_val"), Some(&Value::Bool(true)));
    assert_eq!(config.get("num_val"), Some(&Value::Number(42.5)));
    assert_eq!(
        config.get("str_val"),
        Some(&Value::String("hello".to_string()))
    );
}

#[test]
fn test_parse_block_expressions() {
    let source = r#"
        block test.expressions {
            input x: Number
            input y: Number
            output result: Number

            execute {
                result = (x + y) * 2 - 1
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse block");
    assert!(block_def.execute.is_some());
    let execute = block_def.execute.unwrap();
    assert_eq!(execute.statements.len(), 1);
}

#[test]
fn test_parse_block_conditionals() {
    let source = r#"
        block test.conditional {
            input x: Number
            output result: String

            execute {
                if x > 0 {
                    result = "positive"
                }
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse block");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_parse_qualified_names() {
    let source = r#"
        block my.namespace.deeply.nested.block {
            description "A deeply nested block"
            output value: Number
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse block");
    assert_eq!(block_def.name, "my.namespace.deeply.nested.block");
}

#[test]
fn test_parse_all_value_types() {
    let source = r#"
        block test.types {
            input num: Number
            input str: String
            input bool: Bool
            input arr: Array
            input obj: Object
            input bytes: Bytes
            input any: Any

            output result: Any
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse block");
    assert_eq!(block_def.inputs.len(), 7);
}

#[test]
fn test_invalid_block() {
    let source = r#"
        block invalid syntax here {
            this is not valid
        }
    "#;

    assert!(parse_block(source).is_err());
}

#[test]
fn test_invalid_flow() {
    let source = r#"
        flow invalid {
            node without type specifier
        }
    "#;

    assert!(parse_flow(source).is_err());
}

#[test]
fn test_parse_file_auto_detect_block() {
    let source = r#"
        block auto.detect {
            output value: Number
        }
    "#;

    let result = parse_file(source).expect("Failed to parse");
    match result {
        FileType::Block(block) => {
            assert_eq!(block.name, "auto.detect");
        }
        _ => panic!("Expected Block type"),
    }
}

#[test]
fn test_parse_file_auto_detect_flow() {
    let source = r#"
        flow auto_detect {
            node n1: core.constant
        }
    "#;

    let result = parse_file(source).expect("Failed to parse");
    match result {
        FileType::Flow(flow) => {
            assert_eq!(flow.name, "auto_detect");
        }
        _ => panic!("Expected Flow type"),
    }
}
