use tinywasm::{Extern, Imports};

pub(crate) fn setup_imports() -> Imports {
    // note that we have to provide the args in opposite order
    let log_fn = Extern::typed_func(|ctx, (len, ptr): (i32, i32)| {
        let mem = ctx.exported_memory("memory").expect("failed to get memory");
        let data = mem
            .load(ptr as usize, len as usize)
            .expect("failed to load memory slice");
        let log_msg = str::from_utf8(data).expect("failed to read data as str");

        defmt::info!("module log: {}", log_msg);
        Ok(())
    });

    let mut imports = Imports::new();
    imports.define("logging", "log", log_fn).unwrap();
    imports
}
