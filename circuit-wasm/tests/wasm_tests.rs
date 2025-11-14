use circuit_wasm::WasmEngine;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_wasm_engine_creation() {
    let engine = WasmEngine::new();
    assert_eq!(engine.list_graphs().len(), 0);
}

#[wasm_bindgen_test]
fn test_wasm_engine_default() {
    let engine = WasmEngine::default();
    assert_eq!(engine.list_blocks().len(), 0);
}

#[wasm_bindgen_test]
fn test_load_graph() {
    let mut engine = WasmEngine::new();

    let graph_json = r#"{
        "id": "test_graph",
        "name": "Test Graph",
        "nodes": {},
        "connections": []
    }"#;

    let result = engine.load_graph(graph_json);
    assert!(result.is_ok(), "Failed to load graph: {:?}", result);
    assert_eq!(engine.list_graphs().len(), 1);
    assert!(engine.list_graphs().contains(&"test_graph".to_string()));
}

#[wasm_bindgen_test]
fn test_load_invalid_graph() {
    let mut engine = WasmEngine::new();
    let invalid_json = "{ invalid json }";

    let result = engine.load_graph(invalid_json);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_execute_nonexistent_graph() {
    let engine = WasmEngine::new();
    let result = engine.execute_graph("nonexistent");
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_load_and_list_multiple_graphs() {
    let mut engine = WasmEngine::new();

    let graph1 = r#"{
        "id": "graph1",
        "name": "Graph 1",
        "nodes": {},
        "connections": []
    }"#;

    let graph2 = r#"{
        "id": "graph2",
        "name": "Graph 2",
        "nodes": {},
        "connections": []
    }"#;

    engine.load_graph(graph1).expect("Failed to load graph1");
    engine.load_graph(graph2).expect("Failed to load graph2");

    let graphs = engine.list_graphs();
    assert_eq!(graphs.len(), 2);
    assert!(graphs.contains(&"graph1".to_string()));
    assert!(graphs.contains(&"graph2".to_string()));
}

#[wasm_bindgen_test]
fn test_execute_empty_graph() {
    let mut engine = WasmEngine::new();

    let graph_json = r#"{
        "id": "empty_graph",
        "name": "Empty Graph",
        "nodes": {},
        "connections": []
    }"#;

    engine.load_graph(graph_json).expect("Failed to load graph");
    let result = engine.execute_graph("empty_graph");
    assert!(
        result.is_ok(),
        "Failed to execute empty graph: {:?}",
        result
    );

    // Empty graph should return empty results
    let results_json = result.unwrap();
    assert!(results_json.contains("{}") || results_json.contains("[]"));
}

#[wasm_bindgen_test]
fn test_list_blocks_initially_empty() {
    let engine = WasmEngine::new();
    let blocks = engine.list_blocks();
    // Engine starts with no registered blocks (blocks are registered separately)
    assert_eq!(blocks.len(), 0);
}

#[wasm_bindgen_test]
fn test_multiple_engine_instances() {
    let mut engine1 = WasmEngine::new();
    let engine2 = WasmEngine::new();

    let graph_json = r#"{
        "id": "test",
        "name": "Test",
        "nodes": {},
        "connections": []
    }"#;

    engine1
        .load_graph(graph_json)
        .expect("Failed to load in engine1");

    // engine2 should not have the graph loaded in engine1
    assert_eq!(engine1.list_graphs().len(), 1);
    assert_eq!(engine2.list_graphs().len(), 0);
}

#[wasm_bindgen_test]
fn test_json_serialization_error_handling() {
    let engine = WasmEngine::new();

    // Try to execute a graph that doesn't exist
    let result = engine.execute_graph("does_not_exist");
    assert!(result.is_err());

    // The error message should be meaningful
    if let Err(err) = result {
        let err_str = format!("{:?}", err);
        assert!(!err_str.is_empty());
    }
}
