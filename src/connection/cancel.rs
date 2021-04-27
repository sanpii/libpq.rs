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
        log::trace!("Canceling");

        let capacity = 256;
        let c_error = crate::ffi::new_cstring(capacity);
        let ptr_error = c_error.into_raw();

        let sucess = unsafe { pq_sys::PQcancel(self.into(), ptr_error, capacity as i32) };
        let error = crate::ffi::from_raw(ptr_error);

        if sucess == 1 {
            Ok(())
        } else {
            Err(error)
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
impl From<&Cancel> for *mut pq_sys::pg_cancel {
    fn from(cancel: &Cancel) -> *mut pq_sys::pg_cancel {
        cancel.cancel
    }
}

impl Drop for Cancel {
    fn drop(&mut self) {
        unsafe {
            pq_sys::PQfreeCancel(self.cancel);
        }
    }
}
