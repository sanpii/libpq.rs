#[derive(Clone, Debug, Eq, PartialEq)]
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
    #[deprecated(since = "4.1.0", note = "Use Info::defaults() instead")]
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Returns the default connection options.
     *
     * See [PQconndefaults](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
     */
    pub fn defaults() -> crate::errors::Result<Vec<Self>> {
        unsafe {
            let raw = pq_sys::PQconndefaults();
            let info = Self::vec_from_nta(raw);
            pq_sys::PQconninfoFree(raw);

            info
        }
    }

    /**
     * Returns parsed connection options from the provided connection string.
     *
     * See
     * [PQconninfoParse](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE).
     */
    pub fn from(dsn: &str) -> crate::errors::Result<Vec<Self>> {
        let c_dsn = crate::ffi::to_cstr(dsn);

        unsafe {
            let mut errmsg: *mut libc::c_char = std::ptr::null_mut();
            let raw = pq_sys::PQconninfoParse(c_dsn.as_ptr(), &mut errmsg);

            if raw.is_null() {
                if errmsg.is_null() {
                    return Err(crate::errors::Error::Unknow);
                } else {
                    let err = crate::ffi::to_string(errmsg)?;
                    pq_sys::PQfreemem(errmsg as *mut std::ffi::c_void);
                    return Err(crate::errors::Error::Backend(err));
                }
            }

            let info = Self::vec_from_nta(raw);
            pq_sys::PQconninfoFree(raw);

            info
        }
    }

    fn from_raw(raw: *mut pq_sys::_PQconninfoOption) -> crate::errors::Result<Self> {
        let info = unsafe {
            Self {
                keyword: crate::ffi::to_string((*raw).keyword)?,
                envvar: if (*raw).envvar.is_null() {
                    None
                } else {
                    Some(crate::ffi::to_string((*raw).envvar)?)
                },
                compiled: if (*raw).compiled.is_null() {
                    None
                } else {
                    Some(crate::ffi::to_string((*raw).compiled)?)
                },
                val: if (*raw).val.is_null() {
                    None
                } else {
                    Some(crate::ffi::to_string((*raw).val)?)
                },
                label: if (*raw).label.is_null() {
                    None
                } else {
                    Some(crate::ffi::to_string((*raw).label)?)
                },
                dispchar: crate::ffi::to_string((*raw).dispchar)?,
                dispsize: (*raw).dispsize,
            }
        };

        Ok(info)
    }

    fn vec_from_nta(raw: *mut pq_sys::_PQconninfoOption) -> crate::errors::Result<Vec<Self>> {
        let mut vec = Vec::new();

        for x in 0.. {
            unsafe {
                if (*raw.offset(x)).keyword.is_null() {
                    break;
                } else {
                    let info = raw.offset(x).try_into()?;
                    vec.push(info);
                }
            }
        }

        Ok(vec)
    }
}

impl Default for Info {
    fn default() -> Self {
        unsafe {
            let raw = pq_sys::PQconndefaults();
            let info = raw.try_into().unwrap();
            pq_sys::PQconninfoFree(raw);

            info
        }
    }
}

#[doc(hidden)]
impl TryFrom<*mut pq_sys::_PQconninfoOption> for Info {
    type Error = crate::errors::Error;

    fn try_from(info: *mut pq_sys::_PQconninfoOption) -> Result<Self, Self::Error> {
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
            Err(crate::errors::Error::Backend(
                "missing \"=\" after \"'\" in connection info string\n".to_string()
            ))
        );
    }

    #[test]
    fn defaults() {
        let _ = crate::connection::Info::defaults();
    }
}
