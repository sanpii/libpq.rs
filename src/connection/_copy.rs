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
    #[deprecated(since="1.4.0", note="In v2, this function will take a &[u8], use v2::connection::put_copy_data instead")]
    pub fn put_copy_data(&self, buffer: &str) -> std::result::Result<(), String> {
        crate::v2::connection::put_copy_data(self, buffer.as_bytes())
    }

    /**
     * Sends end-of-data indication to the server during `libpq::Status::CopyIn` state.
     *
     * See
     * [PQputCopyEnd](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQPUTCOPYEND).
     */
    pub fn put_copy_end(&self, errormsg: Option<&str>) -> std::result::Result<(), String> {
        log::trace!("End of copy");

        let cstr = errormsg.map(crate::ffi::to_cstr);
        let ptr = if let Some(ref cstr) = cstr {
            cstr.as_ptr()
        } else {
            std::ptr::null()
        };

        let success = unsafe { pq_sys::PQputCopyEnd(self.into(), ptr) };

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
        let mut ptr = std::ptr::null_mut();

        let success = unsafe { pq_sys::PQgetCopyData(self.into(), &mut ptr, r#async as i32) };

        match success {
            -2 => Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string())),
            -1 => Err("COPY is done".to_string()),
            0 => Err("COPY still in progress".to_string()),
            nbytes => unsafe {
                let buffer = std::slice::from_raw_parts(ptr as *const u8, nbytes as usize).to_vec();
                pq_sys::PQfreemem(ptr as *mut std::ffi::c_void);

                Ok(String::from_utf8_lossy(&buffer).to_string())
            }
        }
    }
}
