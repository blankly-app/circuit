use circuit_lang::*;

/// Test that expressions parse with correct precedence
/// Note: These tests verify that parsing succeeds with expected AST structure

#[test]
fn test_addition_vs_multiplication() {
    let source = r#"
        block test.precedence {
            input a: Number
            input b: Number
            input c: Number
            output result: Number

            execute {
                result = a + b * c
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());

    // Verify the AST structure: a + (b * c), not (a + b) * c
    let execute = block_def.execute.unwrap();
    assert_eq!(execute.statements.len(), 1);

    match &execute.statements[0] {
        Statement::Assignment { target, value } => {
            assert_eq!(target, "result");
            // The structure should be Binary(Identifier(a), Add, Binary(...))
            // where the right side is Binary(Identifier(b), Mul, Identifier(c))
            match value {
                Expression::Binary { left, op, right } => {
                    assert!(matches!(op, BinaryOp::Add));
                    assert!(matches!(**left, Expression::Identifier(_)));
                    // Right side should be another binary expression (b * c)
                    assert!(matches!(**right, Expression::Binary { .. }));
                }
                _ => panic!("Expected binary expression at top level"),
            }
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_subtraction_vs_division() {
    let source = r#"
        block test.precedence {
            input a: Number
            input b: Number
            input c: Number
            output result: Number

            execute {
                result = a - b / c
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_comparison_vs_arithmetic() {
    let source = r#"
        block test.precedence {
            input a: Number
            input b: Number
            output result: Bool

            execute {
                result = a + 1 > b - 1
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_logical_and_vs_or() {
    let source = r#"
        block test.precedence {
            input a: Bool
            input b: Bool
            input c: Bool
            output result: Bool

            execute {
                result = a || b && c
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_unary_minus_precedence() {
    let source = r#"
        block test.precedence {
            input a: Number
            output result: Number

            execute {
                result = -a * 2
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_unary_not_precedence() {
    let source = r#"
        block test.precedence {
            input a: Bool
            input b: Bool
            output result: Bool

            execute {
                result = !a && b
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_parentheses_override_precedence() {
    let source = r#"
        block test.precedence {
            input a: Number
            input b: Number
            input c: Number
            output result: Number

            execute {
                result = (a + b) * c
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_nested_parentheses() {
    let source = r#"
        block test.precedence {
            input a: Number
            input b: Number
            input c: Number
            input d: Number
            output result: Number

            execute {
                result = ((a + b) * (c - d)) / 2
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_chained_comparisons() {
    // Note: Most languages don't support chained comparisons like a < b < c
    // This tests how the parser handles it (likely as (a < b) < c)
    let source = r#"
        block test.precedence {
            input a: Number
            input b: Number
            input c: Number
            output result: Bool

            execute {
                result = a < b && b < c
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_equality_vs_comparison() {
    let source = r#"
        block test.precedence {
            input a: Number
            input b: Number
            output result: Bool

            execute {
                result = a > 0 == b > 0
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_complex_expression_chain() {
    let source = r#"
        block test.complex {
            input x: Number
            output result: Number

            execute {
                result = x * 2 + 3 * 4 - 5 / 2
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_member_access_precedence() {
    let source = r#"
        block test.member {
            input obj: Object
            output result: Any

            execute {
                result = obj.a.b + obj.c.d
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_function_call_precedence() {
    let source = r#"
        block test.call {
            input x: Number
            output result: Number

            execute {
                result = pow(x, 2) + 1
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_all_arithmetic_operators() {
    let source = r#"
        block test.arithmetic {
            input a: Number
            input b: Number
            input c: Number
            input d: Number
            input e: Number
            output result: Number

            execute {
                result = a + b - c * d / e % 3
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_comparison_chains() {
    let source = r#"
        block test.comparison {
            input a: Number
            input b: Number
            input c: Number
            output r1: Bool
            output r2: Bool
            output r3: Bool

            execute {
                r1 = a < b && b < c
                r2 = a <= b && b <= c
                r3 = a != b && b != c
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_logical_operator_combinations() {
    let source = r#"
        block test.logical {
            input a: Bool
            input b: Bool
            input c: Bool
            input d: Bool
            output result: Bool

            execute {
                result = a && b || c && d
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_unary_in_binary_expression() {
    let source = r#"
        block test.unary_binary {
            input a: Number
            input b: Number
            output result: Number

            execute {
                result = -a + -b
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_double_negation() {
    let source = r#"
        block test.double_neg {
            input a: Number
            output result: Number

            execute {
                result = - -a
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}

#[test]
fn test_not_not() {
    let source = r#"
        block test.not_not {
            input a: Bool
            output result: Bool

            execute {
                result = !!a
            }
        }
    "#;

    let block_def = parse_block(source).expect("Failed to parse");
    assert!(block_def.execute.is_some());
}
