use circuit_core::blocks::core::ConstantBlock;
use circuit_core::blocks::math::AddBlock;
use circuit_core::graph::{Connection, Graph, Node};
use circuit_core::{BlockContext, Engine, Value};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::sync::Arc;

use circuit_core::block::Block;

/// Create a linear chain graph: const → add → add → ... → add
fn build_linear_chain(size: usize) -> (Engine, String) {
    let mut engine = Engine::new();
    engine.register_block(Arc::new(ConstantBlock)).unwrap();
    engine.register_block(Arc::new(AddBlock)).unwrap();

    let graph_id = format!("linear_{}", size);
    let mut graph = Graph::new(graph_id.clone(), format!("Linear chain of {}", size));

    // First constant node
    let mut config = HashMap::new();
    config.insert("value".to_string(), Value::Float(1.0));
    graph
        .add_node(Node {
            id: "const_0".to_string(),
            block_type: "core.constant".to_string(),
            config,
            position: None,
        })
        .unwrap();

    // Build chain of add nodes, each adding a constant
    for i in 0..size {
        // Constant for this step
        let mut config = HashMap::new();
        config.insert("value".to_string(), Value::Float(1.0));
        graph
            .add_node(Node {
                id: format!("const_{}", i + 1),
                block_type: "core.constant".to_string(),
                config,
                position: None,
            })
            .unwrap();

        // Add node
        graph
            .add_node(Node {
                id: format!("add_{}", i),
                block_type: "math.add".to_string(),
                config: HashMap::new(),
                position: None,
            })
            .unwrap();

        // Connect previous output to add.a
        if i == 0 {
            graph
                .add_connection(Connection {
                    from_node: "const_0".to_string(),
                    from_port: "value".to_string(),
                    to_node: "add_0".to_string(),
                    to_port: "a".to_string(),
                })
                .unwrap();
        } else {
            graph
                .add_connection(Connection {
                    from_node: format!("add_{}", i - 1),
                    from_port: "result".to_string(),
                    to_node: format!("add_{}", i),
                    to_port: "a".to_string(),
                })
                .unwrap();
        }

        // Connect constant to add.b
        graph
            .add_connection(Connection {
                from_node: format!("const_{}", i + 1),
                from_port: "value".to_string(),
                to_node: format!("add_{}", i),
                to_port: "b".to_string(),
            })
            .unwrap();
    }

    engine.load_graph(graph).unwrap();
    (engine, graph_id)
}

/// Create a wide graph: N independent chains of const → add
fn build_wide_graph(width: usize) -> (Engine, String) {
    let mut engine = Engine::new();
    engine.register_block(Arc::new(ConstantBlock)).unwrap();
    engine.register_block(Arc::new(AddBlock)).unwrap();

    let graph_id = format!("wide_{}", width);
    let mut graph = Graph::new(graph_id.clone(), format!("Wide graph of {}", width));

    for i in 0..width {
        // Two constants per chain
        let mut config_a = HashMap::new();
        config_a.insert("value".to_string(), Value::Float(i as f64));
        graph
            .add_node(Node {
                id: format!("const_a_{}", i),
                block_type: "core.constant".to_string(),
                config: config_a,
                position: None,
            })
            .unwrap();

        let mut config_b = HashMap::new();
        config_b.insert("value".to_string(), Value::Float(1.0));
        graph
            .add_node(Node {
                id: format!("const_b_{}", i),
                block_type: "core.constant".to_string(),
                config: config_b,
                position: None,
            })
            .unwrap();

        // Add node
        graph
            .add_node(Node {
                id: format!("add_{}", i),
                block_type: "math.add".to_string(),
                config: HashMap::new(),
                position: None,
            })
            .unwrap();

        graph
            .add_connection(Connection {
                from_node: format!("const_a_{}", i),
                from_port: "value".to_string(),
                to_node: format!("add_{}", i),
                to_port: "a".to_string(),
            })
            .unwrap();
        graph
            .add_connection(Connection {
                from_node: format!("const_b_{}", i),
                from_port: "value".to_string(),
                to_node: format!("add_{}", i),
                to_port: "b".to_string(),
            })
            .unwrap();
    }

    engine.load_graph(graph).unwrap();
    (engine, graph_id)
}

