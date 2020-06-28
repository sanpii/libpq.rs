#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub keyword: String,
    pub envvar: Option<String>,
    pub compiled: Option<String>,
    pub val: Option<String>,
    pub label: Option<String>,
    pub dispchar: String,
    pub dispsize: i32,
}

impl Info {
    /**
     * Returns the default connection options.
     *
     * See [PQconndefaults](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
     */
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Returns parsed connection options from the provided connection string.
     *
     * See
     * [PQconninfoParse](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE).
     */
    pub fn from(dsn: &str) -> std::result::Result<Vec<Self>, String> {
        let c_dsn = crate::ffi::to_cstr(dsn);

        unsafe {
            let mut errmsg: *mut i8 = std::ptr::null_mut();
            let raw = pq_sys::PQconninfoParse(c_dsn.as_ptr(), &mut errmsg);

            if raw.is_null() {
                if errmsg.is_null() {
                    return Err("Unknow error".to_string());
                } else {
                    let err = crate::ffi::to_string(errmsg);
                    pq_sys::PQfreemem(errmsg as *mut std::ffi::c_void);
                    return Err(err);
                }
            }

            let info = Self::vec_from_nta(raw);
            pq_sys::PQconninfoFree(raw);

            Ok(info)
        }
    }

    fn from_raw(info: *mut pq_sys::_PQconninfoOption) -> Self {
        unsafe {
            Self {
                keyword: crate::ffi::to_string((*info).keyword),
                envvar: if (*info).envvar.is_null() {
                    None
                } else {
                    Some(crate::ffi::to_string((*info).envvar))
                },
                compiled: if (*info).compiled.is_null() {
                    None
                } else {
                    Some(crate::ffi::to_string((*info).compiled))
                },
                val: if (*info).val.is_null() {
                    None
                } else {
                    Some(crate::ffi::to_string((*info).val))
                },
                label: if (*info).label.is_null() {
                    None
                } else {
                    Some(crate::ffi::to_string((*info).label))
                },
                dispchar: crate::ffi::to_string((*info).dispchar),
                dispsize: (*info).dispsize,
            }
        }
    }

    fn vec_from_nta(raw: *mut pq_sys::_PQconninfoOption) -> Vec<Self> {
        let mut vec = Vec::new();

        for x in 0.. {
            unsafe {
                if (*raw.offset(x)).keyword.is_null() {
                    break;
                } else {
                    let info = raw.offset(x).into();
                    vec.push(info);
                }
            }
        }

        vec
    }
}

impl Default for Info {
    fn default() -> Self {
        unsafe {
            let raw = pq_sys::PQconndefaults();
            let info = raw.into();
            pq_sys::PQconninfoFree(raw);

            info
        }
    }
}

#[doc(hidden)]
impl From<*mut pq_sys::_PQconninfoOption> for Info {
    fn from(info: *mut pq_sys::_PQconninfoOption) -> Self {
        Self::from_raw(info)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_info() {
        assert!(crate::connection::Info::from("host=localhost user=postgres").is_ok());
        assert_eq!(
            crate::connection::Info::from("'"),
            Err("missing \"=\" after \"'\" in connection info string\n".to_string())
        );
    }

    #[test]
    fn defaults() {
        let _ = crate::connection::Info::default();
    }
}
