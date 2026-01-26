extern crate alloc;

use alloc::alloc::{alloc, dealloc, Layout};
use core::ffi::c_void;

/// Minimum alignment for WAMR - must be 8 bytes for heap structures
const WAMR_MIN_ALIGN: usize = 8;

/// Allocates memory
/// We will store the size in the header before it so we can dealloc properly
#[no_mangle]
pub extern "C" fn os_malloc(size: usize) -> *mut c_void {
    if size == 0 {
        return core::ptr::null_mut();
    }

    let total_size = size + size_header_offset();
    let layout = match Layout::from_size_align(total_size, WAMR_MIN_ALIGN) {
        Ok(layout) => layout,
        Err(_) => return core::ptr::null_mut(),
    };

    // pointer nonsense -- we need to carefully check this
    unsafe {
        let ptr_u8 = alloc(layout);
        if ptr_u8.is_null() {
            return core::ptr::null_mut();
        }

        // we write the size at the front of the data we just allocated
        *(ptr_u8 as *mut usize) = total_size;

        // the pointer we return needs to point a little bit further, so that WAMR is not aware of our size header
        ptr_u8.add(size_header_offset()) as *mut c_void
    }
}

/// Free the memory
/// We figure out how much to deallocate via the size header we wrote when allocating
#[no_mangle]
pub extern "C" fn os_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }

    // pointer nonsense
    unsafe {
        let size_ptr = (ptr as *mut u8).sub(size_header_offset()) as *mut usize;
        let total_size = *size_ptr;

        let layout = Layout::from_size_align_unchecked(total_size, WAMR_MIN_ALIGN);
        let actual_mem_ptr = size_ptr as *mut u8;

        // the actual deallocation (size header and memory used by WAMR)
        dealloc(actual_mem_ptr, layout);
    }
}

/// Reallocate memory
/// Here, we have to update the size header we wrote during the original allocation
#[no_mangle]
pub extern "C" fn os_realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    if ptr.is_null() {
        // just allocate new memory
        return os_malloc(new_size);
    }

    if new_size == 0 {
        // just free the memory - we  already checked that ptr is not null
        os_free(ptr);
        return core::ptr::null_mut();
    }

    // pointer nonsense
    unsafe {
        // We first want to get the old size
        let size_ptr = (ptr as *mut u8).sub(size_header_offset()) as *mut usize;
        let total_size = *size_ptr;
        let old_size = total_size - size_header_offset();

        if new_size == old_size {
            // we have the memory we need -> just return the pointer
            return ptr;
        }

        // We won't be too crazy here: we'll just always alloate a new block to be safe
        let new_ptr = os_malloc(new_size); // we are using our function -> the size header writing is taken care of there
        if new_ptr.is_null() {
            return core::ptr::null_mut();
        }

        // Copy the smaller of the sizes - we either take what is there (when we expand) or what fits (when we shrink)
        let copy_size = old_size.min(new_size);
        core::ptr::copy_nonoverlapping(ptr as *const u8, new_ptr as *mut u8, copy_size);

        // Free the old block
        os_free(ptr);

        new_ptr
    }
}

fn size_header_offset() -> usize {
    8
}