/// Build a graph without loading it (for load benchmarks)
fn build_unloaded_graph(size: usize) -> (Engine, Graph) {
    let mut engine = Engine::new();
    engine.register_block(Arc::new(ConstantBlock)).unwrap();
    engine.register_block(Arc::new(AddBlock)).unwrap();

    let mut graph = Graph::new(format!("load_test_{}", size), format!("Load test {}", size));

    // Create a chain of add nodes
    let mut config = HashMap::new();
    config.insert("value".to_string(), Value::Float(1.0));
    graph
        .add_node(Node {
            id: "const_0".to_string(),
            block_type: "core.constant".to_string(),
            config,
            position: None,
        })
        .unwrap();

    for i in 0..size {
        let mut config = HashMap::new();
        config.insert("value".to_string(), Value::Float(1.0));
        graph
            .add_node(Node {
                id: format!("const_{}", i + 1),
                block_type: "core.constant".to_string(),
                config,
                position: None,
            })
            .unwrap();

        graph
            .add_node(Node {
                id: format!("add_{}", i),
                block_type: "math.add".to_string(),
                config: HashMap::new(),
                position: None,
            })
            .unwrap();

        if i == 0 {
            graph
                .add_connection(Connection {
                    from_node: "const_0".to_string(),
                    from_port: "value".to_string(),
                    to_node: "add_0".to_string(),
                    to_port: "a".to_string(),
                })
                .unwrap();
        } else {
            graph
                .add_connection(Connection {
                    from_node: format!("add_{}", i - 1),
                    from_port: "result".to_string(),
                    to_node: format!("add_{}", i),
                    to_port: "a".to_string(),
                })
                .unwrap();
        }

        graph
            .add_connection(Connection {
                from_node: format!("const_{}", i + 1),
                from_port: "value".to_string(),
                to_node: format!("add_{}", i),
                to_port: "b".to_string(),
            })
            .unwrap();
    }

    (engine, graph)
}

/// Build a graph for topological sort benchmarks (no engine needed)
fn build_topo_graph(size: usize) -> Graph {
    let mut graph = Graph::new(format!("topo_{}", size), format!("Topo test {}", size));

    for i in 0..size {
        graph
            .add_node(Node {
                id: format!("node_{}", i),
                block_type: "test".to_string(),
                config: HashMap::new(),
                position: None,
            })
            .unwrap();

        if i > 0 {
            graph
                .add_connection(Connection {
                    from_node: format!("node_{}", i - 1),
                    from_port: "out".to_string(),
                    to_node: format!("node_{}", i),
                    to_port: "in".to_string(),
                })
                .unwrap();
        }
    }

    graph
}

fn bench_single_block_execution(c: &mut Criterion) {
    let block = AddBlock;
    let mut context = BlockContext::new();
    context.inputs.insert("a".to_string(), Value::Float(5.0));
    context.inputs.insert("b".to_string(), Value::Float(3.0));

    c.bench_function("single_block_execution", |b| {
        b.iter(|| {
            let ctx = black_box(context.clone());
            black_box(block.execute(ctx).unwrap());
        })
    });
}

fn bench_graph_execution_linear(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_execution/linear_chain");

    for size in [3, 10, 50, 100] {
        let (engine, graph_id) = build_linear_chain(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(engine.execute_graph(&graph_id).unwrap());
            })
        });
    }

    group.finish();
}

fn bench_graph_execution_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_execution/wide");

    for width in [10, 50] {
        let (engine, graph_id) = build_wide_graph(width);

        group.bench_with_input(BenchmarkId::from_parameter(width), &width, |b, _| {
            b.iter(|| {
                black_box(engine.execute_graph(&graph_id).unwrap());
            })
        });
    }

    group.finish();
}

fn bench_graph_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_loading");

    for size in [10, 50, 100] {
        let (base_engine, graph) = build_unloaded_graph(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                let mut engine = Engine::new();
                // Re-register blocks (engine doesn't clone)
                engine.register_block(Arc::new(ConstantBlock)).unwrap();
                engine.register_block(Arc::new(AddBlock)).unwrap();
                engine.load_graph(black_box(graph.clone())).unwrap();
            })
        });

        // Suppress unused variable warning
        let _ = &base_engine;
    }

    group.finish();
}

fn bench_topological_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("topological_sort");

    for size in [10, 50, 100, 500] {
        let graph = build_topo_graph(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(graph.topological_sort().unwrap());
            })
        });
    }

    group.finish();
}

fn bench_flow_parse_and_execute(c: &mut Criterion) {
    let source = r#"
        flow bench_test {
            description "Benchmark flow"

            node a: core.constant {
                value = 10
            }
            node b: core.constant {
                value = 20
            }
            node c: core.constant {
                value = 2
            }

            node sum: math.add
            node product: math.multiply

            connect a.value -> sum.a
            connect b.value -> sum.b
            connect sum.result -> product.a
            connect c.value -> product.b

            output product.result
        }
    "#;

    c.bench_function("flow_parse_and_execute", |b| {
        b.iter(|| {
            let flow = circuit_lang::parse_flow(black_box(source)).unwrap();
            let graph = circuit_lang::flow_to_graph(&flow).unwrap();

            let mut engine = Engine::new();
            engine.register_block(Arc::new(ConstantBlock)).unwrap();
            engine.register_block(Arc::new(AddBlock)).unwrap();
            engine
                .register_block(Arc::new(circuit_core::blocks::math::MultiplyBlock))
                .unwrap();
            engine.load_graph(graph).unwrap();
            black_box(engine.execute_graph("bench_test").unwrap());
        })
    });
}

criterion_group!(
    benches,
    bench_single_block_execution,
    bench_graph_execution_linear,
    bench_graph_execution_wide,
    bench_graph_loading,
    bench_topological_sort,
    bench_flow_parse_and_execute,
);
criterion_main!(benches);
