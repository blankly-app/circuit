# Installation

This guide covers installing Circuit for different platforms and use cases.

## Rust Development

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:

```bash
rustc --version
cargo --version
```

### Clone Circuit

```bash
git clone https://github.com/blankly-app/circuit.git
cd circuit
```

### Build All Packages

```bash
# Build all packages in release mode
cargo build --release

# Run tests to verify everything works
cargo test

# Run an example
cargo run --example calculator
```

## Platform-Specific Installation

### iOS/macOS (Swift)

#### Install Additional Targets

```bash
# For iOS devices (ARM64)
rustup target add aarch64-apple-ios

# For iOS Simulator (x86_64)
rustup target add x86_64-apple-ios

# For iOS Simulator (ARM64, M1/M2 Macs)
rustup target add aarch64-apple-ios-sim

# For macOS
rustup target add aarch64-apple-darwin  # Apple Silicon
rustup target add x86_64-apple-darwin   # Intel Macs
```

#### Build for iOS

```bash
# Build the FFI library for iOS
cargo build --release --target aarch64-apple-ios -p circuit-ffi

# The library will be at: target/aarch64-apple-ios/release/libcircuit_ffi.a
```

See the [Swift Integration Guide](../platforms/swift.md) for detailed Xcode setup.

### Android (Kotlin)

#### Install NDK and Targets

1. Install Android NDK via Android Studio or standalone

2. Add Rust targets:

```bash
rustup target add aarch64-linux-android   # ARM64
rustup target add armv7-linux-androideabi # ARMv7
rustup target add i686-linux-android      # x86
rustup target add x86_64-linux-android    # x86_64
```

#### Configure Cargo for Android

Create or edit `~/.cargo/config.toml`:

```toml
[target.aarch64-linux-android]
ar = "/path/to/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "/path/to/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang"

[target.armv7-linux-androideabi]
ar = "/path/to/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "/path/to/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi30-clang"
```

Replace `/path/to/ndk` with your NDK installation path.

#### Build for Android

```bash
# Build for ARM64 (most common)
cargo build --release --target aarch64-linux-android -p circuit-ffi

# The library will be at: target/aarch64-linux-android/release/libcircuit_ffi.so
```

See the [Kotlin Integration Guide](../platforms/kotlin.md) for Android Studio setup.

### Web/React (WebAssembly)

#### Install wasm-pack

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Or with cargo:

```bash
cargo install wasm-pack
```

#### Build WASM Package

```bash
cd circuit-wasm
wasm-pack build --target web --release

# Or for bundlers (webpack, vite, etc.)
wasm-pack build --target bundler --release
```

This creates a `pkg/` directory with:
- `circuit_wasm_bg.wasm` - The WebAssembly module
- `circuit_wasm.js` - JavaScript bindings
- `circuit_wasm.d.ts` - TypeScript definitions
- `package.json` - NPM package manifest

#### Install in Your Web Project

```bash
cd your-web-project
npm install ../circuit/circuit-wasm/pkg
```

Or publish to NPM and install normally.

See the [React Integration Guide](../platforms/react.md) for detailed web setup.

## Development Tools

### Optional: Install cargo-watch for Auto-rebuild

```bash
cargo install cargo-watch
```

Use it during development:

```bash
cargo watch -x test
cargo watch -x 'run --example calculator'
```

### Optional: Install cargo-expand for Macro Debugging

```bash
cargo install cargo-expand
```

Useful for inspecting generated code:

```bash
cargo expand -p circuit-core
```

### Optional: Install mdBook for Documentation

```bash
cargo install mdbook
```

Build and serve the documentation locally:

```bash
cd docs
mdbook serve --open
```

## Verification

### Run All Tests

```bash
cargo test --all
```

### Build All Targets

```bash
# Native
cargo build --release

# iOS
cargo build --release --target aarch64-apple-ios -p circuit-ffi

# Android
cargo build --release --target aarch64-linux-android -p circuit-ffi

# WASM
cd circuit-wasm && wasm-pack build --target web
```

### Run Examples

```bash
cargo run --example calculator
```

## Troubleshooting

### Linker Errors on macOS

If you encounter linker errors, ensure you have Xcode command line tools:

```bash
xcode-select --install
```

### Android NDK Not Found

Ensure `ANDROID_NDK_HOME` is set:

```bash
export ANDROID_NDK_HOME=/path/to/android-ndk
```

Add to your `.bashrc` or `.zshrc` for persistence.

### WASM Build Fails

Ensure you have the wasm32 target:

```bash
rustup target add wasm32-unknown-unknown
```

And install wasm-bindgen-cli:

```bash
cargo install wasm-bindgen-cli
```

## Next Steps

- [Quick Start](./quick-start.md) - Create your first Circuit application
- [Architecture Overview](./architecture.md) - Understand how Circuit works
- [Platform Integration](../platforms/overview.md) - Integrate with your platform
