use wasmtime::{AsContext, Caller, Config, Engine, Func, Instance, Memory, Module, Store};
use embassy_time::Instant;

// Note for me: https://docs.wasmtime.dev/examples-minimal.html
// (has a nice walkthrough how to shrink stuff - Rust stuffs in general and wasmtime in particular)
// Other note: we pre-compile the module using the wasmtime engine. I have a project on that on my local system

const ITERATIONS: i32 = 100_000;

#[embassy_executor::task]
pub async fn wasm_task() {
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
    config.memory_reservation_for_growth(0);
    config.memory_guard_size(0);
    config.signals_based_traps(false);

    // 3. Explicitly pin Wasm features instead of relying on defaults
    // (exact choices are up to you/your guest, but they must match on both sides)
    config.wasm_simd(false);
    config.wasm_memory64(false);
    config.wasm_relaxed_simd(false);
    config.wasm_tail_call(false);
    config.wasm_multi_value(false); // example – choose what you actually use
    config.wasm_multi_memory(false);

    config.max_wasm_stack(32 * 1024);

    // ...set others you care about explicitly too.

    let engine = Engine::new(&config).expect("engine");

    let precompiled = include_bytes!("../../../benchmark_module.cwasm");

    let module =
        unsafe { Module::deserialize(&engine, precompiled).expect("failed to deser module") };

    let mut store = Store::new(&engine, ());
    let log_func = Func::wrap(&mut store, log);

    let instance = Instance::new(&mut store, &module, &[log_func.into()])
        .expect("failed to instantiate module");

    let run = instance
        .get_typed_func::<i32, ()>(&mut store, "run")
        .unwrap();

    let start = Instant::now();
    run.call(&mut store, ITERATIONS).unwrap();
    let elapsed = Instant::now() - start;
    defmt::info!(
        "benchmark done engine=wasmtime iterations={} elapsed_ticks={} elapsed_us={}",
        ITERATIONS,
        elapsed.as_ticks(),
        elapsed.as_micros()
    );
}

pub(super) fn log(mut caller: Caller<'_, ()>, buffer_ptr: u32, length: u32) {
    let memory = get_memory(&mut caller);
    let store = caller.as_context();
    let data_start = buffer_ptr as usize;
    let data_end = data_start + (length as usize);
    let data = &memory.data(&store)[data_start..data_end];

    let Ok(msg) = str::from_utf8(data) else {
        defmt::error!("module logged using and invalid string");
        return;
    };

    defmt::info!("module msg: {}", msg);
}

fn get_memory<T>(caller: &mut Caller<'_, T>) -> Memory {
    caller
        .get_export("memory")
        .expect("module memory checked during module load")
        .into_memory()
        .expect("module memory checked during module load")
}
