pub(crate) fn literal(conn: &crate::Connection, str: &str) -> std::result::Result<String, String> {
    let c_str = crate::ffi::to_cstr(str);
    unsafe {
        let raw = pq_sys::PQescapeLiteral(conn.into(), c_str.as_ptr(), str.len() as u64);

        if raw.is_null() {
            return Err(conn
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()));
        }

        let escaped = crate::ffi::to_string(raw);
        pq_sys::PQfreemem(raw as *mut std::ffi::c_void);

        Ok(escaped)
    }
}

/**
 * Escape a string for use as an SQL identifier, such as a table, column, or function name.
 *
 * See [PQescapeIdentifier](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEIDENTIFIER).
 */
pub fn identifier(conn: &crate::Connection, str: &str) -> std::result::Result<String, String> {
    let c_str = crate::ffi::to_cstr(str);
    unsafe {
        let raw = pq_sys::PQescapeIdentifier(conn.into(), c_str.as_ptr(), str.len() as u64);

        if raw.is_null() {
            return Err(conn
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()));
        }

        let escaped = crate::ffi::to_string(raw);
        pq_sys::PQfreemem(raw as *mut std::ffi::c_void);

        Ok(escaped)
    }
}

pub(crate) fn string_conn(
    conn: &crate::Connection,
    from: &str,
) -> std::result::Result<String, String> {
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
            from.len() as u64,
            &mut error,
        );

        if error != 0 {
            return Err(conn
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()));
        }
    };

    let to = crate::ffi::from_raw(raw);

    Ok(to)
}

#[deprecated(note = "Use libpq::Connection::escape_string instead")]
pub fn string(from: &str) -> String {
    let c_from = crate::ffi::to_cstr(from);
    // @see https://github.com/postgres/postgres/blob/REL_12_2/src/interfaces/libpq/fe-exec.c#L3329
    let cstring = crate::ffi::new_cstring(2 * from.len() + 1);
    let raw = cstring.into_raw();

    unsafe {
        pq_sys::PQescapeString(raw, c_from.as_ptr(), from.len() as u64);
    };

    crate::ffi::from_raw(raw)
}

pub(crate) fn bytea_conn(
    conn: &crate::Connection,
    from: &[u8],
) -> std::result::Result<Vec<u8>, String> {
    let to = unsafe {
        let mut len = 0;
        let tmp =
            pq_sys::PQescapeByteaConn(conn.into(), from.as_ptr(), from.len() as u64, &mut len);
        if tmp.is_null() {
            return Err(conn
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()));
        }
        let to = std::slice::from_raw_parts(tmp, len as usize - 1).to_vec();
        pq_sys::PQfreemem(tmp as *mut std::ffi::c_void);

        to
    };

    Ok(to)
}

/**
 * See [PQescapeBytea](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEBYTEA).
 */
#[deprecated(note = "Use libpq::Connection::escape_bytea instead")]
pub fn bytea(from: &[u8]) -> std::result::Result<Vec<u8>, String> {
    let to = unsafe {
        let mut len = 0;
        let tmp = pq_sys::PQescapeBytea(from.as_ptr(), from.len() as u64, &mut len);
        let to = std::slice::from_raw_parts(tmp, len as usize - 1).to_vec();
        pq_sys::PQfreemem(tmp as *mut std::ffi::c_void);

        to
    };

    Ok(to)
}

/**
 * Converts a string representation of binary data into binary data — the reverse of
 * `libpq::Connection::escape_bytea`.
 *
 * See
 * [PQunescapeBytea](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQUNESCAPEBYTEA).
 */
pub fn unescape_bytea(from: &[u8]) -> std::result::Result<Vec<u8>, ()> {
    let to = unsafe {
        let mut len = 0;
        let tmp = pq_sys::PQunescapeBytea(from.as_ptr(), &mut len);
        if tmp.is_null() {
            return Err(());
        }
        let to = std::slice::from_raw_parts(tmp, len as usize).to_vec();
        pq_sys::PQfreemem(tmp as *mut std::ffi::c_void);

        to
    };

    Ok(to)
}

#[cfg(test)]
mod test {
    #[test]
    fn literal() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::literal(&conn, "foo"),
            Ok("'foo'".to_string())
        );
    }

    #[test]
    fn identifier() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::identifier(&conn, "foo"),
            Ok("\"foo\"".to_string())
        );
    }

    #[test]
    fn string_conn() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::string_conn(&conn, "'foo'"),
            Ok("''foo''".to_string())
        );
    }

    #[test]
    fn string() {
        #![allow(deprecated)]
        assert_eq!(crate::escape::string("'foo'"), "''foo''".to_string());
    }

    #[test]
    fn bytea_conn() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::bytea_conn(&conn, b"\0"),
            Ok(b"\\x00".to_vec())
        );
    }

    #[test]
    fn bytea() {
        #![allow(deprecated)]
        assert_eq!(crate::escape::bytea(b"'foo'"), Ok(b"''foo''".to_vec()));
    }

    #[test]
    fn unescape_bytea() {
        #![allow(deprecated)]
        assert_eq!(crate::escape::bytea(b"'foo'"), Ok(b"''foo''".to_vec()));
    }
}
