use wasm_bindgen::prelude::*;
use circuit_core::{Engine, Graph};
use std::sync::{Arc, Mutex};

/// WASM wrapper for the Circuit engine
#[wasm_bindgen]
pub struct WasmEngine {
    engine: Arc<Mutex<Engine>>,
}

#[wasm_bindgen]
impl WasmEngine {
    /// Create a new WASM engine instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEngine {
        WasmEngine {
            engine: Arc::new(Mutex::new(Engine::new())),
        }
    }

    /// Load a graph from JSON
    #[wasm_bindgen(js_name = loadGraph)]
    pub fn load_graph(&mut self, graph_json: &str) -> Result<(), JsValue> {
        let graph: Graph = serde_json::from_str(graph_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse graph: {}", e)))?;

        self.engine
            .lock()
            .unwrap()
            .load_graph(graph)
            .map_err(|e| JsValue::from_str(&format!("Failed to load graph: {}", e)))
    }

    /// Execute a graph by ID and return results as JSON
    #[wasm_bindgen(js_name = executeGraph)]
    pub fn execute_graph(&self, graph_id: &str) -> Result<String, JsValue> {
        let results = self
            .engine
            .lock()
            .unwrap()
            .execute_graph(graph_id)
            .map_err(|e| JsValue::from_str(&format!("Execution failed: {}", e)))?;

        serde_json::to_string(&results)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize results: {}", e)))
    }

    /// List registered block types
    #[wasm_bindgen(js_name = listBlocks)]
    pub fn list_blocks(&self) -> Vec<String> {
        self.engine.lock().unwrap().list_blocks()
    }

    /// List loaded graphs
    #[wasm_bindgen(js_name = listGraphs)]
    pub fn list_graphs(&self) -> Vec<String> {
        self.engine.lock().unwrap().list_graphs()
    }
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages in the browser
    // Note: To enable, add console_error_panic_hook feature and dependency
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_wasm_engine_creation() {
        let engine = WasmEngine::new();
        assert_eq!(engine.list_graphs().len(), 0);
    }
}
