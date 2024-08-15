use std::os::raw;

/**
 * PqBytes is used as smart pointer to `std::ffi::c_void` pointer that was allocated by libpq.
 *
 * It frees the memory using libpq.PQfreemem when it is dropped.
 *
 * This struct is not cloneable since it has a raw pointer.
 * If you need to clone it, you must convert it to a owned one by calling [`to_vec()`](#method.to_vec)
 * or use [`std::rc::Rc`].
 *
 * # Examples
 *
 * ```
 * // Connect to postgres
 * let dsn = "host=localhost";
 * # let dsn = std::env::var("PQ_DSN").unwrap_or_else(|_| "host=localhost".to_string());
 * let conn = libpq::Connection::new(&dsn).expect("Failed to connect to postgres");
 *
 * // Create temporary table
 * conn.exec("CREATE TEMPORARY TABLE tmp (id INTEGER);");
 *
 * // Create some data
 * conn.exec("COPY tmp (id) FROM STDIN;");
 * conn.put_copy_data(b"1\n2\n3\n4\n").expect("Error while sending data");
 * conn.put_copy_end(None).expect("Error while sending end of data indication");
 *
 * // Read the data
 * conn.exec("COPY tmp TO STDOUT;");
 *
 * // PqBytes implements Deref<Target = [u8]]>, so it is coerced to &[u8] slice ...
 * let buffer = conn.copy_data(false).expect("Error while reading data");
 * assert_eq!(&*buffer, b"1\n");
 *
 * // ... having all the same methods from &[u8] slice ...
 * let buffer = conn.copy_data(false).expect("Error while reading data");
 * assert_eq!(buffer.to_vec(), vec![b'2', b'\n']);
 * assert_eq!(buffer.len(), 2);
 * assert_eq!(buffer.last(), Some(&b'\n'));
 * // ... and traits like Index ...
 * assert_eq!(buffer[0], b'2');
 *
 * // ... or being used in any function that accepts &[u8] slice ...
 * let buffer = conn.copy_data(false).expect("Error while reading data");
 * fn work_on_u8_slice(b: &[u8]) {
 *     assert_eq!(b, b"3\n");
 * }
 * work_on_u8_slice(&buffer);
 *
 * // ... like String::from_utf8_lossy which requires a &[u8]
 * let buffer = conn.copy_data(false).expect("Error while reading data");
 * assert_eq!(String::from_utf8_lossy(&buffer), "4\n");
 * ```
 *
 * See [`String::from_utf8_lossy`], [`crate::Connection::copy_data`] and [`slice`].
 */
#[derive(Debug)]
pub struct PqBytes {
    ptr: *const u8,
    len: usize,
}

impl std::ops::Deref for PqBytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        // SAFETY: This is safe because:
        // * We know that the pointer/len is/has valid/proper size/align since struct
        //   can only be created by us from a valid pointer/len returned by libpq.
        //   If we trust libpq, we can trust this method.
        // * The lifetime of returned slice is bounded to the lifetime of this
        //   struct (elided).
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for PqBytes {
    fn drop(&mut self) {
        // SAFETY: This is safe because:
        // * This is the recommended way to free memory allocated by libpq.
        // * We know that the pointer is valid since was allocated by libpq.
        // * We know that the pointer is not null since we checked it on creation.
        // * This struct is not cloneable, so it is only called once for that pointer
        //   when there are no more references to it.
        unsafe {
            pq_sys::PQfreemem(self.ptr as *mut std::ffi::c_void);
        };
    }
}

impl PqBytes {
    pub(crate) fn from_raw(ptr: *const u8, len: usize) -> PqBytes {
        debug_assert!(!ptr.is_null(), "Buffer ptr must be not null");
        debug_assert!(len > 0, "Buffer length must be greater than 0");
        debug_assert!(
            len <= isize::MAX as usize,
            "Buffer length must be less than isize::MAX"
        );
        PqBytes { ptr, len }
    }
}

