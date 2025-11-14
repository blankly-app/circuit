use thiserror::Error;

/// Result type alias for Circuit operations
pub type Result<T> = std::result::Result<T, CircuitError>;

/// Errors that can occur in the Circuit engine
#[derive(Debug, Error)]
pub enum CircuitError {
    #[error("Block execution error: {0}")]
    BlockExecution(String),

    #[error("Graph error: {0}")]
    Graph(String),

    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Invalid connection: {0}")]
    InvalidConnection(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Cycle detected in graph")]
    CycleDetected,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}
