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
        self.state.read()
            .unwrap()
            .parameters
            .get("client_encoding")
            .cloned()
            .unwrap_or_else(|| "US-ASCII".to_string())
            .into()
    }

    /**
     * Sets the client encoding.
     *
     * See [PQsetClientEncoding](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQSETCLIENTENCODING).
     */
    pub fn set_client_encoding(&self, encoding: crate::Encoding) {
        log::debug!("Setting client encoding to '{:?}'", encoding);

        self.exec(&format!("set client_encoding to '{}'", encoding.to_string()));
    }

    /**
     * Determines the verbosity of messages returned by `libpq::Connection::error_message` and
     * `libpq::Result::error_message`.
     *
     * See [PQsetErrorVerbosity](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQSETERRORVERBOSITY).
     */
    pub fn set_error_verbosity(&self, verbosity: crate::Verbosity) -> crate::Verbosity {
        log::debug!("Setting client encoding to '{:?}'", verbosity);

        let old = self.state.read().unwrap().verbosity;

        self.state.write().unwrap().verbosity = verbosity;

        old
    }
}
