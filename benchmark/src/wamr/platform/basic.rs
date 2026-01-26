use core::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn atoi(s: *const c_char) -> c_int {
    if s.is_null() {
        return 0;
    }

    let mut i = 0usize;
    let mut result: c_int = 0;
    let mut negative = false;

    // Skip whitespace
    while unsafe { *s.add(i) } == b' ' {
        i += 1;
    }

    // Handle sign
    let c = unsafe { *s.add(i) };
    if c == b'-' {
        negative = true;
        i += 1;
    } else if c == b'+' {
        i += 1;
    }

    // Convert digits
    loop {
        let c = unsafe { *s.add(i) };
        if !c.is_ascii_digit() {
            break;
        }
        result = result.wrapping_mul(10).wrapping_add((c - b'0') as c_int);
        i += 1;
    }

    if negative {
        -result
    } else {
        result
    }
}
