# Memory Footprint Benchmark for WebAssembly Runtimes on MCUs

This repository contains a benchmark suite for measuring the memory footprint of different WebAssembly runtimes when running on microcontrollers.

TODO: Add a information about and link to the corresponding Fosdem talk

## Supported Runtimes

| Runtime | Feature Flag | Module Format |
|---------|--------------|---------------|
| [Wasmi](https://github.com/wasmi-labs/wasmi) | `engine-wasmi` | `.wasm` (interpreted) |
| [Tinywasm](https://github.com/explodingcamera/tinywasm) | `engine-tinywasm` | `.tw` (precompiled) |
| [Wasmtime](https://github.com/bytecodealliance/wasmtime) | `engine-wasmtime` | `.cwasm` (precompiled, Pulley VM) |
| [WAMR](https://github.com/bytecodealliance/wasm-micro-runtime) | `engine-wamr` | `.wasm` (interpreter mode) |

## Hardware Requirements

This benchmark supports:

- **Nordic nRF5340** development kit (default in `benchmark` crate)
- **Seeed Studio XIAO nRF54L15** (verified)

Connect the board via USB. `probe-rs` handles flashing and debugging.

## Prerequisites

### Rust Toolchain

The project uses Rust nightly. The correct toolchain and targets will be installed automatically when you build, thanks to `rust-toolchain.toml`.

### probe-rs

Install [probe-rs](https://probe.rs/) for flashing and running code:

```bash
cargo install probe-rs-tools
```

### For WAMR (optional)

If you want to benchmark WAMR, you'll need the ARM cross-compiler:

```bash
sudo apt-get install gcc-arm-none-eabi
```

## Getting Started

### 1. Clone and Initialize Submodules

```bash
git clone <this-repo>
cd memory-benchmark-wasm-runtimes
git submodule update --init --recursive
```

### 2. Build the Benchmark Modules

The Wasm modules need to be built and preprocessed for each runtime before running the benchmark.

#### Quick Start (all runtimes except WAMR)

If you just want to test the benchmark or aren't interested in WAMR, run:

```bash
./build_modules.sh
```

This script:
- Compiles the benchmark Wasm module
- Precompiles it for Wasmtime (`.cwasm`)
- Precompiles it for Tinywasm (`.tw`)

#### Optional: Build WAMR AOT module

If you want to generate a WAMR AOT module (not required for the default WAMR runtime path in this repo), run after the `build_modules.sh` script finishes:

```bash
./build_wamr_aot_module.sh
```

> **Note:** This script needs to compile LLVM to build the `wamrc` compiler. The first run takes a **long time** (30+ minutes depending on your machine). Subsequent runs are fast as LLVM and `wamrc` are cached.

The script is Linux-oriented and automatically installs required dependencies (`ninja-build`, `cmake`, etc.) if they're missing.

### 3. Run the Benchmark

Run via `just` (recommended):

```bash
just run        # default: XIAO nRF54L15 + wasmi
just run-nrf54  # XIAO nRF54L15 + wasmi
just run-nrf53  # nRF5340 DK + wasmi
just run-wamr-nrf54 # XIAO nRF54L15 + WAMR
just run-wamr-nrf53 # nRF5340 DK + WAMR
```

Or run manually from the benchmark directory:

```bash
cd benchmark
cargo run --release --no-default-features --features engine-wasmi
```

Replace `engine-wasmi` with your desired runtime:
- `engine-wasmi` — Wasmi interpreter
- `engine-tinywasm` — Tinywasm
- `engine-wasmtime` — Wasmtime with Pulley VM
- `engine-wamr` — WAMR in AOT mode

The benchmark will be flashed to the connected board and output will be displayed via RTT (Real-Time Transfer).

If you run manually and want to select the board explicitly, add one board feature:

- `board-nrf53` — nRF5340 DK
- `board-nrf54` — XIAO nRF54L15

## Repository Structure

```
.
├── benchmark/              # Main benchmark crate (supports nRF5340 + nRF54L15)
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── wasmi/          # Wasmi runtime integration
│   │   ├── wasmtime/       # Wasmtime runtime integration
│   │   ├── tiny/           # Tinywasm runtime integration
│   │   └── wamr/           # WAMR runtime integration
│   └── wamr_specific/      # WAMR platform implementation for Embassy
├── benchmark_module/       # The Wasm module used for benchmarking
├── wasmtime_precompile/    # Tool to precompile modules for Wasmtime
├── tinywasm_precompile/    # Tool to precompile modules for Tinywasm
├── third_party/
│   ├── embassy/            # Embassy async framework (git submodule)
│   └── wamr/               # WAMR runtime (git submodule)
├── build_modules.sh        # Build script for most runtimes
└── build_wamr_aot_module.sh # Build script for WAMR AOT module
```

## The Benchmark Module

The benchmark module (`benchmark_module/`) is a simple Wasm program that:
1. Imports a `log` function from the host
2. Runs a loop that repeatedly calls `log` with a message

This provides a minimal but representative workload for measuring runtime overhead.

## Troubleshooting

### "probe-rs" not found

Make sure probe-rs is installed and in your PATH:
```bash
cargo install probe-rs-tools
```

### Submodule errors

If you see errors about missing files in `third_party/`, make sure submodules are initialized:
```bash
git submodule update --init --recursive
```

### WAMR build fails with "platform_internal.h not found"

Ensure the `benchmark/wamr_specific/platform/embassy/` directory exists with the platform header.


## License

The code in this repository (excluding contents of `third_party/` and other vendored projects, which have their own licenses) is licensed under either:

- Apache License, Version 2.0, or
- MIT license

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this repository by you shall be dual licensed as above, without any additional terms or conditions.
