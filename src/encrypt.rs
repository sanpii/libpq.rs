/**
 * Prepares the md5-encrypted form of a PostgreSQL password.
 *
 * See [PQencryptPassword](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQENCRYPTPASSWORD).
 */
pub fn password(passwd: &str, user: &str) -> String {
    crate::ffi::to_string(unsafe {
        pq_sys::PQencryptPassword(crate::cstr!(passwd), crate::cstr!(user))
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn password() {
        assert_eq!(
            crate::encrypt::password("1234", "postgres"),
            "md524bb002702969490e41e26e1a454036c".to_string()
        );
    }
}
