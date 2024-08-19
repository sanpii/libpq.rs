#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::unnecessary_operation)]
#![allow(clippy::identity_op)]

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
