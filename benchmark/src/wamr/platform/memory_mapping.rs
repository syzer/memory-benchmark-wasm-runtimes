extern crate alloc;

use alloc::alloc::{alloc, dealloc, Layout};
use core::ffi::c_void;

const PAGE_SIZE: usize = 4096; // for alignment

fn align_up(size: usize) -> usize {
    (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1) // we add a page size and then to the masking vodoo, which effectively brings us down to the next page boundary
}

#[no_mangle]
pub unsafe extern "C" fn os_mmap(
    _hint: *mut c_void,
    size: usize,
    _prot: i32,  // no mmu, so we ignore it
    _flags: i32, // mapping flags -- we don't use them
    _file: i32,  // no file, so we ignore it
) -> *mut c_void {
    if size == 0 {
        return core::ptr::null_mut();
    }

    let aligned_size = align_up(size);

    let layout = match Layout::from_size_align(aligned_size, PAGE_SIZE) {
        Ok(layout) => layout,
        Err(_) => return core::ptr::null_mut(),
    };

    let ptr = alloc(layout);
    if ptr.is_null() {
        return core::ptr::null_mut();
    }

    // zero the memory (WAMR wants this)
    core::ptr::write_bytes(ptr, 0, aligned_size);

    ptr as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn os_munmap(addr: *mut c_void, size: usize) {
    if addr.is_null() || size == 0 {
        return;
    }

    let aligned_size = align_up(size);
    let layout = match Layout::from_size_align(aligned_size, PAGE_SIZE) {
        Ok(layout) => layout,
        Err(_) => return,
    };

    dealloc(addr as *mut u8, layout);
}

#[no_mangle]
pub unsafe extern "C" fn os_mprotect(addr: *mut c_void, size: usize, _prot: i32) -> i32 {
    if addr.is_null() || size == 0 {
        return -1;
    }

    // no mmu, so we ignore it
    0
}

#[no_mangle]
pub unsafe extern "C" fn os_mremap(
    old_addr: *mut c_void,
    old_size: usize,
    new_size: usize,
) -> *mut c_void {
    if old_addr.is_null() {
        return core::ptr::null_mut();
    }

    let new_addr = os_mmap(core::ptr::null_mut(), new_size, 0, 0, -1);

    if new_addr.is_null() {
        return core::ptr::null_mut();
    }

    // Copy the min between old and new to the memory we just allocated
    let copy_size = old_size.min(new_size);
    if copy_size > 0 {
        core::ptr::copy_nonoverlapping(old_addr as *const u8, new_addr as *mut u8, copy_size);
    }

    // Free old memory
    os_munmap(old_addr, old_size);

    new_addr
}

#[no_mangle]
pub extern "C" fn os_getpagesize() -> i32 {
    PAGE_SIZE as i32
}
