/// Binary search implementation for C code
///
/// # Safety
/// This function is unsafe because it dereferences raw pointers.
/// The caller must ensure:
/// - `base` points to a valid array of `nmemb` elements, each of size `size`
/// - `key` points to a valid object comparable with array elements
/// - `compar` is a valid function pointer that compares correctly
///   Note: this is completely cursor-generated and not checked - want to get stuff to build and run first
#[no_mangle]
pub unsafe extern "C" fn bsearch(
    key: *const core::ffi::c_void,
    base: *const core::ffi::c_void,
    nmemb: usize,
    size: usize,
    compar: unsafe extern "C" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
) -> *mut core::ffi::c_void {
    if base.is_null() || compar as usize == 0 || size == 0 {
        return core::ptr::null_mut();
    }

    let base_ptr = base as *const u8;
    let mut left = 0;
    let mut right = nmemb;

    while left < right {
        let mid = left + (right - left) / 2;
        let mid_ptr = base_ptr.add(mid * size) as *const core::ffi::c_void;

        let cmp = compar(key, mid_ptr);

        if cmp == 0 {
            return mid_ptr as *mut core::ffi::c_void; /* Found */
        } else if cmp < 0 {
            right = mid; /* Search left half */
        } else {
            left = mid + 1; /* Search right half */
        }
    }

    core::ptr::null_mut() /* Not found */
}
