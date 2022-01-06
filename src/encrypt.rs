/**
 * Prepares the md5-encrypted form of a PostgreSQL password.
 *
 * See [PQencryptPassword](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQENCRYPTPASSWORD).
 */
#[deprecated(note = "Use libpq::Connection::encrypt_password instead")]
pub fn password(passwd: &str, user: &str) -> String {
    let c_passwd = crate::ffi::to_cstr(passwd);
    let c_user = crate::ffi::to_cstr(user);

    unsafe {
        let ptr = pq_sys::PQencryptPassword(c_passwd.as_ptr(), c_user.as_ptr());
        let encrypt = std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_string();
        pq_sys::PQfreemem(ptr as *mut std::ffi::c_void);

        encrypt
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn password() {
        #![allow(deprecated)]
        assert_eq!(
            crate::encrypt::password("1234", "postgres"),
            "md524bb002702969490e41e26e1a454036c".to_string()
        );
    }
}
