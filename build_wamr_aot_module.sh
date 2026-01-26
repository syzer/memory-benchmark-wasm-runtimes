#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WAMR_DIR="$SCRIPT_DIR/third_party/wamr"
WAMRC_DIR="$WAMR_DIR/wamr-compiler"
WAMRC_BUILD_DIR="$WAMRC_DIR/build"
WAMRC="$WAMRC_BUILD_DIR/wamrc"

INPUT_WASM="$SCRIPT_DIR/benchmark_module/target/wasm32-unknown-unknown/release/benchmark_module.wasm"
OUTPUT_AOT="$SCRIPT_DIR/benchmark_module.aot"

# Step 0: Install dependencies (if needed)
echo "=== Checking dependencies ==="
MISSING_DEPS=()
for dep in ninja-build build-essential cmake g++; do
    if ! dpkg -l | grep -q "^ii  $dep "; then
        MISSING_DEPS+=("$dep")
    fi
done

if [[ ${#MISSING_DEPS[@]} -gt 0 ]]; then
    echo "=== Installing missing dependencies: ${MISSING_DEPS[*]} ==="
    echo "This requires sudo privileges..."
    sudo apt-get update
    sudo apt-get install -y "${MISSING_DEPS[@]}"
else
    echo "=== All dependencies already installed ==="
fi

# Check that input wasm exists
if [[ ! -f "$INPUT_WASM" ]]; then
    echo "Error: $INPUT_WASM not found. Build the wasm module first using 'the build_modules.sh' script."
    exit 1
fi

# Step 1: Build LLVM (only if not already built)
LLVM_DIR="$WAMRC_DIR/build/core/deps/llvm"
if [[ ! -d "$LLVM_DIR" ]]; then
    echo "=== Building LLVM (this takes a while...) ==="
    cd "$WAMRC_DIR"
    ./build_llvm.sh
else
    echo "=== LLVM already built, skipping ==="
fi

# Step 2: Build wamrc (only if not already built)
if [[ ! -x "$WAMRC" ]]; then
    echo "=== Building wamrc ==="
    mkdir -p "$WAMRC_BUILD_DIR"
    cd "$WAMRC_BUILD_DIR"
    cmake ..
    make -j"$(nproc)"
else
    echo "=== wamrc already built, skipping ==="
fi

# Step 3: Compile .wasm to .aot for nRF53 (Cortex-M33)
echo "=== Compiling $INPUT_WASM to AOT for nRF53 ==="
"$WAMRC" \
    --target=thumbv7 \
    --cpu=cortex-m33 \
    --target-abi=gnueabihf \
    -o "$OUTPUT_AOT" \
    "$INPUT_WASM"

echo "=== Done: $OUTPUT_AOT ==="