//! Abstract Syntax Tree definitions for Circuit Language

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete block definition from a .block file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockDef {
    pub name: String,
    pub description: Option<String>,
    pub inputs: Vec<PortDef>,
    pub outputs: Vec<PortDef>,
    pub config: Vec<ConfigDef>,
    pub execute: Option<ExecuteBlock>,
}

/// Port definition (input or output)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortDef {
    pub name: String,
    pub port_type: ValueType,
    pub description: Option<String>,
    pub default: Option<Value>,
}

/// Config parameter definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigDef {
    pub name: String,
    pub config_type: ValueType,
    pub description: Option<String>,
    pub default: Option<Value>,
}

/// Execution block containing statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteBlock {
    pub statements: Vec<Statement>,
}

/// Statement in execution block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Assignment { target: String, value: Expression },
    Return { value: Expression },
    If { condition: Expression, then_block: Vec<Statement>, else_block: Option<Vec<Statement>> },
}

/// Expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Value(Value),
    Identifier(String),
    Binary { left: Box<Expression>, op: BinaryOp, right: Box<Expression> },
    Unary { op: UnaryOp, operand: Box<Expression> },
    Call { target: Box<Expression>, args: Vec<Expression> },
    Member { object: Box<Expression>, member: String },
}

/// Binary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not, Neg,
}

/// Value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueType {
    Number,
    String,
    Bool,
    Array,
    Object,
    Bytes,
    Any,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValueType::Number => write!(f, "Number"),
            ValueType::String => write!(f, "String"),
            ValueType::Bool => write!(f, "Bool"),
            ValueType::Array => write!(f, "Array"),
            ValueType::Object => write!(f, "Object"),
            ValueType::Bytes => write!(f, "Bytes"),
            ValueType::Any => write!(f, "Any"),
        }
    }
}

/// Runtime values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

/// A complete flow definition from a .flow file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlowDef {
    pub name: String,
    pub description: Option<String>,
    pub nodes: Vec<NodeDef>,
    pub connections: Vec<ConnectionDef>,
    pub outputs: Vec<PortRef>,
}

/// Node instance in a flow
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeDef {
    pub id: String,
    pub block_type: String,
    pub config: HashMap<String, Value>,
    pub position: Option<(f64, f64)>,
}

/// Connection between ports
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionDef {
    pub from: PortRef,
    pub to: PortRef,
}

/// Reference to a port (node.port)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortRef {
    pub node: String,
    pub port: String,
}

impl std::fmt::Display for PortRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.node, self.port)
    }
}
