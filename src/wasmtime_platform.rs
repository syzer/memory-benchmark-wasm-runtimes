use core::sync::atomic::{AtomicPtr, Ordering};

static WASMTIME_TLS: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());

#[no_mangle]
pub extern "C" fn wasmtime_tls_get() -> *mut u8 {
    WASMTIME_TLS.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn wasmtime_tls_set(val: *mut u8) {
    WASMTIME_TLS.store(val, Ordering::Relaxed);
}
