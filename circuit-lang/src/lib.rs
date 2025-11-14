//! Circuit Language Parser
//!
//! This module provides parsers for `.block` and `.flow` files, which are
//! declarative languages for defining computational blocks and flow graphs.

mod ast;
mod parser;
mod converter;

pub use ast::*;
pub use parser::*;
pub use converter::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LangError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, LangError>;

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(block_def.inputs.len(), 1);
        assert_eq!(block_def.outputs.len(), 1);
    }

    #[test]
    fn test_parse_simple_flow() {
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
        assert_eq!(flow_def.nodes.len(), 3);
        assert_eq!(flow_def.connections.len(), 2);
    }
}
