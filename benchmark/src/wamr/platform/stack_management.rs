//! Module for the scary embedded stuff where we have to manage the stack and other scary things

use core::sync::atomic::{AtomicUsize, Ordering};

static STACK_BOUNDARY: AtomicUsize = AtomicUsize::new(0);

/**
 * Note: We are basically just kind of guesstimating. We assume that the stack will not grow more
 * than a fix margin from an area near the stack start. And the resulting maximal address is what we
 * provide WAMR as the address to check for stack overflow.
 *
 * Actually, in newer embassy versions, we could manage our task stack directly and provide the actual bound.
 * that is what we should do if we were to seriously integrate WAMR into our stack.
 */
const STACK_SAFETY_MARGIN: usize = 8 * 1024;

pub fn register_stack_boundary(stack_start: usize) {
    let boundary = stack_start.saturating_sub(STACK_SAFETY_MARGIN);
    STACK_BOUNDARY.store(boundary, Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn os_thread_get_stack_boundary() -> *mut core::ffi::c_void {
    STACK_BOUNDARY.load(Ordering::Relaxed) as *mut core::ffi::c_void
}
