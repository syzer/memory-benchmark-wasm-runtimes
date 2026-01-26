use core::ffi::{c_char, c_int, CStr, VaList};
use core::fmt::Write;

/// Simple printf implementation using defmt
/// This handles basic format specifiers that WAMR uses
#[no_mangle]
pub unsafe extern "C" fn os_printf(format: *const c_char, mut args: ...) -> c_int {
    if format.is_null() {
        return 0;
    }

    if let Ok(format_str) = CStr::from_ptr(format).to_str() {
        // Extract arguments directly from the variadic parameter
        // We can't pass ... to another function, so we inline the formatting here
        let mut result = heapless::String::<512>::new();
        let mut chars = format_str.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '%' {
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        's' => {
                            chars.next();
                            let s = args.arg::<*const c_char>();
                            if !s.is_null() {
                                if let Ok(c_str) = CStr::from_ptr(s).to_str() {
                                    let _ = result.push_str(c_str);
                                }
                            }
                        }
                        'd' | 'i' => {
                            chars.next();
                            let val = args.arg::<i32>();
                            let _ = write!(result, "{}", val);
                        }
                        'u' => {
                            chars.next();
                            let val = args.arg::<u32>();
                            let _ = write!(result, "{}", val);
                        }
                        'x' => {
                            chars.next();
                            let val = args.arg::<u32>();
                            let _ = write!(result, "{:x}", val);
                        }
                        'X' => {
                            chars.next();
                            let val = args.arg::<u32>();
                            let _ = write!(result, "{:X}", val);
                        }
                        'p' => {
                            chars.next();
                            let val = args.arg::<*const core::ffi::c_void>();
                            let _ = write!(result, "{:p}", val);
                        }
                        '%' => {
                            chars.next();
                            let _ = result.push('%');
                        }
                        _ => {
                            // Unknown format specifier, just output the %
                            let _ = result.push('%');
                        }
                    }
                } else {
                    let _ = result.push('%');
                }
            } else {
                let _ = result.push(ch);
            }
        }

        defmt::info!("{}", result.as_str());
        result.len() as c_int
    } else {
        0
    }
}

/// vprintf implementation using defmt
#[allow(unused_mut)] // ap needs to be mutable to pass to format_va_list
#[no_mangle]
pub extern "C" fn os_vprintf(format: *const c_char, mut ap: VaList) -> c_int {
    if format.is_null() {
        return 0;
    }

    unsafe {
        if let Ok(format_str) = CStr::from_ptr(format).to_str() {
            let formatted = format_va_list(format_str, ap);
            defmt::info!("{}", formatted.as_str());
            formatted.len() as c_int
        } else {
            0
        }
    }
}

/// Helper to format a string with VaList
/// This is a minimal implementation - handles %s, %d, %u, %x, %X, %p, %%
unsafe fn format_va_list(format: &str, mut ap: VaList) -> heapless::String<512> {
    let mut result = heapless::String::<512>::new();
    let mut chars = format.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    's' => {
                        chars.next();
                        let s = ap.arg::<*const c_char>();
                        if !s.is_null() {
                            if let Ok(c_str) = CStr::from_ptr(s).to_str() {
                                let _ = result.push_str(c_str);
                            }
                        }
                    }
                    'd' | 'i' => {
                        chars.next();
                        let val = ap.arg::<i32>();
                        let _ = write!(result, "{}", val);
                    }
                    'u' => {
                        chars.next();
                        let val = ap.arg::<u32>();
                        let _ = write!(result, "{}", val);
                    }
                    'x' => {
                        chars.next();
                        let val = ap.arg::<u32>();
                        let _ = write!(result, "{:x}", val);
                    }
                    'X' => {
                        chars.next();
                        let val = ap.arg::<u32>();
                        let _ = write!(result, "{:X}", val);
                    }
                    'p' => {
                        chars.next();
                        let val = ap.arg::<*const core::ffi::c_void>();
                        let _ = write!(result, "{:p}", val);
                    }
                    '%' => {
                        chars.next();
                        let _ = result.push('%');
                    }
                    _ => {
                        // Unknown format specifier, just output the %
                        let _ = result.push('%');
                    }
                }
            } else {
                let _ = result.push('%');
            }
        } else {
            let _ = result.push(ch);
        }
    }

    result
}

