extern crate alloc;

use core::any::Any;

use alloc::boxed::Box;
use alloc::vec::Vec;

use tinywasm::{
    types::{TinyWasmModule, WasmValue},
    Module, StackConfig, Store,
};

use crate::tiny::imports::setup_imports;

mod imports;

#[allow(dead_code)]
fn precompiled_module() -> Module {
    let tw_wasm = include_bytes!("../../../benchmark_module.tw");
    let tw_module =
        TinyWasmModule::from_twasm(tw_wasm).expect("failed to read in tiny wasm module");
    Module::from(tw_module)
}

#[embassy_executor::task]
pub async fn wasm_task() {
    let module = precompiled_module();

    let stack_config = StackConfig::new()
        .with_block_stack_init_size(0)
        .with_value_stack_128_init_size(0)
        .with_value_stack_64_init_size(0)
        .with_value_stack_32_init_size(0)
        .with_value_stack_ref_init_size(0);
    let mut store = Store::with_config(stack_config);

    let imports = setup_imports();

    let instance = module
        .instantiate(&mut store, Some(imports))
        .expect("failed to instantiate");

    // we retrieve functions the same way as before
    let func = instance
        .exported_func::<(), ()>(&store, "run")
        .expect("failed to get function");

    func.call(&mut store, ())
        .expect("failed to call function with tinywasm");
}
