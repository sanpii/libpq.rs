use crate::connection::{PqBytes, PqString};

pub(crate) fn literal(conn: &crate::Connection, str: &str) -> crate::errors::Result<PqString> {
    let c_str = crate::ffi::to_cstr(str);
    unsafe {
        let raw = pq_sys::PQescapeLiteral(conn.into(), c_str.as_ptr(), str.len() as pq_sys::size_t);

        if raw.is_null() {
            conn.error()
        } else {
            Ok(PqString::from_raw(raw))
        }
    }
}

/**
 * Escape a string for use as an SQL identifier, such as a table, column, or function name.
 *
 * On success, this method returns [`PqString`].
 *
 * See [PQescapeIdentifier](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEIDENTIFIER).
 */
pub fn identifier(conn: &crate::Connection, str: &str) -> crate::errors::Result<PqString> {
    let c_str = crate::ffi::to_cstr(str);
    unsafe {
        let raw =
            pq_sys::PQescapeIdentifier(conn.into(), c_str.as_ptr(), str.len() as pq_sys::size_t);

        if raw.is_null() {
            conn.error()
        } else {
            Ok(PqString::from_raw(raw))
        }
    }
}

pub(crate) fn string_conn(conn: &crate::Connection, from: &str) -> crate::errors::Result<PqString> {
    let mut error = 0;

    // @see https://github.com/postgres/postgres/blob/REL_12_2/src/interfaces/libpq/fe-exec.c#L3329
    let cstring = crate::ffi::new_cstring(2 * from.len() + 1);
    let raw = cstring.into_raw();

    let c_from = crate::ffi::to_cstr(from);

    unsafe {
        pq_sys::PQescapeStringConn(
            conn.into(),
            raw,
            c_from.as_ptr(),
            from.len() as pq_sys::size_t,
            &mut error,
        );

        if error != 0 {
            return conn.error();
        }
    };

    Ok(PqString::from_raw(raw))
}

#[deprecated(note = "Use libpq::Connection::escape_string instead")]
pub fn string(from: &str) -> crate::errors::Result<String> {
    let c_from = crate::ffi::to_cstr(from);
    // @see https://github.com/postgres/postgres/blob/REL_12_2/src/interfaces/libpq/fe-exec.c#L3329
    let cstring = crate::ffi::new_cstring(2 * from.len() + 1);
    let raw = cstring.into_raw();

    unsafe {
        pq_sys::PQescapeString(raw, c_from.as_ptr(), from.len() as pq_sys::size_t);
    };

    crate::ffi::from_raw(raw)
}

pub(crate) fn bytea_conn(conn: &crate::Connection, from: &[u8]) -> crate::errors::Result<PqBytes> {
    unsafe {
        let mut to_len: pq_sys::size_t = 0;

        let to_ptr = pq_sys::PQescapeByteaConn(
            conn.into(),
            from.as_ptr(),
            from.len() as pq_sys::size_t,
            &mut to_len,
        );
        if to_ptr.is_null() {
            conn.error()
        } else {
            Ok(PqBytes::from_raw(to_ptr, to_len as usize))
        }
    }
}

/**
 * See [PQescapeBytea](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEBYTEA).
 *
 * On success, this method returns [`PqBytes`].
 */
#[deprecated(note = "Use libpq::Connection::escape_bytea instead")]
pub fn bytea(from: &[u8]) -> crate::errors::Result<PqBytes> {
    unsafe {
        let mut to_len: pq_sys::size_t = 0;
        let to_ptr =
            pq_sys::PQescapeBytea(from.as_ptr(), from.len() as pq_sys::size_t, &mut to_len);
        if to_ptr.is_null() {
            /* According to libpq docs (v14): `Currently, the only possible error is insufficient memory`
             * This was also confirmed by looking at the source code of PQescapeBytea.
             */
            Err(crate::errors::Error::Misc("out of memory\n".to_string()))
        } else {
            Ok(PqBytes::from_raw(to_ptr, to_len as usize))
        }
    }
}

/**
 * Converts a string representation of binary data into binary data — the reverse of
 * `libpq::Connection::escape_bytea`.
 *
 * On success, this method returns [`PqBytes`].
 *
 * See
 * [PQunescapeBytea](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQUNESCAPEBYTEA).
 */
pub fn unescape_bytea(from: &[u8]) -> crate::errors::Result<PqBytes> {
    unsafe {
        let mut len = 0;
        let tmp = pq_sys::PQunescapeBytea(from.as_ptr(), &mut len);
        if tmp.is_null() {
            Err(crate::errors::Error::Unknow)
        } else {
            Ok(PqBytes::from_raw(tmp, len as usize))
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn literal() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::literal(&conn, "foo").unwrap().as_ref(),
            b"'foo'"
        );
    }

    #[test]
    fn identifier() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::identifier(&conn, "foo")
                .unwrap()
                .to_string_lossy(),
            "\"foo\""
        );
    }

    #[test]
    fn string_conn() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::string_conn(&conn, "'foo'")
                .unwrap()
                .to_string_lossy(),
            "''foo''"
        );
    }

    #[test]
    fn string() {
        #![allow(deprecated)]
        assert_eq!(crate::escape::string("'foo'"), Ok("''foo''".to_string()));
    }

    #[test]
    fn bytea_conn() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::bytea_conn(&conn, b"\0").unwrap().as_ref(),
            b"\\x00\0"
        );
    }

    #[test]
    fn bytea() {
        #![allow(deprecated)]
        assert_eq!(
            crate::escape::bytea(b"'foo'").unwrap().as_ref(),
            b"''foo''\0"
        );
    }

    #[test]
    fn unescape_bytea() {
        #![allow(deprecated)]
        assert_eq!(
            crate::escape::bytea(b"'foo'").unwrap().as_ref(),
            b"''foo''\0"
        );
    }
}
