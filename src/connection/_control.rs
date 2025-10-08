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
        log::trace!("Setting client encoding to '{encoding:?}'");

        let c_encoding = crate::ffi::to_cstr(&encoding.to_string());

        unsafe {
            pq_sys::PQsetClientEncoding(self.into(), c_encoding.as_ptr());
        }
    }

    /**
     * Determines the verbosity of messages returned by `libpq::Connection::error_message` and
     * `libpq::Result::error_message`.
     *
     * See [PQsetErrorVerbosity](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQSETERRORVERBOSITY).
     */
    pub fn set_error_verbosity(&self, verbosity: crate::Verbosity) -> crate::Verbosity {
        log::trace!("Setting client encoding to '{verbosity:?}'");

        unsafe { pq_sys::PQsetErrorVerbosity(self.into(), verbosity.into()) }.into()
    }


    /**
     * Determines the handling of CONTEXT fields in messages returned by
     * `libpq::Connection::error_message` and `libpq::PQResult::error_message`.
     *
     * See [PQsetErrorContextVisibility](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQSETERRORCONTEXTVISIBILITY).
     */
    pub fn set_error_context_visibility(&self, visibility: crate::ContextVisibility) -> crate::ContextVisibility {
        log::trace!("Setting client context visibility to '{visibility:?}'");

        unsafe { pq_sys::PQsetErrorContextVisibility(self.into(), visibility.into()) }.into()
    }
}
