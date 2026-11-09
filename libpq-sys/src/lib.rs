#![allow(clippy::manual_div_ceil)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::possible_missing_else)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    #[test]
    fn test_ssl_init() {
        unsafe {
            crate::PQinitSSL(1);
        }
    }
}
