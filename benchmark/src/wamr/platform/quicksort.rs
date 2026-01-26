/// Simple qsort implementation for embedded use
/// This is a minimal quicksort that matches the C standard library signature
/// Note: this is completely cursor-generated and not checked - want to get stuff to build and run first
#[no_mangle]
pub extern "C" fn qsort(
    base: *mut core::ffi::c_void,
    nmemb: usize,
    size: usize,
    compar: extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> core::ffi::c_int,
) {
    if base.is_null() || nmemb == 0 || size == 0 {
        return;
    }

    unsafe {
        qsort_internal(base, nmemb, size, compar);
    }
}

unsafe fn qsort_internal(
    base: *mut core::ffi::c_void,
    nmemb: usize,
    size: usize,
    compar: extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> core::ffi::c_int,
) {
    if nmemb <= 1 {
        return;
    }

    // Partition and get pivot index
    let pivot_idx = partition(base, nmemb, size, compar);

    // Recursively sort left partition
    let left_size = pivot_idx;
    qsort_internal(base, left_size, size, compar);

    // Recursively sort right partition
    let right_base = (base as *mut u8).add((pivot_idx + 1) * size) as *mut core::ffi::c_void;
    let right_size = nmemb - pivot_idx - 1;
    qsort_internal(right_base, right_size, size, compar);
}

unsafe fn partition(
    base: *mut core::ffi::c_void,
    nmemb: usize,
    size: usize,
    compar: extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> core::ffi::c_int,
) -> usize {
    let last_idx = nmemb - 1;
    let pivot = (base as *mut u8).add(last_idx * size);
    let mut i = 0;

    for j in 0..last_idx {
        let elem_j = (base as *mut u8).add(j * size);
        if compar(elem_j as *const _, pivot as *const _) <= 0 {
            swap_bytes(elem_j, (base as *mut u8).add(i * size), size);
            i += 1;
        }
    }

    swap_bytes((base as *mut u8).add(i * size), pivot, size);
    i
}

unsafe fn swap_bytes(a: *mut u8, b: *mut u8, size: usize) {
    // Simple byte-by-byte swap
    for i in 0..size {
        let temp = *a.add(i);
        *a.add(i) = *b.add(i);
        *b.add(i) = temp;
    }
}
