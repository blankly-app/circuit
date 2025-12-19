//! Circuit Language Parser
//!
//! This module provides parsers for `.block` and `.flow` files, which are
//! declarative languages for defining computational blocks and flow graphs.
//!
//! # Overview
//!
//! The Circuit language consists of two file types:
//! - **`.block` files**: Define reusable computational blocks with inputs, outputs, and logic
//! - **`.flow` files**: Define graphs that connect blocks together into workflows
//!
//! # Features
//!
//! - Full expression language with operators, conditionals, and function calls
//! - Support for multiple data types: Number, String, Bool, Array, Object, Bytes
//! - String escape sequences: `\n`, `\t`, `\r`, `\\`, `\"`, etc.
//! - Single-line (`//`) and multi-line (`/* */`) comments
//! - Qualified names for block namespacing (e.g., `math.add`, `string.concat`)
//! - Optional descriptions and default values for inputs/config
//!
//! # Examples
//!
//! ## Parsing a Block
//!
//! ```
//! use circuit_lang::parse_block;
//!
//! let source = r#"
//!     block math.square {
//!         description "Squares a number"
//!         input x: Number
//!         output result: Number
//!         execute {
//!             result = x * x
//!         }
//!     }
//! "#;
//!
//! let block = parse_block(source).unwrap();
//! assert_eq!(block.name, "math.square");
//! ```
//!
//! ## Parsing a Flow
//!
//! ```
//! use circuit_lang::parse_flow;
//!
//! let source = r#"
//!     flow calculator {
//!         node a: core.constant { value = 5 }
//!         node b: core.constant { value = 3 }
//!         node sum: math.add
//!         connect a.value -> sum.a
//!         connect b.value -> sum.b
//!         output sum.result
//!     }
//! "#;
//!
//! let flow = parse_flow(source).unwrap();
//! assert_eq!(flow.nodes.len(), 3);
//! ```

mod ast;
mod converter;
mod parser;

pub use ast::*;
pub use converter::*;
pub use parser::*;

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
