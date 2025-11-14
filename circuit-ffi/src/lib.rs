use circuit_core::{Engine, Graph};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Global engine registry
lazy_static::lazy_static! {
    static ref ENGINES: Mutex<HashMap<u64, Arc<Mutex<Engine>>>> = Mutex::new(HashMap::new());
    static ref NEXT_ENGINE_ID: Mutex<u64> = Mutex::new(0);
}

/// Create a new engine instance and return its handle
#[no_mangle]
pub extern "C" fn circuit_engine_create() -> u64 {
    let engine = Arc::new(Mutex::new(Engine::new()));
    let mut next_id = NEXT_ENGINE_ID.lock().unwrap();
    let id = *next_id;
    *next_id += 1;
    
    ENGINES.lock().unwrap().insert(id, engine);
    id
}

/// Destroy an engine instance
#[no_mangle]
pub extern "C" fn circuit_engine_destroy(handle: u64) {
    ENGINES.lock().unwrap().remove(&handle);
}

/// Load a graph from JSON string
/// Returns 0 on success, non-zero on error
#[no_mangle]
pub extern "C" fn circuit_load_graph(
    handle: u64,
    json: *const c_char,
    error_out: *mut *mut c_char,
) -> i32 {
    let json_str = unsafe {
        if json.is_null() {
            return -1;
        }
        match CStr::from_ptr(json).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        }
    };

    let engine_arc = {
        let engines = ENGINES.lock().unwrap();
        match engines.get(&handle) {
            Some(e) => Arc::clone(e),
            None => {
                set_error(error_out, "Invalid engine handle");
                return -1;
            }
        }
    };

    let graph: Graph = match serde_json::from_str(json_str) {
        Ok(g) => g,
        Err(e) => {
            set_error(error_out, &format!("Failed to parse graph: {}", e));
            return -1;
        }
    };

    let result = {
        let mut engine = engine_arc.lock().unwrap();
        engine.load_graph(graph)
    };

    match result {
        Ok(_) => 0,
        Err(e) => {
            set_error(error_out, &format!("Failed to load graph: {}", e));
            -1
        }
    }
}

/// Execute a graph and return results as JSON
/// Returns a C string that must be freed with circuit_free_string
#[no_mangle]
pub extern "C" fn circuit_execute_graph(
    handle: u64,
    graph_id: *const c_char,
    error_out: *mut *mut c_char,
) -> *mut c_char {
    let graph_id_str = unsafe {
        if graph_id.is_null() {
            return std::ptr::null_mut();
        }
        match CStr::from_ptr(graph_id).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        }
    };

    let engine_arc = {
        let engines = ENGINES.lock().unwrap();
        match engines.get(&handle) {
            Some(e) => Arc::clone(e),
            None => {
                set_error(error_out, "Invalid engine handle");
                return std::ptr::null_mut();
            }
        }
    };

    let results = {
        let engine = engine_arc.lock().unwrap();
        engine.execute_graph(graph_id_str)
    };

    let results = match results {
        Ok(r) => r,
        Err(e) => {
            set_error(error_out, &format!("Execution failed: {}", e));
            return std::ptr::null_mut();
        }
    };

    let json = match serde_json::to_string(&results) {
        Ok(j) => j,
        Err(e) => {
            set_error(error_out, &format!("Failed to serialize results: {}", e));
            return std::ptr::null_mut();
        }
    };

    match CString::new(json) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a string allocated by circuit_execute_graph
#[no_mangle]
pub extern "C" fn circuit_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

// Helper function to set error message
fn set_error(error_out: *mut *mut c_char, message: &str) {
    if !error_out.is_null() {
        if let Ok(c_str) = CString::new(message) {
            unsafe {
                *error_out = c_str.into_raw();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_lifecycle() {
        let handle = circuit_engine_create();
        circuit_engine_destroy(handle);
    }
}
