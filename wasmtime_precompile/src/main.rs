use anyhow::{Context, Result};
use wasmtime::{Config, Engine};

fn main() -> Result<()> {
    let mut config = Config::new();

    // 1. Target must match
    config.target("pulley32").expect("pulley32 target");

    // 2. Match memory/trap behavior to your custom platform.
    //    Using the pattern from the min-platform example:
    config.wasm_custom_page_sizes(true);
    config.gc_support(false);

    // If you are *not* using signals-based traps on the nRF,
    // you should force that consistently:
    config.memory_init_cow(false);
    config.memory_reservation(0);
    config.memory_guard_size(0);
    config.memory_reservation_for_growth(0);
    config.signals_based_traps(false);

    // 3. Explicitly pin Wasm features instead of relying on defaults
    // (exact choices are up to you/your guest, but they must match on both sides)
    config.wasm_simd(false);
    config.wasm_memory64(false);
    config.wasm_relaxed_simd(false);
    config.wasm_tail_call(false);
    config.wasm_multi_value(false); // example – choose what you actually use
    config.wasm_multi_memory(false);
    config.wasm_component_model(false);

    config.max_wasm_stack(32 * 1024);
    // ...set others you care about explicitly too.

    let engine = Engine::new(&config).expect("engine");

    let wasm_bytes = include_bytes!(
        "../../benchmark_module/target/wasm32-unknown-unknown/release/benchmark_module.wasm"
    );

    let compiled = engine
        .precompile_module(wasm_bytes)
        .context("failed to precompile")?;

    std::fs::write("../benchmark_module.cwasm", compiled)?;
    println!("module precompiled for wasmtime; resulting file: 'benchmark_module.cwasm'");

    Ok(())
}
