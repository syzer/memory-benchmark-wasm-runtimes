use crate::wasmi::wasm::{init_runtime, instantiate_module, Runtime};
use embassy_time::Instant;

extern crate alloc;

mod wasm;

const ITERATIONS: i32 = 100_000;

#[embassy_executor::task]
pub async fn wasm_task() {
    let runtime = init_runtime().expect("failed to init runtime");
    let Runtime {
        store,
        module,
        mut linker,
        ..
    } = runtime;

    let (mut store, running) = match instantiate_module(store, module, &mut linker) {
        Ok(res) => res,
        Err(err) => {
            defmt::error!("wasm error: {}", err);
            return;
        }
    };

    let run_fn = running
        .get_typed_func::<i32, ()>(&mut store, "run")
        .expect("failed to get function");

    let start = Instant::now();
    run_fn
        .call(store, ITERATIONS)
        .expect("failed to call run function with wasmi");
    let elapsed = Instant::now() - start;
    defmt::info!(
        "benchmark done engine=wasmi iterations={} elapsed_ticks={} elapsed_us={}",
        ITERATIONS,
        elapsed.as_ticks(),
        elapsed.as_micros()
    );
}
