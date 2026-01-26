/*
 * Minimal platform_internal.h for Embassy (bare-metal nRF)
 * Most functionality is stubbed for single-threaded operation - Anything that is not stubbed is implemented in Rust
 */

 #ifndef _PLATFORM_INTERNAL_H
 #define _PLATFORM_INTERNAL_H
 
 #include <inttypes.h>
 #include <stdbool.h>
 #include <stdarg.h>
 #include <stddef.h>
 #include <string.h>
 #include <math.h>
 
 #ifndef BH_PLATFORM_EMBASSY
 #define BH_PLATFORM_EMBASSY
 #endif
 
 /* Single-threaded stubs - we don't have real threading */
 typedef int korp_thread;
 typedef int korp_tid;
 typedef int korp_mutex;
 typedef int korp_sem;
 
 /* Stub for rwlock (not used in single-threaded) */
 typedef struct {
     int dummy;
 } korp_rwlock;
 
 /* Stub for condition variable */
 typedef struct {
     int dummy;
 } korp_cond;
 
 /* File handle stubs (not used in embedded) */
 typedef int os_file_handle;
 typedef void *os_dir_stream;
 typedef int os_raw_file_handle;
 typedef int os_poll_file_handle;
 typedef unsigned int os_nfds_t;
 typedef int os_timespec;
 
 /* Print functions - will be implemented in Rust */
 
 /* Memory functions - will be implemented in Rust */
 void *os_malloc(unsigned size);
 void *os_realloc(void *ptr, unsigned size);
 void os_free(void *ptr);
 
 /* Time functions - will be implemented in Rust */
 uint64_t os_time_get_boot_microsecond(void);
 uint64_t os_time_get_boot_nanosecond(void);
 
 /* Thread functions - stubbed for single-threaded */
static inline int os_thread_create(korp_tid *p_tid, void *(*start)(void *), void *arg, unsigned int stack_size) {
    (void)p_tid; (void)start; (void)arg; (void)stack_size;
    return -1; /* Not supported in single-threaded */
}
 
 static inline int os_thread_join(korp_thread thread, void **retval) {
     (void)thread; (void)retval;
     return -1;
 }
 
 /* Mutex functions - stubbed (no-op in single-threaded) */
 static inline int os_mutex_init(korp_mutex *mutex) {
     (void)mutex;
     return 0; /* Always succeeds in single-threaded */
 }
 
 static inline int os_mutex_destroy(korp_mutex *mutex) {
     (void)mutex;
     return 0;
 }
 
 static inline int os_mutex_lock(korp_mutex *mutex) {
     (void)mutex;
     return 0; /* No-op in single-threaded */
 }
 
 static inline int os_mutex_unlock(korp_mutex *mutex) {
     (void)mutex;
     return 0;
 }
 
 /* Utility functions */
 static inline os_file_handle os_get_invalid_handle(void) {
     return -1;
 }
 
 static inline int os_getpagesize(void) {
     return 4096; /* Typical page size */
 }

 /* Platform initialization - no-op for bare-metal */
static inline int bh_platform_init(void) {
    return 0; /* Success */
}

/* Platform cleanup - no-op for bare-metal */
static inline void bh_platform_destroy(void) {
    /* Nothing to clean up */
}

/* Thread ID - stub for single-threaded */
static inline korp_tid os_self_thread(void) {
    return 0; /* Single thread, always return 0 */
}

/* Time - stub initially (can implement properly later) */
static inline uint64_t os_time_get_boot_us(void) {
    static uint64_t counter = 0;
    return ++counter; /* Simple incrementing counter */
}

/* Printing functions - stubbed for now, likely sth we would want to implement (in Rust) */
 /* Print functions - stubbed (no-op) for now */

static inline int os_printf(const char *format, ...) {
    (void)format;
    return 0; /* No-op */
}

static inline int os_vprintf(const char *format, va_list ap) {
    (void)format;
    (void)ap;
    return 0; /* No-op */
}

/* Minimal snprintf - handles only %s which is critical for WAMR target matching */
static inline int snprintf(char *buffer, size_t size, const char *format, ...) {
    if (size == 0 || buffer == NULL) return 0;
    
    va_list ap;
    va_start(ap, format);
    
    char *buf = buffer;
    char *end = buffer + size - 1;
    
    while (*format && buf < end) {
        if (*format == '%' && *(format + 1) == 's') {
            format += 2;
            const char *s = va_arg(ap, const char *);
            if (s) {
                while (*s && buf < end) {
                    *buf++ = *s++;
                }
            }
        } else if (*format == '%' && *(format + 1) == '%') {
            format += 2;
            *buf++ = '%';
        } else {
            *buf++ = *format++;
        }
    }
    
    *buf = '\0';
    va_end(ap);
    return buf - buffer;
}

/* vsnprintf - minimal version */
static inline int vsnprintf(char *buffer, size_t size, const char *format, va_list ap) {
    if (size == 0 || buffer == NULL) return 0;
    
    char *buf = buffer;
    char *end = buffer + size - 1;
    
    while (*format && buf < end) {
        if (*format == '%' && *(format + 1) == 's') {
            format += 2;
            const char *s = va_arg(ap, const char *);
            if (s) {
                while (*s && buf < end) {
                    *buf++ = *s++;
                }
            }
        } else {
            *buf++ = *format++;
        }
    }
    
    *buf = '\0';
    return buf - buffer;
}


