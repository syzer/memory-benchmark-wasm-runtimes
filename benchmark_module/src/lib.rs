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
pub extern "C" fn run(iterations: i32) {
    let mut acc: u32 = 0;
    for i in 0..iterations {
        let val = i as u32;
        acc = acc
            .wrapping_add(val)
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
    }

    let msg = if acc & 1 == 0 { "done-even" } else { "done-odd" };
    log_msg(msg);
}
