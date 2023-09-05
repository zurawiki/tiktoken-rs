#!/bin/sh

# Install the wasm32 rust build target
rustup target add wasm32-unknown-unknown

# Install the toml-cli
cargo install toml-cli

# Install wasm-snip
cargo install wasm-snip

# Install wasm-bindgen
cargo install wasm-bindgen-cli

# Install wasm-tools
cargo install wasm-tools

# Create the temporary directory
rm -Rf "$1"
mkdir "$1"
cp "Cargo.toml" "$1"/Cargo.toml

# Ensure the module at {{dir}} has the crate-type = ["cdylib"]
toml set "$1"/Cargo.toml lib.crate-type ["cdylib"] > "$1"/Cargo-local.toml && \
  mv "$1"/Cargo.toml "$1"/Cargo-bak.toml && \
  mv "$1"/Cargo-local.toml "$1"/Cargo.toml

# Clean up artifacts left by the toml CLI program ("["cdylib"]" -> ["cdylib"])
sed -i .bak 's/"\[cdylib\]"/\["cdylib"\]/g' "$1"/Cargo.toml && \
  rm -rf "$1"/Cargo.toml.bak

# Ensure the package name = "module"
toml set "$1"/Cargo.toml package.name "module" > "$1"/Cargo-local.toml && \
  rm -rf "$1"/Cargo.toml && \
  mv "$1"/Cargo-local.toml "$1"/Cargo.toml

# Ensure the Wasm module is configured to use imported memory
export RUSTFLAGS="-C link-arg=-z -C link-arg=stack-size=65536 -C link-arg=--import-memory"

# Build the module
cargo build --manifest-path Cargo.toml \
  --target wasm32-unknown-unknown --release

# Replace the modified Cargo.toml with the backup
rm -rf "$1"/Cargo.toml && mv "$1"/Cargo-bak.toml "$1"/Cargo.toml

# Enable the "WASM_INTERFACE_TYPES" feature, which will remove the __wbindgen_throw import.
# See: https://github.com/rustwasm/wasm-bindgen/blob/7f4663b70bd492278bf0e7bba4eeddb3d840c868/crates/cli-support/src/lib.rs#L397-L403
export WASM_INTERFACE_TYPES=1

# Run wasm-bindgen over the module, replacing all placeholder __wbindgen_... imports
wasm-bindgen ../../target/wasm32-unknown-unknown/release/module.wasm --out-dir "$1" --out-name bg_module.wasm

# Run wasm-tools strip to remove the wasm-interface-types custom section
wasm-tools strip "$1"/bg_module.wasm -d wasm-interface-types -o "$1"/strip_module.wasm
rm -rf "$1"/bg_module.wasm

# Run wasm-snip to trip down the size of the binary, removing any dead code
wasm-snip "$1"/strip_module.wasm -o "$1"/snipped_module.wasm
rm -rf "$1"/strip_module.wasm

# Use wasm-opt to perform the "asyncify" post-processing step over all modules
export ASYNCIFY_STACK_SIZE=24576
wasm-opt --asyncify -Os "$1"/snipped_module.wasm -o "$1"/wrap.wasm
rm -rf "$1"/snipped_module.wasm

cp "$1"/wrap.wasm ./build/wrap.wasm

