use crate::wasmi::wasm::{init_runtime, instantiate_module, Runtime};

extern crate alloc;

mod wasm;

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

    let led_fn = running
        .get_typed_func::<(), ()>(&mut store, "run")
        .expect("failed to get function");

    led_fn
        .call(store, ())
        .expect("failed to call run function with wasmi");
}
