set shell := ["bash", "-cu"]

run:
    just run-nrf54

run-nrf54:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF54L15 --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf54,engine-wasmi

run-nrf53:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF5340_xxAA --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf53,engine-wasmi

run-wamr-nrf54:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF54L15 --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf54,engine-wamr

run-wamr-nrf53:
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF5340_xxAA --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf53,engine-wamr

run-wasmr-aot:
    test -f benchmark_module.aot || (echo "benchmark_module.aot missing. Run ./build_wamr_aot_module.sh first." && exit 1)
    cd benchmark && CARGO_TARGET_THUMBV8M_MAIN_NONE_EABIHF_RUNNER="probe-rs run --chip nRF54L15 --allow-erase-all" rustup run nightly-2025-06-15 cargo run --release --no-default-features --features board-nrf54,engine-wamr,wamr-aot
