use circuit_lang::*;
use std::fs;
use std::path::PathBuf;

fn get_example_path(relative_path: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up to circuit root
    path.push("examples");
    path.push(relative_path);
    path
}

#[test]
fn test_load_all_block_examples() {
    let block_files = vec![
        "blocks/math.square.block",
        "blocks/math.power.block",
        "blocks/string.format.block",
        "blocks/logic.compare.block",
    ];

    for file in block_files {
        let path = get_example_path(file);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

        let block = parse_block(&source)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e));

        // Verify basic structure
        assert!(!block.name.is_empty(), "Block name should not be empty in {}", file);
        println!("✓ Loaded block: {} from {}", block.name, file);
    }
}

#[test]
fn test_load_all_flow_examples() {
    let flow_files = vec![
        "flows/calculator.flow",
        "flows/string_processing.flow",
        "flows/data_pipeline.flow",
    ];

    for file in flow_files {
        let path = get_example_path(file);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

        let flow = parse_flow(&source)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e));

        // Verify basic structure
        assert!(!flow.name.is_empty(), "Flow name should not be empty in {}", file);
        assert!(!flow.nodes.is_empty(), "Flow should have nodes in {}", file);
        println!("✓ Loaded flow: {} ({} nodes) from {}", flow.name, flow.nodes.len(), file);
    }
}

#[test]
fn test_math_square_block() {
    let path = get_example_path("blocks/math.square.block");
    let source = fs::read_to_string(&path).expect("Failed to read file");
    let block = parse_block(&source).expect("Failed to parse");

    assert_eq!(block.name, "math.square");
    assert_eq!(block.inputs.len(), 1);
    assert_eq!(block.inputs[0].name, "x");
    assert_eq!(block.outputs.len(), 1);
    assert_eq!(block.outputs[0].name, "result");
    assert!(block.execute.is_some());
}

#[test]
fn test_calculator_flow() {
    let path = get_example_path("flows/calculator.flow");
    let source = fs::read_to_string(&path).expect("Failed to read file");
    let flow = parse_flow(&source).expect("Failed to parse");

    assert_eq!(flow.name, "calculator");
    assert!(flow.description.is_some());
    assert_eq!(flow.nodes.len(), 6); // 3 constants + add + multiply + debug
    assert!(flow.connections.len() >= 2);
}

#[test]
fn test_flow_to_graph_conversion() {
    let path = get_example_path("flows/calculator.flow");
    let source = fs::read_to_string(&path).expect("Failed to read file");
    let flow = parse_flow(&source).expect("Failed to parse");
    let graph = flow_to_graph(&flow).expect("Failed to convert to graph");

    assert_eq!(graph.nodes.len(), flow.nodes.len());
    assert_eq!(graph.connections.len(), flow.connections.len());

    // Verify specific nodes exist
    assert!(graph.nodes.contains_key("const5"));
    assert!(graph.nodes.contains_key("const3"));
    assert!(graph.nodes.contains_key("add"));
    assert!(graph.nodes.contains_key("multiply"));
}

#[test]
fn test_string_processing_flow() {
    let path = get_example_path("flows/string_processing.flow");
    let source = fs::read_to_string(&path).expect("Failed to read file");
    let flow = parse_flow(&source).expect("Failed to parse");

    assert_eq!(flow.name, "string_processing");
    assert!(flow.nodes.iter().any(|n| n.block_type == "string.concat"));
}

#[test]
fn test_data_pipeline_flow() {
    let path = get_example_path("flows/data_pipeline.flow");
    let source = fs::read_to_string(&path).expect("Failed to read file");
    let flow = parse_flow(&source).expect("Failed to parse");

    assert_eq!(flow.name, "data_pipeline");

    // Verify the pipeline structure
    assert!(flow.nodes.iter().any(|n| n.id == "stage1"));
    assert!(flow.nodes.iter().any(|n| n.id == "stage2"));
    assert!(flow.nodes.iter().any(|n| n.id == "stage3"));
}

#[test]
fn test_block_with_config() {
    let path = get_example_path("blocks/math.power.block");
    let source = fs::read_to_string(&path).expect("Failed to read file");
    let block = parse_block(&source).expect("Failed to parse");

    assert_eq!(block.name, "math.power");
    assert_eq!(block.inputs.len(), 2);

    // Check for default value
    let exponent_input = block.inputs.iter().find(|i| i.name == "exponent").expect("exponent input not found");
    assert!(exponent_input.default.is_some());
}

#[test]
fn test_block_with_multiple_outputs() {
    let path = get_example_path("blocks/logic.compare.block");
    let source = fs::read_to_string(&path).expect("Failed to read file");
    let block = parse_block(&source).expect("Failed to parse");

    assert_eq!(block.name, "logic.compare");
    assert_eq!(block.outputs.len(), 3);

    let output_names: Vec<String> = block.outputs.iter().map(|o| o.name.clone()).collect();
    assert!(output_names.contains(&"equal".to_string()));
    assert!(output_names.contains(&"greater".to_string()));
    assert!(output_names.contains(&"less".to_string()));
}
