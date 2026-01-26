#![no_std] // required for wasm32-unknown-unknown

use core::panic::PanicInfo;
#[panic_handler] // required when you drop std
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link(wasm_import_module = "logging")]
extern "C" {
    fn log(buffer: *const u8, length: i32);
}

// Safe wrapper for logging
pub(crate) fn log_msg(msg: &str) {
    unsafe {
        log(msg.as_ptr(), msg.len() as i32);
    }
}

#[no_mangle]
pub extern "C" fn run() {
    loop {
        log_msg("iterating");
    }
}
