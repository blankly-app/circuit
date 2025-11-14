//! Circuit - A node-based runtime engine for building apps with blocks
//!
//! Circuit is a flexible, cross-platform runtime engine that allows you to create
//! applications using a node-based architecture. It supports execution on multiple
//! platforms including Swift (iOS/macOS), Kotlin (Android), and Web (via WebAssembly).
//!
//! # Core Concepts
//!
//! - **Blocks**: Individual computational units that process data
//! - **Nodes**: Instances of blocks in an execution graph
//! - **Graph**: The execution flow connecting nodes together
//! - **Engine**: The runtime that executes the graph

pub mod block;
pub mod blocks;
pub mod engine;
pub mod error;
pub mod graph;
pub mod value;

pub use block::{Block, BlockContext, BlockMetadata};
pub use engine::Engine;
pub use error::{CircuitError, Result};
pub use graph::{Graph, NodeId};
pub use value::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_workflow() {
        let engine = Engine::new();
        assert!(engine.graphs.is_empty());
    }
}
