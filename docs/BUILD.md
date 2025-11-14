# Build Configuration for Circuit

This document describes how to build Circuit for various target platforms.

## Cross-Compilation Setup

### iOS Targets

```bash
# Add iOS targets
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim

# Build for iOS device (ARM64)
cargo build --release --target aarch64-apple-ios -p circuit-ffi

# Build for iOS simulator (x86_64)
cargo build --release --target x86_64-apple-ios -p circuit-ffi

# Build for iOS simulator (ARM64, M1+)
cargo build --release --target aarch64-apple-ios-sim -p circuit-ffi

# Create universal library
lipo -create \
    target/aarch64-apple-ios/release/libcircuit_ffi.a \
    target/x86_64-apple-ios/release/libcircuit_ffi.a \
    -output libcircuit_ios.a
```

### macOS Targets

```bash
# Add macOS targets
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

# Build for macOS ARM64
cargo build --release --target aarch64-apple-darwin -p circuit-ffi

# Build for macOS x86_64
cargo build --release --target x86_64-apple-darwin -p circuit-ffi

# Create universal library
lipo -create \
    target/aarch64-apple-darwin/release/libcircuit_ffi.a \
    target/x86_64-apple-darwin/release/libcircuit_ffi.a \
    -output libcircuit_macos.a
```

### Android Targets

```bash
# Install Android NDK (if not already installed)
# Download from: https://developer.android.com/ndk/downloads

# Set environment variables
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/26.1.10909125

# Add Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Create cargo config for Android
mkdir -p .cargo
cat > .cargo/config.toml << 'EOF'
[target.aarch64-linux-android]
ar = "aarch64-linux-android-ar"
linker = "aarch64-linux-android-clang"

[target.armv7-linux-androideabi]
ar = "arm-linux-androideabi-ar"
linker = "armv7a-linux-androideabi-clang"

[target.i686-linux-android]
ar = "i686-linux-android-ar"
linker = "i686-linux-android-clang"

[target.x86_64-linux-android]
ar = "x86_64-linux-android-ar"
linker = "x86_64-linux-android-clang"
EOF

# Add NDK to PATH
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

# Build for Android
cargo build --release --target aarch64-linux-android -p circuit-ffi
cargo build --release --target armv7-linux-androideabi -p circuit-ffi
cargo build --release --target i686-linux-android -p circuit-ffi
cargo build --release --target x86_64-linux-android -p circuit-ffi
```

### WebAssembly

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Add wasm target
rustup target add wasm32-unknown-unknown

# Build for web
cd circuit-wasm
wasm-pack build --target web --release

# Build for bundler (webpack, rollup, etc.)
wasm-pack build --target bundler --release

# Build for Node.js
wasm-pack build --target nodejs --release
```

## CI/CD Configuration

### GitHub Actions

Create `.github/workflows/build.yml`:

```yaml
name: Build

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Run tests
        run: cargo test --all-features

  build-ios:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Add iOS targets
        run: |
          rustup target add aarch64-apple-ios
          rustup target add x86_64-apple-ios
      - name: Build for iOS
        run: |
          cargo build --release --target aarch64-apple-ios -p circuit-ffi
          cargo build --release --target x86_64-apple-ios -p circuit-ffi

  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Setup Android NDK
        uses: nttld/setup-ndk@v1
        with:
          ndk-version: r26b
      - name: Add Android targets
        run: |
          rustup target add aarch64-linux-android
          rustup target add armv7-linux-androideabi
      - name: Build for Android
        run: |
          cargo build --release --target aarch64-linux-android -p circuit-ffi
          cargo build --release --target armv7-linux-androideabi -p circuit-ffi

  build-wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      - name: Build WASM
        run: |
          cd circuit-wasm
          wasm-pack build --target web --release
```

## Optimization Flags

For production builds, add to `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

## Binary Size Optimization

For smaller binary sizes (important for mobile/web):

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable link-time optimization
codegen-units = 1   # Reduce parallel code generation
panic = "abort"     # Remove unwinding code
strip = true        # Strip symbols
```

Then build with:

```bash
cargo build --release -p circuit-ffi
```

For WASM, use:

```bash
cd circuit-wasm
wasm-pack build --target web --release
wasm-opt -Oz -o pkg/circuit_wasm_bg_optimized.wasm pkg/circuit_wasm_bg.wasm
```
