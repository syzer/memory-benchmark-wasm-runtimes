set shell := ["bash", "-cu"]

run:
    just run-wasmi-nrf54

run-nrf54:
    just run-wasmi-nrf54

run-nrf53:
    just run-wasmi-nrf53

run-wasmi-nrf54:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF54L15 --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf54,engine-wasmi

run-wasmi-nrf53:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF5340_xxAA --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf53,engine-wasmi

run-wamr-nrf54:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF54L15 --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf54,engine-wamr

run-wamr-nrf53:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF5340_xxAA --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf53,engine-wamr

run-tinywasm-nrf54:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF54L15 --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf54,engine-tinywasm

run-tinywasm-nrf53:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF5340_xxAA --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf53,engine-tinywasm

run-wasmtime-nrf54:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF54L15 --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf54,engine-wasmtime

run-wasmtime-nrf53:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF5340_xxAA --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf53,engine-wasmtime

run-wamr-aot-nrf54:
    test -f benchmark_module.aot || (echo "benchmark_module.aot missing. Run ./build_wamr_aot_module.sh first." && exit 1)
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF54L15 --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf54,engine-wamr,wamr-aot

run-wamr-aot-nrf53:
    test -f benchmark_module.aot || (echo "benchmark_module.aot missing. Run ./build_wamr_aot_module.sh first." && exit 1)
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF5340_xxAA --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf53,engine-wamr,wamr-aot

run-wasmr-aot:
    just run-wamr-aot-nrf54
