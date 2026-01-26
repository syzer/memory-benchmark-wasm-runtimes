use core::str;

use wasmi::{AsContext, Caller, Config, Engine, Instance, Linker, Module, Store};

extern crate alloc;

pub(super) struct Runtime {
    pub(super) _engine: Engine,
    pub(super) store: Store<()>,
    pub(super) module: Module,
    pub(super) linker: Linker<()>,
}

pub fn init_runtime() -> Result<Runtime, &'static str> {
    let mut cfg = Config::default();
    cfg.compilation_mode(wasmi::CompilationMode::Eager);
    let engine = Engine::new(&cfg);

    static WASM: &[u8] = include_bytes!(
        "../../../benchmark_module/target/wasm32-unknown-unknown/release/benchmark_module.wasm"
    ); // the module we loaded
    let module =
        unsafe { Module::new_unchecked(&engine, WASM).map_err(|_e| "failed to load module")? };

    let state = ();
    let store = Store::new(&engine, state);
    let mut linker = <Linker<()>>::new(&engine);
    link_host_functions(&mut linker)?;

    Ok(Runtime {
        _engine: engine,
        store,
        module,
        linker,
    })
}

pub fn instantiate_module(
    mut store: Store<()>,
    module: Module,
    linker: &mut Linker<()>,
) -> Result<(Store<()>, Instance), &'static str> {
    let instance = linker
        .instantiate_and_start(&mut store, &module)
        .expect("failed to start instance");

    Ok((store, instance))
}

fn link_host_functions(linker: &mut Linker<()>) -> Result<(), &'static str> {
    link_logging(linker)?;
    Ok(())
}

fn link_logging(linker: &mut Linker<()>) -> Result<(), &'static str> {
    linker
        .func_wrap(
            "logging",
            "log",
            |caller: Caller<'_, ()>, buffer_ptr: u32, length: u32| {
                let memory = caller
                    .get_export("memory")
                    .expect("module does not export memory")
                    .into_memory()
                    .expect("failed to get memory");
                let store = caller.as_context();
                let data_start = buffer_ptr as usize;
                let data_end = data_start + (length as usize);
                let data = &memory.data(store)[data_start..data_end];

                let log_msg = str::from_utf8(data).expect("failed to convert string");
                defmt::info!("module log: {}", log_msg);
            },
        )
        .map_err(|_| "failed to link log function")?;
    Ok(())
}
