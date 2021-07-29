use std::os::raw::c_char;

pub(crate) fn to_cstr(s: &str) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap()
}

pub(crate) fn to_str(s: *const c_char) -> &'static str {
    let buffer = unsafe { std::ffi::CStr::from_ptr(s) };

    buffer.to_str().unwrap()
}

pub(crate) fn to_string(s: *const c_char) -> String {
    to_str(s).to_string()
}

pub(crate) fn to_option_str(s: *const c_char) -> Option<&'static str> {
    if s.is_null() {
        return None;
    }

    let s = to_str(s);

    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub(crate) fn to_option_string(s: *const c_char) -> Option<String> {
    to_option_str(s).map(String::from)
}

pub(crate) fn from_raw(raw: *mut c_char) -> String {
    unsafe {
        std::ffi::CString::from_raw(raw)
            .to_str()
            .unwrap()
            .to_string()
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

pub(crate) fn new_cstring(size: usize) -> std::ffi::CString {
    unsafe { std::ffi::CString::from_vec_unchecked(vec![0; size]) }
}

pub(crate) fn vec_to_nta<S: ToString>(v: &[S]) -> (Vec<std::ffi::CString>, Vec<*const c_char>) {
    let c = v
        .iter()
        .map(|x| crate::ffi::to_cstr(&x.to_string()))
        .collect::<Vec<_>>();
    let mut ptr = c.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
    ptr.push(std::ptr::null());

    (c, ptr)
}
