pub(crate) fn literal(_: &crate::Connection, str: &str) -> std::result::Result<String, crate::Error> {
    escape(str, false)
}

pub(crate) fn identifier(_: &crate::Connection, str: &str) -> std::result::Result<String, crate::Error> {
    escape(str, true)
}

fn escape(str: &str, as_ident: bool) -> std::result::Result<String, crate::Error> {
    let quote_char = if as_ident {
        '"'
    } else {
        '\''
    };

    let mut s = String::new();

    s.push(quote_char);
    for c in str.chars() {
        if c == quote_char || (!as_ident && c == '\'') {
            s.push(c);
        }
        s.push(c);
    }
    s.push(quote_char);


    Ok(s)
}

pub(crate) fn string_conn(
    conn: &crate::Connection,
    from: &str,
) -> std::result::Result<String, crate::Error> {
    let mut s = String::new();
    let std_strings = conn.std_strings()?;

    for c in from.chars() {
        if c == '\'' || (std_strings && c == '\\') {
            s.push(c);
        }
        s.push(c);
    }

    Ok(s)
}

pub(crate) fn bytea_conn(
    conn: &crate::Connection,
    from: &[u8],
) -> std::result::Result<Vec<u8>, crate::Error> {
    let mut v = Vec::new();
    let std_strings = conn.std_strings()?;
    let use_hex = conn.server_version() >= 90_000;

    if use_hex {
        if !std_strings {
            v.push(b'\\');
        }
        v.push(b'\\');
        v.push(b'x');
    }

    for b in from {
        if use_hex {
            static HEXTBL: &[u8] = b"0123456789abcdef";

            v.push(HEXTBL[((b >> 4) & 0xF) as usize]);
            v.push(HEXTBL[(b & 0xF) as usize]);
        } else {
            if *b < 0x20 || *b > 0x7e {
                if !std_strings {
                    v.push(b'\\');
                }
                v.push(b'\\');
                v.push((b >> 6) + b'0');
                v.push(((b >> 3) & 07) + b'0');
                v.push((b & 07) + b'0');
            } else if *b == b'\'' {
                v.extend_from_slice(b"\\x00");
            } else if *b == b'\\' {
                if !std_strings {
                    v.extend_from_slice(b"\\\\");
                }
                v.extend_from_slice(b"\\\\");
            } else {
                v.push(*b);
            }
        }
    }

    Ok(v)
}

/**
 * Converts a string representation of binary data into binary data — the reverse of
 * `libpq::Connection::escape_bytea`.
 *
 * See
 * [PQunescapeBytea](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQUNESCAPEBYTEA).
 */
pub fn unescape_bytea(from: &[u8]) -> std::result::Result<Vec<u8>, crate::Error> {
   todo!()
}

#[cfg(test)]
mod test {
    #[test]
    fn literal() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::literal(&conn, "foo").unwrap(),
            "'foo'".to_string()
        );
    }

    #[test]
    fn identifier() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::identifier(&conn, "foo").unwrap(),
            "\"foo\"".to_string()
        );
    }

    #[test]
    fn string_conn() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::string_conn(&conn, "'foo'").unwrap(),
            "''foo''".to_string()
        );
    }

    #[test]
    fn bytea_conn() {
        let conn = crate::test::new_conn();

        assert_eq!(
            crate::escape::bytea_conn(&conn, b"\0").unwrap(),
            b"\\x00".to_vec()
        );
    }
}
