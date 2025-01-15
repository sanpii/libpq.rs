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
    pub fn put_copy_data(&self, buffer: &[u8]) -> crate::errors::Result {
        log::trace!("Sending copy data");

        let success = unsafe {
            pq_sys::PQputCopyData(
                self.into(),
                buffer.as_ptr() as *const raw::c_char,
                buffer.len() as i32,
            )
        };

        match success {
            -1 => self.error(),
            0 => Err(crate::errors::Error::Backend("Full buffers".to_string())),
            1 => Ok(()),
            _ => self.error(),
        }
    }

    /**
     * Sends end-of-data indication to the server during `libpq::Status::CopyIn` state.
     *
     * See
     * [PQputCopyEnd](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQPUTCOPYEND).
     */
    pub fn put_copy_end(&self, errormsg: Option<&str>) -> crate::errors::Result {
        log::trace!("End of copy");

        let cstr = errormsg.map(crate::ffi::to_cstr);
        let ptr = if let Some(ref cstr) = cstr {
            cstr.as_ptr()
        } else {
            std::ptr::null()
        };

        let success = unsafe { pq_sys::PQputCopyEnd(self.into(), ptr) };

        match success {
            -1 => self.error(),
            0 => Err(crate::errors::Error::Backend("Full buffers".to_string())),
            1 => Ok(()),
            _ => self.error(),
        }
    }

    /**
     * Receives data from the server during `libpq::Status::CopyOut` or `libpq::Status::CopyBoth` state.
     *
     * On success, this method returns [`PqBytes`].
     *
     * See
     * [PQgetCopyData](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQGETCOPYDATA)
     */
    pub fn copy_data(&self, r#async: bool) -> crate::errors::Result<PqBytes> {
        let mut ptr = std::ptr::null_mut();

        let success = unsafe { pq_sys::PQgetCopyData(self.into(), &mut ptr, r#async as i32) };

        match success {
            -2 => self.error(),
            -1 => Err(crate::errors::Error::Backend("COPY is done".to_string())),
            0 => Err(crate::errors::Error::Backend("COPY still in progress".to_string())),
            nbytes => Ok(PqBytes::from_raw(ptr as *const u8, nbytes as usize)),
        }
    }
}
