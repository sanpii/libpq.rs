use std::os::raw;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Info {
    /** The keyword of the option */
    pub keyword: String,
    /** Fallback environment variable name */
    pub envvar: Option<String>,
    /** Fallback compiled in default value */
    pub compiled: Option<String>,
    /** Option's current value, or None */
    pub val: Option<String>,
    /** Label for field in connect dialog */
    pub label: Option<String>,
    /**
     * Indicates how to display this field in a connect dialog. Values are:
     *   ""        Display entered value as is
     *   "*"       Password field - hide value
     *   "D"       Debug option - don't show by default
     */
    pub dispchar: String,
    /** Field size in characters for dialog */
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

    #[deprecated(since = "6.1.0", note = "Use Info::parse() instead")]
    pub fn from(dsn: &str) -> crate::errors::Result<Vec<Self>> {
        Self::parse(dsn)
    }

    /**
     * Returns parsed connection options from the provided connection string.
     *
     * See
     * [PQconninfoParse](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE).
     */
    pub fn parse(dsn: &str) -> crate::errors::Result<Vec<Self>> {
        let c_dsn = crate::ffi::to_cstr(dsn);

        unsafe {
            let mut errmsg: *mut raw::c_char = std::ptr::null_mut();
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
        assert!(crate::connection::Info::parse("host=localhost user=postgres").is_ok());
        assert_eq!(
            crate::connection::Info::parse("'"),
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
