use core::ffi::{c_char, c_int};

/// Simple strcmp implementation for embedded use
/// Compares two null-terminated strings
/// Returns: 0 if equal, <0 if s1 < s2, >0 if s1 > s2
/// Note: this is completely cursor-generated and not checked - want to get stuff to build and run first
#[no_mangle]
pub extern "C" fn strcmp(
    s1: *const core::ffi::c_char,
    s2: *const core::ffi::c_char,
) -> core::ffi::c_int {
    if s1.is_null() || s2.is_null() {
        // If either is null, undefined behavior in C, but we'll return 0 if both null
        if s1.is_null() && s2.is_null() {
            return 0;
        }
        // One is null - not standard behavior, but safe fallback
        return if s1.is_null() { -1 } else { 1 };
    }

    unsafe {
        let mut p1 = s1;
        let mut p2 = s2;

        loop {
            let c1 = *p1;
            let c2 = *p2;

            // If characters differ, return the difference
            if c1 != c2 {
                return (c1 as core::ffi::c_int).wrapping_sub(c2 as core::ffi::c_int);
            }

            // If we've reached the end of both strings, they're equal
            if c1 == 0 {
                return 0;
            }

            // Move to next character
            p1 = p1.add(1);
            p2 = p2.add(1);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    if n == 0 {
        return 0;
    }

    let mut i = 0;
    while i < n {
        let c1 = unsafe { *s1.add(i) };
        let c2 = unsafe { *s2.add(i) };

        if c1 != c2 {
            return (c1 as c_int) - (c2 as c_int);
        }

        if c1 == 0 {
            return 0;
        }

        i += 1;
    }

    0
}

/// strlen implementation - returns length of null-terminated string
#[no_mangle]
pub extern "C" fn strlen(s: *const core::ffi::c_char) -> usize {
    if s.is_null() {
        return 0;
    }

    unsafe {
        let mut len = 0;
        let mut p = s;
        while *p != 0 {
            len += 1;
            p = p.add(1);
        }
        len
    }
}

/// memcmp implementation - compares two memory regions
#[no_mangle]
pub extern "C" fn memcmp(
    s1: *const core::ffi::c_void,
    s2: *const core::ffi::c_void,
    n: usize,
) -> core::ffi::c_int {
    if s1.is_null() || s2.is_null() || n == 0 {
        return 0;
    }

    unsafe {
        let p1 = s1 as *const u8;
        let p2 = s2 as *const u8;
        for i in 0..n {
            let c1 = *p1.add(i);
            let c2 = *p2.add(i);
            if c1 != c2 {
                return (c1 as core::ffi::c_int).wrapping_sub(c2 as core::ffi::c_int);
            }
        }
        0
    }
}

/// abort() implementation for embedded
/// Called when an assertion fails in WAMR
#[no_mangle]
pub extern "C" fn abort() -> ! {
    // Log the abort (if defmt is available)
    // Note: This might not work if we're in a bad state, but it's worth trying
    defmt::error!("abort() called - assertion failed or fatal error");

    // Loop forever - in embedded systems, abort typically doesn't return
    loop {
        cortex_m::asm::nop(); // Prevent optimization
    }
}