/// snprintf implementation - formats into a buffer
#[no_mangle]
pub unsafe extern "C" fn snprintf(
    buffer: *mut c_char,
    size: usize,
    format: *const c_char,
    mut args: ...
) -> c_int {
    if buffer.is_null() || size == 0 || format.is_null() {
        return 0;
    }

    if let Ok(format_str) = CStr::from_ptr(format).to_str() {
        // Inline formatting since we can't pass ... to another function
        // Format directly into result string
        let mut result = heapless::String::<512>::new();
        let mut chars = format_str.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '%' {
                // Parse format specifier: %[flags][width][.precision][length]type
                let mut left_align = false;
                let mut zero_pad = false;
                let mut width = 0;
                let mut length_modifier = 0; // 0=none, 1=l, 2=ll

                // Parse flags and width
                while let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '-' => {
                            chars.next();
                            left_align = true;
                        }
                        '0' => {
                            chars.next();
                            zero_pad = true;
                            // If 0 is followed by digits, it's part of width
                            if let Some(&after_zero) = chars.peek() {
                                if after_zero.is_ascii_digit() {
                                    // Parse width starting with 0
                                    let mut width_str = heapless::String::<10>::new();
                                    width_str.push('0').ok();
                                    while let Some(&digit) = chars.peek() {
                                        if digit.is_ascii_digit() {
                                            chars.next();
                                            width_str.push(digit).ok();
                                        } else {
                                            break;
                                        }
                                    }
                                    width = width_str.parse::<usize>().unwrap_or(0);
                                }
                            }
                        }
                        '1'..='9' => {
                            // Parse width
                            let mut width_str = heapless::String::<10>::new();
                            while let Some(&digit) = chars.peek() {
                                if digit.is_ascii_digit() {
                                    chars.next();
                                    width_str.push(digit).ok();
                                } else {
                                    break;
                                }
                            }
                            width = width_str.parse::<usize>().unwrap_or(0);
                        }
                        'l' => {
                            chars.next();
                            if let Some(&'l') = chars.peek() {
                                chars.next();
                                length_modifier = 2; // ll
                            } else {
                                length_modifier = 1; // l
                            }
                        }
                        'h' => {
                            chars.next();
                            // Ignore h/hh for now
                        }
                        's' | 'd' | 'i' | 'u' | 'x' | 'X' | 'p' | '%' => {
                            break;
                        }
                        _ => break,
                    }
                }

                if let Some(&type_ch) = chars.peek() {
                    chars.next();
                    match type_ch {
                        's' => {
                            let s = args.arg::<*const c_char>();
                            let mut str_val = heapless::String::<256>::new();
                            if !s.is_null() {
                                if let Ok(c_str) = CStr::from_ptr(s).to_str() {
                                    str_val.push_str(c_str).ok();
                                }
                            }
                            // Apply width and alignment
                            if width > 0 && str_val.len() < width {
                                if left_align {
                                    let _ = result.push_str(&str_val);
                                    for _ in 0..(width - str_val.len()) {
                                        let _ = result.push(' ');
                                    }
                                } else {
                                    for _ in 0..(width - str_val.len()) {
                                        let _ = result.push(' ');
                                    }
                                    let _ = result.push_str(&str_val);
                                }
                            } else {
                                let _ = result.push_str(&str_val);
                            }
                        }
                        'd' | 'i' => {
                            let val = args.arg::<i32>();
                            let mut num_str = heapless::String::<32>::new();
                            let _ = write!(num_str, "{}", val);
                            // Apply width and zero-padding
                            if width > 0 && num_str.len() < width {
                                if zero_pad && !left_align {
                                    for _ in 0..(width - num_str.len()) {
                                        let _ = result.push('0');
                                    }
                                } else if !zero_pad && !left_align {
                                    for _ in 0..(width - num_str.len()) {
                                        let _ = result.push(' ');
                                    }
                                }
                            }
                            let _ = result.push_str(&num_str);
                            if width > 0 && num_str.len() < width && left_align {
                                for _ in 0..(width - num_str.len()) {
                                    let _ = result.push(' ');
                                }
                            }
                        }
                        'u' => {
                            let val = args.arg::<u32>();
                            let mut num_str = heapless::String::<32>::new();
                            let _ = write!(num_str, "{}", val);
                            // Apply width and zero-padding
                            if width > 0 && num_str.len() < width {
                                if zero_pad && !left_align {
                                    for _ in 0..(width - num_str.len()) {
                                        let _ = result.push('0');
                                    }
                                } else if !zero_pad && !left_align {
                                    for _ in 0..(width - num_str.len()) {
                                        let _ = result.push(' ');
                                    }
                                }
                            }
                            let _ = result.push_str(&num_str);
                            if width > 0 && num_str.len() < width && left_align {
                                for _ in 0..(width - num_str.len()) {
                                    let _ = result.push(' ');
                                }
                            }
                        }
                        'x' => {
                            let val = if length_modifier >= 2 {
                                args.arg::<u64>() as u32 // For ll, we'll truncate (simplified)
                            } else if length_modifier == 1 {
                                args.arg::<usize>() as u32 // For l, treat as usize
                            } else {
                                args.arg::<u32>()
                            };
                            let mut num_str = heapless::String::<32>::new();
                            let _ = write!(num_str, "{:x}", val);
                            // Apply width and zero-padding
                            if width > 0 && num_str.len() < width {
                                if zero_pad && !left_align {
                                    for _ in 0..(width - num_str.len()) {
                                        let _ = result.push('0');
                                    }
                                } else if !zero_pad && !left_align {
                                    for _ in 0..(width - num_str.len()) {
                                        let _ = result.push(' ');
                                    }
                                }
                            }
                            let _ = result.push_str(&num_str);
                            if width > 0 && num_str.len() < width && left_align {
                                for _ in 0..(width - num_str.len()) {
                                    let _ = result.push(' ');
                                }
                            }
                        }
                        'X' => {
                            let val = if length_modifier >= 2 {
                                args.arg::<u64>() as u32 // For ll, we'll truncate (simplified)
                            } else if length_modifier == 1 {
                                args.arg::<usize>() as u32 // For l, treat as usize
                            } else {
                                args.arg::<u32>()
                            };
                            let mut num_str = heapless::String::<32>::new();
                            let _ = write!(num_str, "{:X}", val);
                            // Apply width and zero-padding
                            if width > 0 && num_str.len() < width {
                                if zero_pad && !left_align {
                                    for _ in 0..(width - num_str.len()) {
                                        let _ = result.push('0');
                                    }
                                } else if !zero_pad && !left_align {
                                    for _ in 0..(width - num_str.len()) {
                                        let _ = result.push(' ');
                                    }
                                }
                            }
                            let _ = result.push_str(&num_str);
                            if width > 0 && num_str.len() < width && left_align {
                                for _ in 0..(width - num_str.len()) {
                                    let _ = result.push(' ');
                                }
                            }
                        }
                        'p' => {
                            let val = args.arg::<*const core::ffi::c_void>();
                            let mut num_str = heapless::String::<32>::new();
                            let _ = write!(num_str, "{:p}", val);
                            let _ = result.push_str(&num_str);
                        }
                        '%' => {
                            let _ = result.push('%');
                        }
                        _ => {
                            // Unknown type, output the %
                            let _ = result.push('%');
                        }
                    }
                } else {
                    // No type found, just output %
                    let _ = result.push('%');
                }
            } else {
                let _ = result.push(ch);
            }
        }

        // Copy formatted string to buffer
        let formatted_bytes = result.as_bytes();
        let copy_len = if formatted_bytes.len() < size - 1 {
            formatted_bytes.len()
        } else {
            size - 1
        };

        core::ptr::copy_nonoverlapping(formatted_bytes.as_ptr(), buffer, copy_len);
        *buffer.add(copy_len) = 0; // null terminate

        formatted_bytes.len() as c_int
    } else {
        0
    }
}
