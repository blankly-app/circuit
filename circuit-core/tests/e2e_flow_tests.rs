use circuit_core::blocks::control::*;
use circuit_core::blocks::core::*;
use circuit_core::blocks::logic::*;
use circuit_core::blocks::math::*;
use circuit_core::blocks::string::*;
use circuit_core::graph::{Connection, Graph, Node};
use circuit_core::{Engine, Value};
use circuit_lang::{flow_to_graph, parse_flow};
use std::collections::HashMap;
use std::sync::Arc;

/// Register all 34 block types with the engine
fn create_engine_with_all_blocks() -> Engine {
    let mut engine = Engine::new();

    // Core blocks
    engine.register_block(Arc::new(ConstantBlock)).unwrap();
    engine.register_block(Arc::new(DebugBlock)).unwrap();

    // Math blocks
    engine.register_block(Arc::new(AddBlock)).unwrap();
    engine.register_block(Arc::new(SubtractBlock)).unwrap();
    engine.register_block(Arc::new(MultiplyBlock)).unwrap();
    engine.register_block(Arc::new(DivideBlock)).unwrap();
    engine.register_block(Arc::new(ModuloBlock)).unwrap();
    engine.register_block(Arc::new(AbsBlock)).unwrap();
    engine.register_block(Arc::new(NegateBlock)).unwrap();
    engine.register_block(Arc::new(PowerBlock)).unwrap();
    engine.register_block(Arc::new(SqrtBlock)).unwrap();
    engine.register_block(Arc::new(MinBlock)).unwrap();
    engine.register_block(Arc::new(MaxBlock)).unwrap();
    engine.register_block(Arc::new(ClampBlock)).unwrap();
    engine.register_block(Arc::new(RoundBlock)).unwrap();
    engine.register_block(Arc::new(FloorBlock)).unwrap();
    engine.register_block(Arc::new(CeilBlock)).unwrap();
    engine.register_block(Arc::new(SinBlock)).unwrap();
    engine.register_block(Arc::new(CosBlock)).unwrap();
    engine.register_block(Arc::new(TanBlock)).unwrap();

    // Logic blocks
    engine.register_block(Arc::new(AndBlock)).unwrap();
    engine.register_block(Arc::new(OrBlock)).unwrap();
    engine.register_block(Arc::new(NotBlock)).unwrap();
    engine.register_block(Arc::new(EqualBlock)).unwrap();
    engine.register_block(Arc::new(GreaterBlock)).unwrap();
    engine.register_block(Arc::new(LessBlock)).unwrap();

    // String blocks
    engine.register_block(Arc::new(ConcatBlock)).unwrap();
    engine.register_block(Arc::new(LengthBlock)).unwrap();
    engine.register_block(Arc::new(UppercaseBlock)).unwrap();
    engine.register_block(Arc::new(LowercaseBlock)).unwrap();
    engine.register_block(Arc::new(TrimBlock)).unwrap();
    engine.register_block(Arc::new(ContainsBlock)).unwrap();
    engine.register_block(Arc::new(ReplaceBlock)).unwrap();
    engine.register_block(Arc::new(SplitBlock)).unwrap();
    engine.register_block(Arc::new(JoinBlock)).unwrap();
    engine.register_block(Arc::new(SubstringBlock)).unwrap();
    engine.register_block(Arc::new(TemplateBlock)).unwrap();

    // Control blocks
    engine.register_block(Arc::new(IfBlock)).unwrap();
    engine.register_block(Arc::new(SwitchBlock)).unwrap();
    engine.register_block(Arc::new(GateBlock)).unwrap();
    engine.register_block(Arc::new(CounterBlock)).unwrap();
    engine.register_block(Arc::new(AccumulatorBlock)).unwrap();

    engine
}

fn load_flow_file(name: &str) -> String {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up to circuit root
    path.push("examples/flows");
    path.push(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e))
}

