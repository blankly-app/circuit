//! Converter from Circuit Language AST to Circuit Core types

use crate::ast::*;
use crate::{LangError, Result};
use circuit_core::graph::{Connection, Graph, Node};
use circuit_core::Value as CoreValue;
use std::collections::HashMap;

/// Convert a FlowDef to a Graph
pub fn flow_to_graph(flow: &FlowDef) -> Result<Graph> {
    let mut graph = Graph::new(
        flow.name.clone(),
        flow.description.clone().unwrap_or_default(),
    );

    // Add all nodes
    for node_def in &flow.nodes {
        let node = node_def_to_node(node_def)?;
        graph
            .add_node(node)
            .map_err(|e| LangError::ValidationError(format!("Failed to add node: {}", e)))?;
    }

    // Add all connections
    for conn_def in &flow.connections {
        let connection = connection_def_to_connection(conn_def);
        graph
            .add_connection(connection)
            .map_err(|e| LangError::ValidationError(format!("Failed to add connection: {}", e)))?;
    }

    Ok(graph)
}

fn node_def_to_node(node_def: &NodeDef) -> Result<Node> {
    let config = convert_value_map(&node_def.config)?;

    Ok(Node {
        id: node_def.id.clone(),
        block_type: node_def.block_type.clone(),
        config,
        position: node_def.position,
    })
}

fn connection_def_to_connection(conn_def: &ConnectionDef) -> Connection {
    Connection {
        from_node: conn_def.from.node.clone(),
        from_port: conn_def.from.port.clone(),
        to_node: conn_def.to.node.clone(),
        to_port: conn_def.to.port.clone(),
    }
}

fn convert_value_map(map: &HashMap<String, Value>) -> Result<HashMap<String, CoreValue>> {
    let mut result = HashMap::new();
    for (key, value) in map {
        result.insert(key.clone(), value_to_core_value(value)?);
    }
    Ok(result)
}

fn value_to_core_value(value: &Value) -> Result<CoreValue> {
    match value {
        Value::Null => Ok(CoreValue::Null),
        Value::Bool(b) => Ok(CoreValue::Bool(*b)),
        Value::Number(n) => Ok(CoreValue::Float(*n)),
        Value::String(s) => Ok(CoreValue::String(s.clone())),
        Value::Array(arr) => {
            let mut result = Vec::new();
            for item in arr {
                result.push(value_to_core_value(item)?);
            }
            Ok(CoreValue::Array(result))
        }
        Value::Object(obj) => {
            let mut result = HashMap::new();
            for (key, val) in obj {
                result.insert(key.clone(), value_to_core_value(val)?);
            }
            Ok(CoreValue::Object(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_flow;

    #[test]
    fn test_convert_simple_flow() {
        let source = r#"
            flow test {
                description "Test flow"

                node n1: core.constant {
                    value = 42
                }

                node n2: core.constant {
                    value = 10
                }

                node add: math.add

                connect n1.value -> add.a
                connect n2.value -> add.b
            }
        "#;

        let flow = parse_flow(source).expect("Failed to parse");
        let graph = flow_to_graph(&flow).expect("Failed to convert");

        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.connections.len(), 2);
    }

    #[test]
    fn test_convert_with_positions() {
        let source = r#"
            flow positioned {
                node n1: test.block {
                    position(100, 200)
                }
            }
        "#;

        let flow = parse_flow(source).expect("Failed to parse");
        let graph = flow_to_graph(&flow).expect("Failed to convert");

        let node = &graph.nodes.get("n1").expect("Node not found");
        assert_eq!(node.position, Some((100.0, 200.0)));
    }

    #[test]
    fn test_convert_complex_values() {
        let source = r#"
            flow values {
                node n1: test.block {
                    num = 42.5
                    str = "hello"
                    bool = true
                    arr = [1, 2, 3]
                    obj = {"key": "value"}
                }
            }
        "#;

        let flow = parse_flow(source).expect("Failed to parse");
        let graph = flow_to_graph(&flow).expect("Failed to convert");

        let node = &graph.nodes.get("n1").expect("Node not found");
        assert_eq!(node.config.get("num"), Some(&CoreValue::Float(42.5)));
        assert_eq!(
            node.config.get("str"),
            Some(&CoreValue::String("hello".to_string()))
        );
        assert_eq!(node.config.get("bool"), Some(&CoreValue::Bool(true)));
    }
}
