# Circuit FFI Bindings for Swift (iOS/macOS)

This document describes how to use Circuit in Swift applications.

## Building the Library

Build the static library for your target platform:

```bash
# For iOS (ARM64)
cargo build --release --target aarch64-apple-ios

# For iOS Simulator (x86_64)
cargo build --release --target x86_64-apple-ios

# For macOS (ARM64)
cargo build --release --target aarch64-apple-darwin

# For macOS (x86_64)
cargo build --release --target x86_64-apple-darwin
```

## Swift Integration

### 1. Add the Library to Your Xcode Project

1. Copy the built `.a` file to your project
2. Add it to your target's "Link Binary With Libraries" build phase
3. Create a bridging header if needed

### 2. Swift Wrapper

```swift
import Foundation

class CircuitEngine {
    private var handle: UInt64
    
    init() {
        self.handle = circuit_engine_create()
    }
    
    deinit {
        circuit_engine_destroy(handle)
    }
    
    func loadGraph(json: String) throws {
        var error: UnsafeMutablePointer<CChar>? = nil
        let result = json.withCString { jsonPtr in
            circuit_load_graph(handle, jsonPtr, &error)
        }
        
        if result != 0 {
            if let errorPtr = error {
                let errorMsg = String(cString: errorPtr)
                circuit_free_string(errorPtr)
                throw CircuitError.loadFailed(errorMsg)
            }
            throw CircuitError.loadFailed("Unknown error")
        }
    }
    
    func executeGraph(id: String) throws -> String {
        var error: UnsafeMutablePointer<CChar>? = nil
        let resultPtr = id.withCString { idPtr in
            circuit_execute_graph(handle, idPtr, &error)
        }
        
        if let resultPtr = resultPtr {
            let result = String(cString: resultPtr)
            circuit_free_string(resultPtr)
            return result
        }
        
        if let errorPtr = error {
            let errorMsg = String(cString: errorPtr)
            circuit_free_string(errorPtr)
            throw CircuitError.executionFailed(errorMsg)
        }
        
        throw CircuitError.executionFailed("Unknown error")
    }
}

enum CircuitError: Error {
    case loadFailed(String)
    case executionFailed(String)
}

// C FFI declarations
@_silgen_name("circuit_engine_create")
func circuit_engine_create() -> UInt64

@_silgen_name("circuit_engine_destroy")
func circuit_engine_destroy(_ handle: UInt64)

@_silgen_name("circuit_load_graph")
func circuit_load_graph(_ handle: UInt64, _ json: UnsafePointer<CChar>, _ error: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> Int32

@_silgen_name("circuit_execute_graph")
func circuit_execute_graph(_ handle: UInt64, _ graphId: UnsafePointer<CChar>, _ error: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?) -> UnsafeMutablePointer<CChar>?

@_silgen_name("circuit_free_string")
func circuit_free_string(_ s: UnsafeMutablePointer<CChar>?)
```

### 3. Usage Example

```swift
let engine = CircuitEngine()

let graphJson = """
{
    "id": "test",
    "name": "Test Graph",
    "nodes": {
        "node1": {
            "id": "node1",
            "block_type": "core.constant",
            "config": {
                "value": {"type": "Float", "value": 5.0}
            },
            "position": null
        }
    },
    "connections": []
}
"""

do {
    try engine.loadGraph(json: graphJson)
    let result = try engine.executeGraph(id: "test")
    print("Result: \\(result)")
} catch {
    print("Error: \\(error)")
}
```

## Cross-Compilation

To build a universal library for iOS:

```bash
# Build for all iOS targets
cargo build --release --target aarch64-apple-ios
cargo build --release --target x86_64-apple-ios

# Create a universal library
lipo -create \
    target/aarch64-apple-ios/release/libcircuit_ffi.a \
    target/x86_64-apple-ios/release/libcircuit_ffi.a \
    -output libcircuit_universal.a
```
