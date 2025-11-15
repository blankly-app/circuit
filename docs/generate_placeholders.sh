#!/bin/bash

# This script generates placeholder pages for all missing documentation files

create_placeholder() {
    local file=$1
    local title=$2

    if [ ! -f "$file" ]; then
        mkdir -p "$(dirname "$file")"
        cat > "$file" << EOF
# $title

> **Note**: This section is under development. Check back soon for detailed documentation.

## Overview

This page will cover $title in detail.

## Coming Soon

Detailed documentation for this topic is being written.

## In the Meantime

- Check out the [Getting Started](../getting-started/quick-start.md) guide
- Explore the [examples](../../examples/) directory
- Read the source code in the repository

## Contribute

Help us improve this documentation! If you'd like to contribute, please:

1. Fork the repository
2. Add your documentation
3. Submit a pull request

EOF
        echo "Created placeholder: $file"
    else
        echo "Skipped (exists): $file"
    fi
}

cd "$(dirname "$0")/src"

# Guide section
create_placeholder "guide/blocks.md" "Understanding Blocks"
create_placeholder "guide/flows.md" "Creating Flows"
create_placeholder "guide/builtin-blocks.md" "Built-in Blocks"
create_placeholder "guide/custom-blocks.md" "Creating Custom Blocks"
create_placeholder "guide/custom-blocks/block-files.md" "Using .block Files"
create_placeholder "guide/custom-blocks/rust-blocks.md" "Using Rust Code"
create_placeholder "guide/engine.md" "The Graph Engine"
create_placeholder "guide/values.md" "Values and Types"
create_placeholder "guide/errors.md" "Error Handling"

# Platform section
create_placeholder "platforms/overview.md" "Platform Overview"
create_placeholder "platforms/swift.md" "Swift (iOS/macOS)"
create_placeholder "platforms/kotlin.md" "Kotlin (Android)"
create_placeholder "platforms/react.md" "React (Web)"
create_placeholder "platforms/wasm.md" "WebAssembly"

# Advanced section
create_placeholder "advanced/building.md" "Building from Source"
create_placeholder "advanced/ffi.md" "FFI Integration"
create_placeholder "advanced/performance.md" "Performance Optimization"
create_placeholder "advanced/testing.md" "Testing Your Blocks"
create_placeholder "advanced/cross-compilation.md" "Cross-Compilation"

# Examples section
create_placeholder "examples/calculator.md" "Calculator Example"
create_placeholder "examples/data-pipeline.md" "Data Pipeline"
create_placeholder "examples/string-processing.md" "String Processing"

# API section
create_placeholder "api/core.md" "Core API"
create_placeholder "api/lang.md" "Language API"
create_placeholder "api/wasm.md" "WASM API"
create_placeholder "api/ffi.md" "FFI API"
create_placeholder "api/rustdoc.md" "Generated Rust Docs"

# Contributing section
create_placeholder "contributing/how-to.md" "How to Contribute"
create_placeholder "contributing/dev-setup.md" "Development Setup"
create_placeholder "contributing/style.md" "Code Style"

echo "Done generating placeholders!"
