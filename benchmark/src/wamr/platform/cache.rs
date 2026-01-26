#[unsafe(no_mangle)]
pub extern "C" fn os_dcache_flush() {
    cortex_m::asm::dsb();
}

#[unsafe(no_mangle)]
pub extern "C" fn os_icache_flush(_start: *mut core::ffi::c_void, _len: usize) {
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}

#[unsafe(no_mangle)]
pub extern "C" fn os_thread_jit_write_protect_np(_enabled: bool) {
    // No-op on non-Apple platforms
}
