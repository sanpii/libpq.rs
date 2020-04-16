use std::os::raw::c_char;

#[macro_export]
#[doc(hidden)]
macro_rules! cstr {
    ( $s:expr ) => {
        $crate::ffi::to_cstr($s).as_ptr()
    };
}

pub(crate) fn to_cstr(s: &str) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap()
}

pub(crate) fn to_string(s: *const c_char) -> String {
    let buffer = unsafe { std::ffi::CStr::from_ptr(s) };

    buffer.to_str().unwrap().to_string()
}

pub(crate) fn to_option_string(s: *const c_char) -> Option<String> {
    let buffer = unsafe { std::ffi::CStr::from_ptr(s) };
    let s = buffer.to_str().unwrap().to_string();

    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub(crate) fn vec_from_nta(raw: *const *const c_char) -> Vec<String> {
    let mut vec = Vec::new();

    for x in 0.. {
        unsafe {
            if (*raw.offset(x)).is_null() {
                break;
            } else {
                let s = to_string(*raw.offset(x));
                vec.push(s);
            }
        }
    }

    vec
}