#[test]
fn test_calculator_flow_e2e() {
    // (5 + 3) * 2 = 16
    let source = load_flow_file("calculator.flow");
    let flow = parse_flow(&source).expect("Failed to parse calculator.flow");
    let graph = flow_to_graph(&flow).expect("Failed to convert to graph");

    let mut engine = create_engine_with_all_blocks();
    engine.load_graph(graph).expect("Failed to load graph");

    let results = engine
        .execute_graph("calculator")
        .expect("Failed to execute");

    // The multiply node should have result = 16.0
    let multiply_output = results
        .get("multiply")
        .expect("Missing multiply node output");
    let result = multiply_output.get("result").expect("Missing result");
    assert_eq!(result.as_float(), Some(16.0));
}

#[test]
fn test_data_pipeline_flow_e2e() {
    // (10 + 20) * 30 + 100 = 1000
    let source = load_flow_file("data_pipeline.flow");
    let flow = parse_flow(&source).expect("Failed to parse data_pipeline.flow");
    let graph = flow_to_graph(&flow).expect("Failed to convert to graph");

    let mut engine = create_engine_with_all_blocks();
    engine.load_graph(graph).expect("Failed to load graph");

    let results = engine
        .execute_graph("data_pipeline")
        .expect("Failed to execute");

    // stage3 (final add) should produce 1000
    let stage3_output = results.get("stage3").expect("Missing stage3 node output");
    let result = stage3_output.get("result").expect("Missing result");
    assert_eq!(result.as_float(), Some(1000.0));
}

#[test]
fn test_string_processing_flow_e2e() {
    // "Hello" + " " + "World" = "Hello World"
    let source = load_flow_file("string_processing.flow");
    let flow = parse_flow(&source).expect("Failed to parse string_processing.flow");
    let graph = flow_to_graph(&flow).expect("Failed to convert to graph");

    let mut engine = create_engine_with_all_blocks();
    engine.load_graph(graph).expect("Failed to load graph");

    let results = engine
        .execute_graph("string_processing")
        .expect("Failed to execute");

    // concat2 should produce "Hello World"
    let concat2_output = results.get("concat2").expect("Missing concat2 node output");
    let result = concat2_output.get("result").expect("Missing result");
    assert_eq!(result, &Value::String("Hello World".to_string()));
}

#[test]
fn test_advanced_math_flow_e2e() {
    // abs(sqrt(9) - 4) = abs(3 - 4) = abs(-1) = 1.0
    let source = load_flow_file("advanced_math.flow");
    let flow = parse_flow(&source).expect("Failed to parse advanced_math.flow");
    let graph = flow_to_graph(&flow).expect("Failed to convert to graph");

    let mut engine = create_engine_with_all_blocks();
    engine.load_graph(graph).expect("Failed to load graph");

    let results = engine
        .execute_graph("advanced_math")
        .expect("Failed to execute");

    let abs_output = results.get("abs").expect("Missing abs node output");
    let result = abs_output.get("result").expect("Missing result");
    assert_eq!(result.as_float(), Some(1.0));
}

#[test]
fn test_conditional_flow_e2e() {
    // if 10 > 5 then "yes" else "no" → "yes"
    let source = load_flow_file("conditional.flow");
    let flow = parse_flow(&source).expect("Failed to parse conditional.flow");
    let graph = flow_to_graph(&flow).expect("Failed to convert to graph");

    let mut engine = create_engine_with_all_blocks();
    engine.load_graph(graph).expect("Failed to load graph");

    let results = engine
        .execute_graph("conditional")
        .expect("Failed to execute");

    let branch_output = results.get("branch").expect("Missing branch node output");
    let result = branch_output.get("result").expect("Missing result");
    assert_eq!(result, &Value::String("yes".to_string()));
}

