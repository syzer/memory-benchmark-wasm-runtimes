//! Math function implementations for AOT relocation symbols
//! These are called by AOT-compiled WASM code

use core::ffi::c_double;
use core::ffi::c_float;

#[unsafe(no_mangle)]
pub extern "C" fn fmin(x: c_double, y: c_double) -> c_double {
    if x < y {
        x
    } else {
        y
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fminf(x: c_float, y: c_float) -> c_float {
    if x < y {
        x
    } else {
        y
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fmax(x: c_double, y: c_double) -> c_double {
    if x > y {
        x
    } else {
        y
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fmaxf(x: c_float, y: c_float) -> c_float {
    if x > y {
        x
    } else {
        y
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ceil(x: c_double) -> c_double {
    libm::ceil(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn ceilf(x: c_float) -> c_float {
    libm::ceilf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn floor(x: c_double) -> c_double {
    libm::floor(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn floorf(x: c_float) -> c_float {
    libm::floorf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn trunc(x: c_double) -> c_double {
    libm::trunc(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn truncf(x: c_float) -> c_float {
    libm::truncf(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn rint(x: c_double) -> c_double {
    libm::rint(x)
}

#[unsafe(no_mangle)]
pub extern "C" fn rintf(x: c_float) -> c_float {
    libm::rintf(x)
}
