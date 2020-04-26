/**
 * [Functions Associated with the `COPY`
 * Command](https://www.postgresql.org/docs/current/libpq-copy.html)
 */
impl Connection {
    /**
     * Sends data to the server during `libpq::Status::CopyIn` state.
     *
     * See
     * [PQputCopyData](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQPUTCOPYDATA).
     */
    pub fn put_copy_data(&self, buffer: &str) -> std::result::Result<(), String> {
        log::debug!("Sending copy data");

        let success = unsafe {
            pq_sys::PQputCopyData(self.into(), crate::cstr!(buffer), buffer.len() as i32)
        };

        match success {
            -1 => Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string())),
            0 => Err("Full buffers".to_string()),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    /**
     * Sends end-of-data indication to the server during `libpq::Status::CopyIn` state.
     *
     * See
     * [PQputCopyEnd](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQPUTCOPYEND).
     */
    pub fn put_copy_end(&self, errormsg: Option<&str>) -> std::result::Result<(), String> {
        log::debug!("End of copy");

        let cstr = if let Some(msg) = errormsg {
            crate::cstr!(msg)
        } else {
            std::ptr::null()
        };

        let success = unsafe { pq_sys::PQputCopyEnd(self.into(), cstr) };

        match success {
            -1 => Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string())),
            0 => Err("Full buffers".to_string()),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    /**
     * Receives data from the server during `libpq::Status::CopyOut` state.
     *
     * See
     * [PQgetCopyData](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQGETCOPYDATA).
     */
    pub fn copy_data(&self, r#async: bool) -> std::result::Result<String, String> {
        let mut buffer = std::ffi::CString::new("").unwrap().into_raw();

        let success = unsafe { pq_sys::PQgetCopyData(self.into(), &mut buffer, r#async as i32) };

        match success {
            -2 => Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string())),
            -1 => Err("COPY is done".to_string()),
            0 => Err("COPY still in progress".to_string()),
            _ => Ok(crate::ffi::to_string(buffer)),
        }
    }
}