#[test]
fn test_string_transform_flow_e2e() {
    // uppercase("hello world") → "HELLO WORLD", split by " " → ["HELLO", "WORLD"]
    let source = load_flow_file("string_transform.flow");
    let flow = parse_flow(&source).expect("Failed to parse string_transform.flow");
    let graph = flow_to_graph(&flow).expect("Failed to convert to graph");

    let mut engine = create_engine_with_all_blocks();
    engine.load_graph(graph).expect("Failed to load graph");

    let results = engine
        .execute_graph("string_transform")
        .expect("Failed to execute");

    let split_output = results.get("split").expect("Missing split node output");
    let result = split_output.get("result").expect("Missing result");
    assert_eq!(
        result,
        &Value::Array(vec![
            Value::String("HELLO".to_string()),
            Value::String("WORLD".to_string()),
        ])
    );
}

#[test]
fn test_programmatic_graph_e2e() {
    // Build a graph in code: clamp(power(2, 10), 0, 500) = 500
    let mut engine = create_engine_with_all_blocks();

    let mut graph = Graph::new("programmatic".to_string(), "Programmatic Graph".to_string());

    // Constant: base = 2
    let mut config_base = HashMap::new();
    config_base.insert("value".to_string(), Value::Float(2.0));
    graph
        .add_node(Node {
            id: "base".to_string(),
            block_type: "core.constant".to_string(),
            config: config_base,
            position: None,
        })
        .unwrap();

    // Constant: exponent = 10
    let mut config_exp = HashMap::new();
    config_exp.insert("value".to_string(), Value::Float(10.0));
    graph
        .add_node(Node {
            id: "exp".to_string(),
            block_type: "core.constant".to_string(),
            config: config_exp,
            position: None,
        })
        .unwrap();

    // Power: 2^10 = 1024
    graph
        .add_node(Node {
            id: "pow".to_string(),
            block_type: "math.power".to_string(),
            config: HashMap::new(),
            position: None,
        })
        .unwrap();

    // Constant: min = 0
    let mut config_min = HashMap::new();
    config_min.insert("value".to_string(), Value::Float(0.0));
    graph
        .add_node(Node {
            id: "min_val".to_string(),
            block_type: "core.constant".to_string(),
            config: config_min,
            position: None,
        })
        .unwrap();

    // Constant: max = 500
    let mut config_max = HashMap::new();
    config_max.insert("value".to_string(), Value::Float(500.0));
    graph
        .add_node(Node {
            id: "max_val".to_string(),
            block_type: "core.constant".to_string(),
            config: config_max,
            position: None,
        })
        .unwrap();

    // Clamp: clamp(1024, 0, 500) = 500
    graph
        .add_node(Node {
            id: "clamp".to_string(),
            block_type: "math.clamp".to_string(),
            config: HashMap::new(),
            position: None,
        })
        .unwrap();

    // Connections
    graph
        .add_connection(Connection {
            from_node: "base".to_string(),
            from_port: "value".to_string(),
            to_node: "pow".to_string(),
            to_port: "base".to_string(),
        })
        .unwrap();
    graph
        .add_connection(Connection {
            from_node: "exp".to_string(),
            from_port: "value".to_string(),
            to_node: "pow".to_string(),
            to_port: "exponent".to_string(),
        })
        .unwrap();
    graph
        .add_connection(Connection {
            from_node: "pow".to_string(),
            from_port: "result".to_string(),
            to_node: "clamp".to_string(),
            to_port: "value".to_string(),
        })
        .unwrap();
    graph
        .add_connection(Connection {
            from_node: "min_val".to_string(),
            from_port: "value".to_string(),
            to_node: "clamp".to_string(),
            to_port: "min".to_string(),
        })
        .unwrap();
    graph
        .add_connection(Connection {
            from_node: "max_val".to_string(),
            from_port: "value".to_string(),
            to_node: "clamp".to_string(),
            to_port: "max".to_string(),
        })
        .unwrap();

    engine.load_graph(graph).unwrap();
    let results = engine.execute_graph("programmatic").unwrap();

    let clamp_output = results.get("clamp").expect("Missing clamp node output");
    let result = clamp_output.get("result").expect("Missing result");
    assert_eq!(result.as_float(), Some(500.0));
}
