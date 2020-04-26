/**
 * [Control Functions](https://www.postgresql.org/docs/current/libpq-control.html)
 */
impl Connection {
    /**
     * Returns the client encoding.
     *
     * See
     * [PQclientEncoding](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQCLIENTENCODING).
     */
    pub fn client_encoding(&self) -> crate::Encoding {
        unsafe { pq_sys::PQclientEncoding(self.into()) }.into()
    }

    /**
     * Sets the client encoding.
     *
     * See [PQsetClientEncoding](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQSETCLIENTENCODING).
     */
    pub fn set_client_encoding(&self, encoding: crate::Encoding) {
        log::debug!("Setting client encoding to '{:?}'", encoding);

        unsafe {
            pq_sys::PQsetClientEncoding(self.into(), crate::cstr!(&encoding.to_string()));
        }
    }

    /**
     * Determines the verbosity of messages returned by `libpq::Connection::error_message` and
     * `libpq::Result::error_message`.
     *
     * See [PQsetErrorVerbosity](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQSETERRORVERBOSITY).
     */
    pub fn set_error_verbosity(&self, verbosity: crate::Verbosity) -> crate::Verbosity {
        log::debug!("Setting client encoding to '{:?}'", verbosity);

        unsafe { pq_sys::PQsetErrorVerbosity(self.into(), verbosity.into()) }.into()
    }

    /**
     * Enables tracing of the client/server communication to a debugging file stream.
     *
     * See [PQtrace](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQTRACE).
     */
    pub fn trace(&self, file: std::fs::File) {
        use std::os::unix::io::IntoRawFd;

        log::debug!("Enable trace");

        unsafe {
            let stream = libc::fdopen(file.into_raw_fd(), crate::cstr!("w"));
            pq_sys::PQtrace(self.into(), stream as *mut pq_sys::__sFILE);
        }
    }

    /**
     * Disables tracing started by `libpq::Connection::trace`.
     *
     * See [PQuntrace](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQUNTRACE).
     */
    pub fn untrace(&self) {
        log::debug!("Disable trace");

        unsafe {
            pq_sys::PQuntrace(self.into());
        }
    }
}