// Generated C implementations

// Activate this to get real printing (and deactivate it to get a smaller footprint)

// #include <stdarg.h>

// /* Helper: write a single char to buffer if space available */
// static inline int _put_char(char **buf, char *end, char c) {
//     if (*buf < end) {
//         **buf = c;
//         (*buf)++;
//     }
//     return 1;
// }

// /* Helper: write a string to buffer */
// static inline int _put_string(char **buf, char *end, const char *s) {
//     int count = 0;
//     if (!s) s = "(null)";
//     while (*s) {
//         _put_char(buf, end, *s++);
//         count++;
//     }
//     return count;
// }

// /* Helper: write unsigned integer in given base */
// static inline int _put_uint(char **buf, char *end, unsigned long val, int base, int uppercase) {
//     char tmp[24];  /* enough for 64-bit in decimal */
//     char *p = tmp + sizeof(tmp) - 1;
//     const char *digits = uppercase ? "0123456789ABCDEF" : "0123456789abcdef";
//     int count = 0;
    
//     *p = '\0';
//     if (val == 0) {
//         *--p = '0';
//     } else {
//         while (val) {
//             *--p = digits[val % base];
//             val /= base;
//         }
//     }
//     while (*p) {
//         _put_char(buf, end, *p++);
//         count++;
//     }
//     return count;
// }

// /* Helper: write signed integer */
// static inline int _put_int(char **buf, char *end, long val) {
//     int count = 0;
//     if (val < 0) {
//         _put_char(buf, end, '-');
//         count++;
//         val = -val;
//     }
//     return count + _put_uint(buf, end, (unsigned long)val, 10, 0);
// }

// /* Minimal vsnprintf implementation */
// static inline int vsnprintf(char *buffer, size_t size, const char *format, va_list ap) {
//     char *buf = buffer;
//     char *end = buffer + size - 1;  /* leave room for null terminator */
//     int count = 0;
    
//     if (size == 0) return 0;
//     if (!format) {
//         buffer[0] = '\0';
//         return 0;
//     }
    
//     while (*format) {
//         if (*format != '%') {
//             _put_char(&buf, end, *format++);
//             count++;
//             continue;
//         }
        
//         format++;  /* skip '%' */
        
//         /* Handle %% */
//         if (*format == '%') {
//             _put_char(&buf, end, '%');
//             count++;
//             format++;
//             continue;
//         }
        
//         /* Skip flags, width, precision (simplified) */
//         while (*format == '-' || *format == '+' || *format == ' ' || 
//                *format == '#' || *format == '0') {
//             format++;
//         }
//         while (*format >= '0' && *format <= '9') format++;  /* width */
//         if (*format == '.') {
//             format++;
//             while (*format >= '0' && *format <= '9') format++;  /* precision */
//         }
        
//         /* Handle length modifiers */
//         int is_long = 0;
//         if (*format == 'l') {
//             is_long = 1;
//             format++;
//             if (*format == 'l') format++;  /* ll */
//         } else if (*format == 'z' || *format == 'h') {
//             format++;
//         }
        
//         /* Handle conversion specifier */
//         switch (*format) {
//             case 's': {
//                 const char *s = va_arg(ap, const char *);
//                 count += _put_string(&buf, end, s);
//                 break;
//             }
//             case 'd':
//             case 'i': {
//                 long val = is_long ? va_arg(ap, long) : va_arg(ap, int);
//                 count += _put_int(&buf, end, val);
//                 break;
//             }
//             case 'u': {
//                 unsigned long val = is_long ? va_arg(ap, unsigned long) : va_arg(ap, unsigned int);
//                 count += _put_uint(&buf, end, val, 10, 0);
//                 break;
//             }
//             case 'x': {
//                 unsigned long val = is_long ? va_arg(ap, unsigned long) : va_arg(ap, unsigned int);
//                 count += _put_uint(&buf, end, val, 16, 0);
//                 break;
//             }
//             case 'X': {
//                 unsigned long val = is_long ? va_arg(ap, unsigned long) : va_arg(ap, unsigned int);
//                 count += _put_uint(&buf, end, val, 16, 1);
//                 break;
//             }
//             case 'p': {
//                 void *ptr = va_arg(ap, void *);
//                 _put_char(&buf, end, '0');
//                 _put_char(&buf, end, 'x');
//                 count += 2 + _put_uint(&buf, end, (unsigned long)ptr, 16, 0);
//                 break;
//             }
//             case 'c': {
//                 char c = (char)va_arg(ap, int);
//                 _put_char(&buf, end, c);
//                 count++;
//                 break;
//             }
//             default:
//                 /* Unknown format, just output it */
//                 _put_char(&buf, end, '%');
//                 _put_char(&buf, end, *format);
//                 count += 2;
//                 break;
//         }
//         format++;
//     }
    
//     *buf = '\0';  /* null terminate */
//     return count;
// }

// /* snprintf wrapper */
// static inline int snprintf(char *buffer, size_t size, const char *format, ...) {
//     va_list ap;
//     va_start(ap, format);
//     int result = vsnprintf(buffer, size, format, ap);
//     va_end(ap);
//     return result;
// }



 #endif /* end of _PLATFORM_INTERNAL_H */