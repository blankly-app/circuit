# Circuit FFI Bindings for Kotlin (Android)

This document describes how to use Circuit in Kotlin/Android applications.

## Building the Library

Build the shared library for Android targets:

```bash
# Add Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Build for Android ARM64
cargo build --release --target aarch64-linux-android

# Build for Android ARMv7
cargo build --release --target armv7-linux-androideabi

# Build for Android x86_64
cargo build --release --target x86_64-linux-android
```

## Kotlin Integration

### 1. Add JNI Wrapper

Create a JNI wrapper in `src/main/cpp/circuit_jni.cpp`:

```cpp
#include <jni.h>
#include <string>

extern "C" {
    // Circuit FFI functions
    uint64_t circuit_engine_create();
    void circuit_engine_destroy(uint64_t handle);
    int32_t circuit_load_graph(uint64_t handle, const char* json, char** error);
    char* circuit_execute_graph(uint64_t handle, const char* graph_id, char** error);
    void circuit_free_string(char* s);
}

extern "C" JNIEXPORT jlong JNICALL
Java_com_example_circuit_CircuitEngine_nativeCreate(JNIEnv* env, jobject /* this */) {
    return (jlong)circuit_engine_create();
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_circuit_CircuitEngine_nativeDestroy(JNIEnv* env, jobject /* this */, jlong handle) {
    circuit_engine_destroy((uint64_t)handle);
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_circuit_CircuitEngine_nativeLoadGraph(
    JNIEnv* env, jobject /* this */, jlong handle, jstring json) {
    const char* jsonStr = env->GetStringUTFChars(json, nullptr);
    char* error = nullptr;
    
    int32_t result = circuit_load_graph((uint64_t)handle, jsonStr, &error);
    
    env->ReleaseStringUTFChars(json, jsonStr);
    
    if (result != 0) {
        jclass exceptionClass = env->FindClass("java/lang/RuntimeException");
        if (error) {
            env->ThrowNew(exceptionClass, error);
            circuit_free_string(error);
        } else {
            env->ThrowNew(exceptionClass, "Failed to load graph");
        }
    }
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_example_circuit_CircuitEngine_nativeExecuteGraph(
    JNIEnv* env, jobject /* this */, jlong handle, jstring graphId) {
    const char* graphIdStr = env->GetStringUTFChars(graphId, nullptr);
    char* error = nullptr;
    
    char* result = circuit_execute_graph((uint64_t)handle, graphIdStr, &error);
    
    env->ReleaseStringUTFChars(graphId, graphIdStr);
    
    if (result) {
        jstring jresult = env->NewStringUTF(result);
        circuit_free_string(result);
        return jresult;
    }
    
    jclass exceptionClass = env->FindClass("java/lang/RuntimeException");
    if (error) {
        env->ThrowNew(exceptionClass, error);
        circuit_free_string(error);
    } else {
        env->ThrowNew(exceptionClass, "Failed to execute graph");
    }
    
    return nullptr;
}
```

### 2. Kotlin Wrapper Class

```kotlin
package com.example.circuit

class CircuitEngine {
    private var handle: Long = 0
    
    init {
        System.loadLibrary("circuit_ffi")
        handle = nativeCreate()
    }
    
    fun loadGraph(json: String) {
        nativeLoadGraph(handle, json)
    }
    
    fun executeGraph(graphId: String): String {
        return nativeExecuteGraph(handle, graphId)
    }
    
    fun close() {
        if (handle != 0L) {
            nativeDestroy(handle)
            handle = 0
        }
    }
    
    protected fun finalize() {
        close()
    }
    
    private external fun nativeCreate(): Long
    private external fun nativeDestroy(handle: Long)
    private external fun nativeLoadGraph(handle: Long, json: String)
    private external fun nativeExecuteGraph(handle: Long, graphId: String): String
}
```

### 3. Usage Example

```kotlin
val engine = CircuitEngine()

val graphJson = """
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

try {
    engine.loadGraph(graphJson)
    val result = engine.executeGraph("test")
    println("Result: $result")
} catch (e: Exception) {
    println("Error: ${e.message}")
} finally {
    engine.close()
}
```

### 4. Android CMakeLists.txt

```cmake
cmake_minimum_required(VERSION 3.18.1)
project("circuit_android")

add_library(circuit_jni SHARED circuit_jni.cpp)

# Add the pre-built Circuit FFI library
add_library(circuit_ffi STATIC IMPORTED)
set_target_properties(circuit_ffi PROPERTIES
    IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/../jniLibs/${ANDROID_ABI}/libcircuit_ffi.a)

target_link_libraries(circuit_jni
    circuit_ffi
    log)
```

### 5. Gradle Configuration

```gradle
android {
    ...
    externalNativeBuild {
        cmake {
            path "src/main/cpp/CMakeLists.txt"
            version "3.18.1"
        }
    }
    
    defaultConfig {
        ...
        ndk {
            abiFilters 'arm64-v8a', 'armeabi-v7a', 'x86_64'
        }
    }
}
```
