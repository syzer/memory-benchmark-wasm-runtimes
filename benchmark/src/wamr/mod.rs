use crate::wamr::{bindings::wasm_runtime_init, platform::register_stack_boundary};
use embassy_time::Instant;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/wamr_bindings.rs"));
}

mod platform;

const ITERATIONS: i32 = 100_000;

#[embassy_executor::task]
pub async fn wasm_task() {
    // set the stack boundary
    let stack_start = 0u8;
    let start_address = &stack_start as *const u8 as usize;
    register_stack_boundary(start_address);

    match fallible_logic() {
        Ok(()) => (),
        Err(e) => {
            defmt::error!("WAMR engine initialization failed: {}", e);
        }
    }
}

fn fallible_logic() -> Result<(), &'static str> {
    // Setting up the wamr engine
    defmt::info!("Init of the WAMR engine");

    init_wamr_runtime()?;
    defmt::info!("WAMR engine initialized");

    // registering the log function - native symbols have to live until the end (we have to write a proper safe wrapper here)
    let native_symbols = [bindings::NativeSymbol {
        // we want this to live as long as everything lives
        symbol: c"log".as_ptr(),
        func_ptr: log_host_function as *mut core::ffi::c_void,
        signature: c"(*~)".as_ptr(), // this means pointer and length + no return -- this is a WAMR specific thing -- see its docs
        attachment: core::ptr::null_mut(),
    }];

    let success = unsafe {
        bindings::wasm_runtime_register_natives(
            c"logging".as_ptr(),
            native_symbols.as_ptr() as *mut bindings::NativeSymbol,
            native_symbols.len() as u32,
        )
    };

    if !success {
        return Err("failed to register log function");
    }
    defmt::info!("Log function registered");

    let wasm_bytes =
        include_bytes!("../../../benchmark_module/target/wasm32-unknown-unknown/release/benchmark_module.wasm");

    defmt::info!("Wasm file size: {} bytes", wasm_bytes.len());
    if wasm_bytes.len() < 16 {
        return Err("Wasm file too small");
    }

    let mut wasm_vec = wasm_bytes.to_vec();
    let module = load_module(&mut wasm_vec)?;
    defmt::info!("Module loaded");

    let module_inst = instantiate_module(module)?;
    defmt::info!("Module instantiated");

    let start = Instant::now();
    call_run_function(module_inst, ITERATIONS)?;
    let elapsed_us = (Instant::now() - start).as_micros();
    defmt::info!(
        "benchmark done engine=wamr iterations={} elapsed_us={}",
        ITERATIONS,
        elapsed_us
    );

    Ok(())
}

fn init_wamr_runtime() -> Result<(), &'static str> {
    let init_success = unsafe { wasm_runtime_init() };
    if !init_success {
        return Err("Failed to initialize WAMR runtime");
    }
    Ok(())
}

/// This is the function that will be called by the guest module to log a message
/// # Safety
/// This function is unsafe because it dereferences raw pointers and performs memory operations.
/// The caller must ensure that the pointers are valid and that the memory is not corrupted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_host_function(
    _exec_env: bindings::wasm_exec_env_t,
    buffer: *const u8, // Changed from u32 - this is already the converted native pointer!
    length: u32,
) {
    if buffer.is_null() {
        defmt::info!("buffer pointer is null");
        return;
    }

    let slice = unsafe { core::slice::from_raw_parts(buffer, length as usize) };
    if let Ok(msg) = core::str::from_utf8(slice) {
        defmt::info!("module log: {}", msg);
    } else {
        defmt::info!("module logged using an invalid string");
    }
}

fn load_module(wasm_bytes: &mut [u8]) -> Result<bindings::wasm_module_t, &'static str> {
    let error_buf = [0i8; 256]; // not really using it right now, but it's here for the API

    let module = unsafe {
        bindings::wasm_runtime_load(
            wasm_bytes.as_mut_ptr(),
            wasm_bytes.len() as u32,
            error_buf.as_ptr() as *mut core::ffi::c_char,
            error_buf.len() as u32,
        )
    };

    if !module.is_null() {
        Ok(module)
    } else {
        defmt::error!("looking at error message");
        // ADD: Print the actual error message
        let error_msg =
            unsafe { core::ffi::CStr::from_ptr(error_buf.as_ptr() as *const core::ffi::c_char) };
        if let Ok(msg) = error_msg.to_str() {
            if !msg.is_empty() {
                defmt::error!("WAMR load error: {}", msg);
            }
        }
        Err("Failed to load module")
    }
}

fn instantiate_module(
    module: bindings::wasm_module_t,
) -> Result<bindings::wasm_module_inst_t, &'static str> {
    let error_buf = [0i8; 256]; // not really using it right now, but it's here for the API

    let default_stack_size = 8 * 1024; // like, for no reason
    let host_managed_heap_size = 16 * 1024; // like, for no reason times two
    let module_inst = unsafe {
        bindings::wasm_runtime_instantiate(
            module,
            default_stack_size,
            host_managed_heap_size,
            error_buf.as_ptr() as *mut core::ffi::c_char,
            error_buf.len() as u32,
        )
    };

    // Note: in principle, it would be nice to read out the error buffer properly
    if !module_inst.is_null() {
        let exception = unsafe { bindings::wasm_runtime_get_exception(module_inst) };
        if !exception.is_null() {
            let exception_str = unsafe { core::ffi::CStr::from_ptr(exception) };
            defmt::warn!(
                "Exception after instantiation (but instance created): {}",
                exception_str.to_string_lossy().as_str()
            );
        } else {
            defmt::info!("no exceptions after instantiation");
        }

        let error_msg = unsafe {
            core::ffi::CStr::from_ptr(error_buf.as_ptr() as *const core::ffi::c_char)
                .to_string_lossy()
        };

        if !error_msg.is_empty() && error_msg != "\0" {
            defmt::info!("Error buffer after instantiation: {}", error_msg.as_str());
        } else {
            defmt::info!("also nothing in the error buffer");
        }

        Ok(module_inst)
    } else {
        Err("Failed to instantiate module")
    }
}

fn call_run_function(
    module_inst: bindings::wasm_module_inst_t,
    iterations: i32,
) -> Result<(), &'static str> {
    // Look up the run function
    let function = unsafe {
        bindings::wasm_runtime_lookup_function(
            module_inst,
            c"run".as_ptr() as *const core::ffi::c_char,
        )
    };

    if function.is_null() {
        return Err("function 'run' not found");
    }

    // create the execution env for the function
    let stack_size = 8 * 1024;
    let exec_env = unsafe { bindings::wasm_runtime_create_exec_env(module_inst, stack_size) };

    if exec_env.is_null() {
        return Err("failed to create exec environment");
    }

    let mut argv = [iterations as u32, 0u32];

    defmt::info!("about to call run function");

    let success = unsafe { bindings::wasm_runtime_call_wasm(exec_env, function, 1, argv.as_mut_ptr()) };

    // cleanup
    unsafe {
        bindings::wasm_runtime_destroy_exec_env(exec_env);
    }

    if !success {
        let exception = unsafe { bindings::wasm_runtime_get_exception(module_inst) };

        if !exception.is_null() {
            // let exception_str = unsafe { core::ffi::CStr::from_ptr(exception) };
            return Err("Wasm exception");
        } else {
            return Err("failed to call 'run' function");
        }
    }
    Ok(())
}
