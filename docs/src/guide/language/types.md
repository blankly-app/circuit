# Type System

Circuit supports a rich type system for data flowing through graphs.

## Supported Types

| Type | Description | Example Values |
|------|-------------|----------------|
| `Number` | Floating-point numbers | `42`, `3.14`, `-10.5` |
| `String` | Text strings | `"hello"`, `"world"` |
| `Bool` | Boolean values | `true`, `false` |
| `Array` | Ordered lists | `[1, 2, 3]`, `["a", "b"]` |
| `Object` | Key-value maps | `{"key": "value"}` |
| `Bytes` | Binary data | Raw byte arrays |
| `Any` | Any type | Any of the above |
| `Null` | Null value | `null` |

## Value Literals

### Null

```
null
```

### Booleans

```
true
false
```

### Numbers

Circuit uses 64-bit floating-point numbers internally:

```
42
3.14159
-10.5
0.0
1e6      // Scientific notation
```

### Strings

Double-quoted text:

```
"hello world"
"multi word string"
"with \"escaped\" quotes"
"line 1\nline 2"   // Escape sequences
```

### Arrays

Ordered collections of values:

```
[1, 2, 3]
["a", "b", "c"]
[true, false, true]
[]   // Empty array
```

Arrays can contain mixed types:

```
[1, "two", true, null]
```

### Objects

Key-value pairs (like JSON objects):

```
{"name": "Alice", "age": 30}
{"x": 10, "y": 20}
{}   // Empty object
```

### Nested Structures

Arrays and objects can be nested:

```
{
    "user": "bob",
    "scores": [10, 20, 30],
    "metadata": {
        "active": true,
        "level": 5
    }
}
```

## Type Annotations

In block and flow definitions, specify types:

```
input count: Number
input name: String
input flag: Bool
input items: Array
input data: Object
input anything: Any
```

## Rust Value Type

In Rust code, all values use the `Value` enum:

```rust
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Bytes(Vec<u8>),
}
```

### Creating Values

```rust
use circuit_core::Value;

let num = Value::Int(42);
let text = Value::String("hello".to_string());
let flag = Value::Bool(true);
let list = Value::Array(vec![
    Value::Int(1),
    Value::Int(2),
    Value::Int(3),
]);
let obj = Value::Object([
    ("key".to_string(), Value::String("value".to_string())),
].iter().cloned().collect());
```

### Extracting Values

```rust
// Safe extraction with Option
if let Some(num) = value.as_int() {
    println!("Integer: {}", num);
}

if let Some(text) = value.as_str() {
    println!("String: {}", text);
}

// Check type
if value.is_null() {
    println!("Value is null");
}
```

### Type Conversion

```rust
// Convert to JSON
let json = serde_json::to_string(&value)?;

// Parse from JSON
let value: Value = serde_json::from_str(&json)?;
```

## Type Checking (Future)

Currently, Circuit performs minimal type checking. Future versions will include:

- **Compile-time type validation**: Catch type errors before execution
- **Type inference**: Automatically infer types from connections
- **Custom types**: Define your own data structures
- **Generics**: Blocks that work with multiple types

## Best Practices

### 1. Use Specific Types

Prefer specific types over `Any`:

```
// Good
input count: Number

// Less good
input count: Any
```

### 2. Document Expected Formats

For `Object` types, document the expected structure:

```
input config: Object {
    description "Config object with fields: timeout (Number), retries (Number)"
}
```

### 3. Validate in Execute

Add validation in your block's execute method:

```rust
fn execute(&self, context: BlockContext) -> Result<HashMap<String, Value>> {
    let count = context.get_input("count")
        .and_then(|v| v.as_int())
        .ok_or_else(|| CircuitError::InvalidInput("count must be a number".into()))?;

    if count < 0 {
        return Err(CircuitError::InvalidInput("count must be positive".into()));
    }

    // ... rest of logic
}
```

### 4. Handle Null Values

Check for null before processing:

```rust
match context.get_input("optional_value") {
    Some(Value::Null) | None => {
        // Use default
    }
    Some(value) => {
        // Process value
    }
}
```

## Type Coercion

Circuit performs automatic type coercion in some cases:

- `Int` → `Float`: Automatic
- `Float` → `Int`: Truncates (may lose precision)
- `Bool` → `Number`: `true` = 1, `false` = 0
- Any type → `String`: Via `to_string()`

**Example:**

```rust
// These are equivalent internally
Value::Int(42) == Value::Float(42.0)
```

## JSON Compatibility

All Circuit values are JSON-compatible:

```rust
use serde_json;

let value = Value::Object(/* ... */);

// Serialize
let json = serde_json::to_string(&value)?;

// Deserialize
let value: Value = serde_json::from_str(&json)?;
```

This makes it easy to:
- Store graphs and values
- Transfer data across platforms
- Integrate with web APIs

## Platform-Specific Types

### Swift

```swift
// Circuit values map to Swift types
Int64 -> Int
Double -> Double
String -> String
Array -> [Any]
Dictionary -> [String: Any]
```

### Kotlin

```kotlin
// Circuit values map to Kotlin types
Int64 -> Long
Double -> Double
String -> String
Array -> List<Any>
Object -> Map<String, Any>
```

### JavaScript/TypeScript

```typescript
// Circuit values map to JS types
Int64 -> number
Float64 -> number
String -> string
Array -> Array<any>
Object -> Record<string, any>
Bool -> boolean
Null -> null
```

## See Also

- [Values and Types Guide](../values.md) - Practical usage
- [Block Syntax](./block-syntax.md) - Using types in blocks
- [API Reference](../../api/core.md) - Complete Value API
