//! Parser implementation for Circuit Language using Pest

use crate::ast::*;
use crate::{LangError, Result};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CircuitParser;

/// Parse a .block file
pub fn parse_block(source: &str) -> Result<BlockDef> {
    let pairs = CircuitParser::parse(Rule::block_def, source)
        .map_err(|e| LangError::ParseError(e.to_string()))?;

    let mut block_def = BlockDef {
        name: String::new(),
        description: None,
        inputs: Vec::new(),
        outputs: Vec::new(),
        config: Vec::new(),
        execute: None,
    };

    for pair in pairs {
        match pair.as_rule() {
            Rule::block_def => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::qualified_name => {
                            block_def.name = inner.as_str().to_string();
                        }
                        Rule::block_body => {
                            parse_block_body(inner, &mut block_def)?;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(block_def)
}

fn parse_block_body(pair: pest::iterators::Pair<Rule>, block_def: &mut BlockDef) -> Result<()> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::description_stmt => {
                block_def.description = Some(parse_description(inner)?);
            }
            Rule::input_def => {
                block_def.inputs.push(parse_port_def(inner)?);
            }
            Rule::output_def => {
                block_def.outputs.push(parse_port_def(inner)?);
            }
            Rule::config_def => {
                block_def.config.push(parse_config_def(inner)?);
            }
            Rule::execute_block => {
                block_def.execute = Some(parse_execute_block(inner)?);
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_description(pair: pest::iterators::Pair<Rule>) -> Result<String> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::string_literal {
            let s = inner.as_str();
            return Ok(s[1..s.len()-1].to_string()); // Remove quotes
        }
    }
    Err(LangError::ParseError("Missing description string".to_string()))
}

fn parse_port_def(pair: pest::iterators::Pair<Rule>) -> Result<PortDef> {
    let mut port_def = PortDef {
        name: String::new(),
        port_type: ValueType::Any,
        description: None,
        default: None,
    };

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                port_def.name = inner.as_str().to_string();
            }
            Rule::value_type => {
                port_def.port_type = parse_value_type(inner)?;
            }
            Rule::input_body | Rule::output_body => {
                for body_item in inner.into_inner() {
                    match body_item.as_rule() {
                        Rule::description_stmt => {
                            port_def.description = Some(parse_description(body_item)?);
                        }
                        Rule::default_stmt => {
                            port_def.default = Some(parse_default_stmt(body_item)?);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(port_def)
}

fn parse_config_def(pair: pest::iterators::Pair<Rule>) -> Result<ConfigDef> {
    let mut config_def = ConfigDef {
        name: String::new(),
        config_type: ValueType::Any,
        description: None,
        default: None,
    };

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                config_def.name = inner.as_str().to_string();
            }
            Rule::value_type => {
                config_def.config_type = parse_value_type(inner)?;
            }
            Rule::config_body => {
                for body_item in inner.into_inner() {
                    match body_item.as_rule() {
                        Rule::description_stmt => {
                            config_def.description = Some(parse_description(body_item)?);
                        }
                        Rule::default_stmt => {
                            config_def.default = Some(parse_default_stmt(body_item)?);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(config_def)
}

fn parse_value_type(pair: pest::iterators::Pair<Rule>) -> Result<ValueType> {
    match pair.as_str() {
        "Number" => Ok(ValueType::Number),
        "String" => Ok(ValueType::String),
        "Bool" => Ok(ValueType::Bool),
        "Array" => Ok(ValueType::Array),
        "Object" => Ok(ValueType::Object),
        "Bytes" => Ok(ValueType::Bytes),
        "Any" => Ok(ValueType::Any),
        _ => Err(LangError::ParseError(format!("Unknown type: {}", pair.as_str()))),
    }
}

fn parse_default_stmt(pair: pest::iterators::Pair<Rule>) -> Result<Value> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::value {
            return parse_value(inner);
        }
    }
    Err(LangError::ParseError("Missing default value".to_string()))
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> Result<Value> {
    let inner = pair.into_inner().next()
        .ok_or_else(|| LangError::ParseError("Empty value".to_string()))?;

    match inner.as_rule() {
        Rule::null_literal => Ok(Value::Null),
        Rule::bool_literal => {
            Ok(Value::Bool(inner.as_str() == "true"))
        }
        Rule::number_literal => {
            let num = inner.as_str().parse::<f64>()
                .map_err(|e| LangError::ParseError(format!("Invalid number: {}", e)))?;
            Ok(Value::Number(num))
        }
        Rule::string_literal => {
            let s = inner.as_str();
            Ok(Value::String(s[1..s.len()-1].to_string()))
        }
        Rule::array_value => {
            let mut values = Vec::new();
            for item in inner.into_inner() {
                if item.as_rule() == Rule::value {
                    values.push(parse_value(item)?);
                }
            }
            Ok(Value::Array(values))
        }
        Rule::object_value => {
            let mut map = HashMap::new();
            for item in inner.into_inner() {
                if item.as_rule() == Rule::object_pair {
                    let (key, val) = parse_object_pair(item)?;
                    map.insert(key, val);
                }
            }
            Ok(Value::Object(map))
        }
        _ => Err(LangError::ParseError(format!("Unexpected value type: {:?}", inner.as_rule()))),
    }
}

fn parse_object_pair(pair: pest::iterators::Pair<Rule>) -> Result<(String, Value)> {
    let mut key = String::new();
    let mut value = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                key = inner.as_str().to_string();
            }
            Rule::string_literal => {
                let s = inner.as_str();
                key = s[1..s.len()-1].to_string();
            }
            Rule::value => {
                value = Some(parse_value(inner)?);
            }
            _ => {}
        }
    }

    let value = value.ok_or_else(|| LangError::ParseError("Missing object value".to_string()))?;
    Ok((key, value))
}

fn parse_execute_block(pair: pest::iterators::Pair<Rule>) -> Result<ExecuteBlock> {
    let mut statements = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::statement {
            statements.push(parse_statement(inner)?);
        }
    }

    Ok(ExecuteBlock { statements })
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement> {
    let inner = pair.into_inner().next()
        .ok_or_else(|| LangError::ParseError("Empty statement".to_string()))?;

    match inner.as_rule() {
        Rule::assignment_stmt => {
            let mut target = String::new();
            let mut value = None;

            for item in inner.into_inner() {
                match item.as_rule() {
                    Rule::identifier => {
                        target = item.as_str().to_string();
                    }
                    Rule::expression => {
                        value = Some(parse_expression(item)?);
                    }
                    _ => {}
                }
            }

            let value = value.ok_or_else(|| LangError::ParseError("Missing assignment value".to_string()))?;
            Ok(Statement::Assignment { target, value })
        }
        Rule::return_stmt => {
            let expr = inner.into_inner().next()
                .ok_or_else(|| LangError::ParseError("Missing return value".to_string()))?;
            Ok(Statement::Return { value: parse_expression(expr)? })
        }
        Rule::if_stmt => {
            let mut condition = None;
            let mut then_block = Vec::new();
            let mut else_block = None;

            for item in inner.into_inner() {
                match item.as_rule() {
                    Rule::expression => {
                        condition = Some(parse_expression(item)?);
                    }
                    Rule::statement => {
                        if condition.is_some() && then_block.is_empty() {
                            then_block.push(parse_statement(item)?);
                        } else {
                            if else_block.is_none() {
                                else_block = Some(Vec::new());
                            }
                            else_block.as_mut().unwrap().push(parse_statement(item)?);
                        }
                    }
                    _ => {}
                }
            }

            let condition = condition.ok_or_else(|| LangError::ParseError("Missing if condition".to_string()))?;
            Ok(Statement::If { condition, then_block, else_block })
        }
        _ => Err(LangError::ParseError(format!("Unexpected statement: {:?}", inner.as_rule()))),
    }
}

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
    let inner = pair.into_inner().next()
        .ok_or_else(|| LangError::ParseError("Empty expression".to_string()))?;

    match inner.as_rule() {
        Rule::binary_expr => {
            let mut items = inner.into_inner();
            let left = parse_primary_expr(items.next().unwrap())?;
            let op = parse_binary_op(items.next().unwrap())?;
            let right = parse_expression(items.next().unwrap())?;

            Ok(Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        }
        Rule::unary_expr => {
            let mut items = inner.into_inner();
            let op = parse_unary_op(items.next().unwrap())?;
            let operand = parse_expression(items.next().unwrap())?;

            Ok(Expression::Unary {
                op,
                operand: Box::new(operand),
            })
        }
        Rule::call_expr => {
            let mut target = None;
            let mut args = Vec::new();

            for item in inner.into_inner() {
                match item.as_rule() {
                    Rule::identifier => {
                        target = Some(Expression::Identifier(item.as_str().to_string()));
                    }
                    Rule::member_expr => {
                        target = Some(parse_member_expr(item)?);
                    }
                    Rule::expression => {
                        args.push(parse_expression(item)?);
                    }
                    _ => {}
                }
            }

            let target = target.ok_or_else(|| LangError::ParseError("Missing call target".to_string()))?;
            Ok(Expression::Call {
                target: Box::new(target),
                args,
            })
        }
        Rule::member_expr => parse_member_expr(inner),
        Rule::primary_expr => parse_primary_expr(inner),
        _ => Err(LangError::ParseError(format!("Unexpected expression: {:?}", inner.as_rule()))),
    }
}

fn parse_primary_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
    let inner = pair.into_inner().next()
        .ok_or_else(|| LangError::ParseError("Empty primary expression".to_string()))?;

    match inner.as_rule() {
        Rule::value => Ok(Expression::Value(parse_value(inner)?)),
        Rule::identifier => Ok(Expression::Identifier(inner.as_str().to_string())),
        Rule::expression => parse_expression(inner),
        _ => Err(LangError::ParseError(format!("Unexpected primary expr: {:?}", inner.as_rule()))),
    }
}

fn parse_member_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
    let items = pair.into_inner().collect::<Vec<_>>();

    if items.is_empty() {
        return Err(LangError::ParseError("Empty member expression".to_string()));
    }

    let mut expr = match items[0].as_rule() {
        Rule::identifier => Expression::Identifier(items[0].as_str().to_string()),
        Rule::call_expr => parse_expression(items[0].clone().into())?,
        _ => return Err(LangError::ParseError("Invalid member expression base".to_string())),
    };

    for item in items.iter().skip(1) {
        if item.as_rule() == Rule::identifier {
            expr = Expression::Member {
                object: Box::new(expr),
                member: item.as_str().to_string(),
            };
        }
    }

    Ok(expr)
}

fn parse_binary_op(pair: pest::iterators::Pair<Rule>) -> Result<BinaryOp> {
    match pair.as_str() {
        "+" => Ok(BinaryOp::Add),
        "-" => Ok(BinaryOp::Sub),
        "*" => Ok(BinaryOp::Mul),
        "/" => Ok(BinaryOp::Div),
        "%" => Ok(BinaryOp::Mod),
        "==" => Ok(BinaryOp::Eq),
        "!=" => Ok(BinaryOp::Ne),
        "<" => Ok(BinaryOp::Lt),
        ">" => Ok(BinaryOp::Gt),
        "<=" => Ok(BinaryOp::Le),
        ">=" => Ok(BinaryOp::Ge),
        "&&" => Ok(BinaryOp::And),
        "||" => Ok(BinaryOp::Or),
        _ => Err(LangError::ParseError(format!("Unknown binary op: {}", pair.as_str()))),
    }
}

fn parse_unary_op(pair: pest::iterators::Pair<Rule>) -> Result<UnaryOp> {
    match pair.as_str() {
        "!" => Ok(UnaryOp::Not),
        "-" => Ok(UnaryOp::Neg),
        _ => Err(LangError::ParseError(format!("Unknown unary op: {}", pair.as_str()))),
    }
}

/// Parse a .flow file
pub fn parse_flow(source: &str) -> Result<FlowDef> {
    let pairs = CircuitParser::parse(Rule::flow_def, source)
        .map_err(|e| LangError::ParseError(e.to_string()))?;

    let mut flow_def = FlowDef {
        name: String::new(),
        description: None,
        nodes: Vec::new(),
        connections: Vec::new(),
        outputs: Vec::new(),
    };

    for pair in pairs {
        match pair.as_rule() {
            Rule::flow_def => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::identifier => {
                            flow_def.name = inner.as_str().to_string();
                        }
                        Rule::flow_body => {
                            parse_flow_body(inner, &mut flow_def)?;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(flow_def)
}

fn parse_flow_body(pair: pest::iterators::Pair<Rule>, flow_def: &mut FlowDef) -> Result<()> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::description_stmt => {
                flow_def.description = Some(parse_description(inner)?);
            }
            Rule::node_def => {
                flow_def.nodes.push(parse_node_def(inner)?);
            }
            Rule::connect_stmt => {
                flow_def.connections.push(parse_connection(inner)?);
            }
            Rule::output_stmt => {
                flow_def.outputs.push(parse_output_stmt(inner)?);
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_node_def(pair: pest::iterators::Pair<Rule>) -> Result<NodeDef> {
    let mut node_def = NodeDef {
        id: String::new(),
        block_type: String::new(),
        config: HashMap::new(),
        position: None,
    };

    let mut is_id = true;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                if is_id {
                    node_def.id = inner.as_str().to_string();
                    is_id = false;
                }
            }
            Rule::qualified_name => {
                node_def.block_type = inner.as_str().to_string();
            }
            Rule::node_body => {
                for body_item in inner.into_inner() {
                    match body_item.as_rule() {
                        Rule::config_assign => {
                            let (key, val) = parse_config_assign(body_item)?;
                            node_def.config.insert(key, val);
                        }
                        Rule::position_stmt => {
                            node_def.position = Some(parse_position(body_item)?);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(node_def)
}

fn parse_config_assign(pair: pest::iterators::Pair<Rule>) -> Result<(String, Value)> {
    let mut key = String::new();
    let mut value = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                key = inner.as_str().to_string();
            }
            Rule::value => {
                value = Some(parse_value(inner)?);
            }
            _ => {}
        }
    }

    let value = value.ok_or_else(|| LangError::ParseError("Missing config value".to_string()))?;
    Ok((key, value))
}

fn parse_position(pair: pest::iterators::Pair<Rule>) -> Result<(f64, f64)> {
    let mut numbers = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::number_literal {
            let num = inner.as_str().parse::<f64>()
                .map_err(|e| LangError::ParseError(format!("Invalid position number: {}", e)))?;
            numbers.push(num);
        }
    }

    if numbers.len() == 2 {
        Ok((numbers[0], numbers[1]))
    } else {
        Err(LangError::ParseError("Position requires exactly 2 numbers".to_string()))
    }
}

fn parse_connection(pair: pest::iterators::Pair<Rule>) -> Result<ConnectionDef> {
    let mut port_refs = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::port_ref {
            port_refs.push(parse_port_ref(inner)?);
        }
    }

    if port_refs.len() == 2 {
        Ok(ConnectionDef {
            from: port_refs[0].clone(),
            to: port_refs[1].clone(),
        })
    } else {
        Err(LangError::ParseError("Connection requires exactly 2 port refs".to_string()))
    }
}

fn parse_output_stmt(pair: pest::iterators::Pair<Rule>) -> Result<PortRef> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::port_ref {
            return parse_port_ref(inner);
        }
    }
    Err(LangError::ParseError("Missing output port ref".to_string()))
}

fn parse_port_ref(pair: pest::iterators::Pair<Rule>) -> Result<PortRef> {
    let mut parts = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::identifier {
            parts.push(inner.as_str().to_string());
        }
    }

    if parts.len() == 2 {
        Ok(PortRef {
            node: parts[0].clone(),
            port: parts[1].clone(),
        })
    } else {
        Err(LangError::ParseError("Port ref must be node.port".to_string()))
    }
}

/// Parse a file (auto-detect .block or .flow)
pub fn parse_file(source: &str) -> Result<FileType> {
    // Try parsing as block first
    if let Ok(block) = parse_block(source) {
        return Ok(FileType::Block(block));
    }

    // Try parsing as flow
    if let Ok(flow) = parse_flow(source) {
        return Ok(FileType::Flow(flow));
    }

    Err(LangError::ParseError("Could not parse as block or flow".to_string()))
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Block(BlockDef),
    Flow(FlowDef),
}
