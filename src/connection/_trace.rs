impl Connection {
    /**
     * Enables tracing of the client/server communication to a debugging file stream.
     *
     * See [PQtrace](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQTRACE).
     */
    #[cfg(unix)]
    pub fn trace(&self, file: std::fs::File) {
        use std::os::unix::io::IntoRawFd;

        log::trace!("Enable trace");

        let c_mode = c"w";

        unsafe {
            let stream = libc::fdopen(file.into_raw_fd(), c_mode.as_ptr());
            pq_sys::PQtrace(self.into(), stream as *mut _);
        }
    }

    /**
     * Disables tracing started by `libpq::Connection::trace`.
     *
     * See [PQuntrace](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQUNTRACE).
     */
    #[cfg(unix)]
    pub fn untrace(&self) {
        log::trace!("Disable trace");

        unsafe {
            pq_sys::PQuntrace(self.into());
        }
    }

    /**
     * Controls the tracing behavior of client/server communication.
     */
    #[cfg(feature = "v14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
    pub fn trace_set_flags(&self, flags: crate::trace::Flags) {
        unsafe {
            pq_sys::PQsetTraceFlags(self.into(), flags.bits() as i32);
        }
    }
}
