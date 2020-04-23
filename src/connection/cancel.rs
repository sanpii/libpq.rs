#[derive(Clone, Debug)]
pub struct Cancel {
    cancel: *mut pq_sys::pg_cancel,
}

impl Cancel {
    /**
     * Requests that the server abandon processing of the current command.
     *
     * See [PQcancel](https://www.postgresql.org/docs/current/libpq-cancel.html#LIBPQ-PQCANCEL).
     */
    pub fn request(&self) -> std::result::Result<(), String> {
        log::debug!("Canceling");

        let capacity = 256;
        let error = std::ffi::CString::new(String::with_capacity(capacity))
            .unwrap()
            .into_raw();

        let sucess = unsafe { pq_sys::PQcancel(self.into(), error, capacity as i32) };

        if sucess == 1 {
            Ok(())
        } else {
            Err(crate::ffi::to_string(error))
        }
    }
}

#[doc(hidden)]
impl From<*mut pq_sys::pg_cancel> for Cancel {
    fn from(cancel: *mut pq_sys::pg_cancel) -> Self {
        Self { cancel }
    }
}

#[doc(hidden)]
impl Into<*mut pq_sys::pg_cancel> for &Cancel {
    fn into(self) -> *mut pq_sys::pg_cancel {
        self.cancel
    }
}

impl Drop for Cancel {
    fn drop(&mut self) {
        unsafe {
            pq_sys::PQfreeCancel(self.cancel);
        }
    }
}
