/**
 * [Command Execution Functions](https://www.postgresql.org/docs/current/libpq-exec.html)
 */
impl Connection {
    /**
     * Submits a command to the server and waits for the result.
     *
     * See [PQexec](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXEC).
     */
    pub fn exec(&self, query: &str) -> crate::Result {
        match self.send_query(query) {
            Ok(_) => self.result().unwrap_or_default(),
            Err(err) => err.into(),
        }
    }

    /**
     * Submits a command to the server and waits for the result, with the ability to pass
     * parameters separately from the SQL command text.
     *
     * See [PQexecParams](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXECPARAMS).
     */
    pub fn exec_params(
        &self,
        command: &str,
        param_types: &[crate::Oid],
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::Result {
        match self.send_query_params(command, param_types, param_values, param_formats, result_format) {
            Ok(_) => self.result().unwrap_or_default(),
            Err(err) => err.into(),
        }
    }

    /**
     * Submits a request to create a prepared statement with the given parameters, and waits for completion.
     *
     * See [PQprepare](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQPREPARE).
     */
    pub fn prepare(
        &self,
        name: Option<&str>,
        query: &str,
        param_types: &[crate::Oid],
    ) -> crate::Result {
        match self.send_prepare(name, query, param_types) {
            Ok(_) => self.result().unwrap_or_default(),
            Err(err) => err.into(),
        }
    }

    /**
     * Sends a request to execute a prepared statement with given parameters, and waits for the
     * result.
     *
     * See [PQexecPrepared](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXECPREPARED).
     */
    pub fn exec_prepared(
        &self,
        name: Option<&str>,
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::Result {
        match self.send_query_prepared(name, param_values, param_formats, result_format) {
            Ok(_) => self.result().unwrap_or_default(),
            Err(err) => err.into(),
        }
    }

    /**
     * Submits a request to obtain information about the specified prepared statement, and waits
     * for completion.
     *
     * See [PQdescribePrepared](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPREPARED).
     */
    pub fn describe_prepared(&self, name: Option<&str>) -> crate::Result {
        match self.send_describe_prepared(name) {
            Ok(_) => self.result().unwrap_or_default(),
            Err(err) => err.into(),
        }
    }

    /**
     * Submits a request to obtain information about the specified portal, and waits for completion.
     *
     * See [PQdescribePortal](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPORTAL).
     */
    pub fn describe_portal(&self, name: Option<&str>) -> crate::Result {
        match self.send_describe_portal(name) {
            Ok(_) => self.result().unwrap_or_default(),
            Err(err) => err.into(),
        }
    }

    /**
     * Escape a string for use within an SQL command.
     *
     * See
     * [PQescapeLiteral](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPELITERAL).-     */
    pub fn escape_literal(&self, str: &str) -> std::result::Result<String, crate::Error> {
        crate::escape::literal(&self, str)
    }

    /**
     * Escapes a string for use as an SQL identifier, such as a table, column, or function name.
     *
     * See
     * [PQescapeIdentifier](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEIDENTIFIER).
     */
    pub fn escape_identifier(&self, str: &str) -> std::result::Result<String, crate::Error> {
        crate::escape::identifier(&self, str)
    }

    /**
     * Escape string literals, much like `libpq::Connection::literal`.
     *
     * See
     * [PQescapeStringConn](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPESTRINGCONN).
     */
    pub fn escape_string(&self, from: &str) -> std::result::Result<String, crate::Error> {
        crate::escape::string_conn(&self, from)
    }

    /**
     * Escapes binary data for use within an SQL command with the type bytea.
     *
     * See
     * [PQescapeByteaConn](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEBYTEACONN).
     */
    pub fn escape_bytea(&self, from: &[u8]) -> std::result::Result<Vec<u8>, crate::Error> {
        crate::escape::bytea_conn(&self, from)
    }
}