/**
* PqString is used as smart pointer to `std::os::raw::c_char` pointer that was allocated by libpq.
*
* It frees the memory using libpq.PQfreemem when it is dropped.
*
* This struct is not cloneable since it has a raw pointer.
* If you need to clone it, you must convert to a owned one by calling
* [`to_owned()`](std::ffi::CStr::to_owned) or use [`std::rc::Rc`].
* # Examples
*
* ```
* # // Since there isn't a public method to create PqString, let's
* # // create a CString and transmute it to PqString.
* # let cstring = std::ffi::CString::new("Something returned by postgres").unwrap();
* # let s = unsafe {std::mem::transmute::<_, libpq::connection::PqString>(cstring.as_ptr())};
* // let s = /* ... "Something returned by postgres" ... */
*
* // PqString implements Deref<Target = CStr>, so it is coerced
* // to &CStr and it has all the same methods ...
* assert_eq!(s.to_bytes(), b"Something returned by postgres");
* assert_eq!(s.to_string_lossy(), "Something returned by postgres");
* // ... and traits like ToOwned
* assert_eq!(s.to_owned(), std::ffi::CString::new("Something returned by postgres").unwrap());
*
* // We can use to_str() to safe convert it a rust &str slice ...
* assert_eq!(s.to_str().ok(), Some("Something returned by postgres"));
* // .. and use to_string() on it to create a owned Rust String.
* assert_eq!(s.to_str().unwrap().to_string(), String::from("Something returned by postgres"));
*
* // Since PqString implements AsRef<[u8]>, is can be used in any method that requires &[u8] ...
* fn work_on_bytes<T: AsRef<[u8]>>(input: &T) {
*    assert_eq!(input.as_ref(), b"Something returned by postgres");
* }
* work_on_bytes(&s);
* // .. including String::from_utf8_lossy
* assert_eq!(String::from_utf8_lossy(s.as_ref()), "Something returned by postgres");
*
* # // We don't want to libpq to free the memory that does not belong to it, so we must use ManuallyDrop.
* # std::mem::ManuallyDrop::new(s);
* ```
*
* See [`std::ffi::CStr`].
*/
#[derive(Debug)]
pub struct PqString {
    ptr: *const raw::c_char,
}

impl std::ops::Deref for PqString {
    type Target = std::ffi::CStr;

    fn deref(&self) -> &std::ffi::CStr {
        // SAFETY: This is safe because:
        // * We know that the pointer is valid and it is aligned since this struct
        //   can only be created by us from a valid pointer returned by libpq.
        //   If we trust libpq, we can trust this method.
        // * We know that the pointer has '\0' at the end and does not contain
        //   any other interior '\0' since that libpq assures this to us.
        // * The lifetime of CStr is bounded to the lifetime of this struct (elided).
        // * The ptr is not changed during the lifetime of the struct.
        unsafe { std::ffi::CStr::from_ptr(self.ptr) }
    }
}

impl AsRef<[u8]> for PqString {
    fn as_ref(&self) -> &[u8] {
        self.to_bytes()
    }
}

impl Drop for PqString {
    fn drop(&mut self) {
        // SAFETY: This is safe because:
        // * This is the recommended way to free memory allocated by libpq.
        // * We know that the pointer is valid since was allocated by libpq.
        // * We know that the pointer is not null since we checked it on creation.
        // * This struct is not cloneable, so it is only called once for that pointer when
        //   there are no more references to it.
        unsafe {
            pq_sys::PQfreemem(self.ptr as *mut std::ffi::c_void);
        };
    }
}

impl PqString {
    pub(crate) fn from_raw(ptr: *const raw::c_char) -> PqString {
        debug_assert!(!ptr.is_null(), "ptr must be not null");
        PqString { ptr }
    }

    /**
     * Yields a &str slice without checking that the _CStrBuffer contains valid UTF-8
     * and without issuing new allocations.
     *
     * To safe version, see [`to_str`](#method.to_str).
     *
     * # Safety
     *
     * This function is a shortcut to [`std::str::from_utf8_unchecked`] and
     * same safety caveats apply.
     *
     * In Rust, strings are composed of a slice of u8 and are guaranteed to be valid UTF-8,
     * which allows for NUL bytes in the interior of the string. In C (thus libpq), strings
     * are just pointers to a char and are terminated by a NUL byte (with the integer value 0).
     * So, some work is needed to convert between these two representations.
     *
     * If the connection client encoding is UTF-8, this function could be safe to use
     * since lipq does the conversion for us and we could trust in libpq.
     * If the connection client encoding is not UTF-8, this function is not safe to use
     * without additional checking. If this is the case, you should use
     * [`to_str`](#method.to_str).
     *
     * See [`Connection::set_client_encoding`](crate::Connection::set_client_encoding) and
     * [`Connection::client_encoding`](crate::Connection::client_encoding).
     */

    pub unsafe fn to_str_unchecked(&self) -> &str {
        std::str::from_utf8_unchecked(self.as_ref())
    }
}
