#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=== Building benchmark module ==="
cd "$SCRIPT_DIR/benchmark_module"
cargo build --release

echo "=== Precompiling module for wasmtime ==="
cd "$SCRIPT_DIR/wasmtime_precompile"
cargo run

echo "=== Done ==="