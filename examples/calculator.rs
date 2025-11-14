//! Example: Simple calculator using Circuit blocks
//!
//! This example demonstrates how to create and execute a simple graph
//! that performs mathematical calculations.

use circuit_core::{blocks::*, *};
use std::collections::HashMap;
use std::sync::Arc;

fn main() {
    // Create a new engine
    let mut engine = Engine::new();

    // Register blocks
    println!("Registering blocks...");
    engine.register_block(Arc::new(ConstantBlock)).unwrap();
    engine.register_block(Arc::new(AddBlock)).unwrap();
    engine.register_block(Arc::new(MultiplyBlock)).unwrap();
    engine.register_block(Arc::new(DebugBlock)).unwrap();

    // Create a graph: (5 + 3) * 2 = 16
    println!("\nCreating graph: (5 + 3) * 2");
    let mut graph = Graph::new("calculator".to_string(), "Simple Calculator".to_string());

    // Create constant node for 5
    let mut config1 = HashMap::new();
    config1.insert("value".to_string(), Value::Float(5.0));
    let node1 = graph::Node {
        id: "const_5".to_string(),
        block_type: "core.constant".to_string(),
        config: config1,
        position: Some((0.0, 0.0)),
    };

    // Create constant node for 3
    let mut config2 = HashMap::new();
    config2.insert("value".to_string(), Value::Float(3.0));
    let node2 = graph::Node {
        id: "const_3".to_string(),
        block_type: "core.constant".to_string(),
        config: config2,
        position: Some((0.0, 100.0)),
    };

    // Create add node
    let node3 = graph::Node {
        id: "add".to_string(),
        block_type: "math.add".to_string(),
        config: HashMap::new(),
        position: Some((200.0, 50.0)),
    };

    // Create constant node for 2
    let mut config4 = HashMap::new();
    config4.insert("value".to_string(), Value::Float(2.0));
    let node4 = graph::Node {
        id: "const_2".to_string(),
        block_type: "core.constant".to_string(),
        config: config4,
        position: Some((200.0, 150.0)),
    };

    // Create multiply node
    let node5 = graph::Node {
        id: "multiply".to_string(),
        block_type: "math.multiply".to_string(),
        config: HashMap::new(),
        position: Some((400.0, 100.0)),
    };

    // Create debug node
    let node6 = graph::Node {
        id: "debug".to_string(),
        block_type: "core.debug".to_string(),
        config: HashMap::new(),
        position: Some((600.0, 100.0)),
    };

    // Add nodes to graph
    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();
    graph.add_node(node4).unwrap();
    graph.add_node(node5).unwrap();
    graph.add_node(node6).unwrap();

    // Connect nodes
    graph
        .add_connection(graph::Connection {
            from_node: "const_5".to_string(),
            from_port: "value".to_string(),
            to_node: "add".to_string(),
            to_port: "a".to_string(),
        })
        .unwrap();

    graph
        .add_connection(graph::Connection {
            from_node: "const_3".to_string(),
            from_port: "value".to_string(),
            to_node: "add".to_string(),
            to_port: "b".to_string(),
        })
        .unwrap();

    graph
        .add_connection(graph::Connection {
            from_node: "add".to_string(),
            from_port: "result".to_string(),
            to_node: "multiply".to_string(),
            to_port: "a".to_string(),
        })
        .unwrap();

    graph
        .add_connection(graph::Connection {
            from_node: "const_2".to_string(),
            from_port: "value".to_string(),
            to_node: "multiply".to_string(),
            to_port: "b".to_string(),
        })
        .unwrap();

    graph
        .add_connection(graph::Connection {
            from_node: "multiply".to_string(),
            from_port: "result".to_string(),
            to_node: "debug".to_string(),
            to_port: "value".to_string(),
        })
        .unwrap();

    // Load graph into engine
    println!("\nLoading graph into engine...");
    engine.load_graph(graph).unwrap();

    // Execute the graph
    println!("\nExecuting graph...");
    let results = engine.execute_graph("calculator").unwrap();

    // Print results
    println!("\n=== Execution Results ===");
    for (node_id, outputs) in results.iter() {
        println!("Node '{}': {:?}", node_id, outputs);
    }

    // Get final result
    if let Some(multiply_outputs) = results.get("multiply") {
        if let Some(result) = multiply_outputs.get("result") {
            println!("\n✓ Final result: {:?}", result);
        }
    }

    println!("\nExample completed successfully!");
}
