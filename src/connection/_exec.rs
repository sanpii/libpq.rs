/**
 * [Command Execution Functions](https://www.postgresql.org/docs/current/libpq-exec.html)
 */
impl Connection {
    /**
     * Submits a command to the server and waits for the result.
     *
     * See [PQexec](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXEC).
     */
    pub fn exec(&self, query: &str) -> crate::PQResult {
        log::trace!("Execute query '{query}'");

        let c_query = crate::ffi::to_cstr(query);
        unsafe { pq_sys::PQexec(self.into(), c_query.as_ptr()) }.into()
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
        param_values: &[Option<&[u8]>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::PQResult {
        let (values, formats, lengths) = Self::transform_params(param_values, param_formats);

        Self::trace_query("Sending", command, param_types, param_values, param_formats);

        let c_command = crate::ffi::to_cstr(command);

        unsafe {
            pq_sys::PQexecParams(
                self.into(),
                c_command.as_ptr(),
                values.len() as i32,
                if param_types.is_empty() {
                    std::ptr::null()
                } else {
                    param_types.as_ptr()
                },
                values.as_ptr(),
                if lengths.is_empty() {
                    std::ptr::null()
                } else {
                    lengths.as_ptr()
                },
                if formats.is_empty() {
                    std::ptr::null()
                } else {
                    formats.as_ptr()
                },
                result_format as i32,
            )
        }
        .into()
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
    ) -> crate::PQResult {
        let prefix = format!("Prepare {}", name.unwrap_or("anonymous"));
        Self::trace_query(&prefix, query, param_types, &[], &[]);

        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());
        let c_query = crate::ffi::to_cstr(query);

        unsafe {
            pq_sys::PQprepare(
                self.into(),
                c_name.as_ptr(),
                c_query.as_ptr(),
                param_types.len() as i32,
                param_types.as_ptr(),
            )
        }
        .into()
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
        param_values: &[Option<&[u8]>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::PQResult {
        let prefix = format!("Execute {} prepared query", name.unwrap_or("anonymous"));
        Self::trace_query(&prefix, "", &[], param_values, param_formats);

        let (values, formats, lengths) = Self::transform_params(param_values, param_formats);

        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        unsafe {
            pq_sys::PQexecPrepared(
                self.into(),
                c_name.as_ptr(),
                values.len() as i32,
                values.as_ptr(),
                if lengths.is_empty() {
                    std::ptr::null()
                } else {
                    lengths.as_ptr()
                },
                if formats.is_empty() {
                    std::ptr::null()
                } else {
                    formats.as_ptr()
                },
                result_format as i32,
            )
        }
        .into()
    }

    /**
     * Submits a request to obtain information about the specified prepared statement, and waits
     * for completion.
     *
     * See [PQdescribePrepared](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPREPARED).
     */
    pub fn describe_prepared(&self, name: Option<&str>) -> crate::PQResult {
        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        unsafe { pq_sys::PQdescribePrepared(self.into(), c_name.as_ptr()) }.into()
    }

    /**
     * Submits a request to obtain information about the specified portal, and waits for completion.
     *
     * See [PQdescribePortal](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPORTAL).
     */
    pub fn describe_portal(&self, name: Option<&str>) -> crate::PQResult {
        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        unsafe { pq_sys::PQdescribePortal(self.into(), c_name.as_ptr()) }.into()
    }

    /**
     * Escape a string for use within an SQL command.
     *
     * On success, this method returns [`PqString`].
     * See
     * [PQescapeLiteral](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPELITERAL).
     */
    pub fn escape_literal(&self, str: &str) -> crate::errors::Result<PqString> {
        crate::escape::literal(self, str)
    }

    /**
     * Escapes a string for use as an SQL identifier, such as a table, column, or function name.
     *
     * On success, this method returns [`PqString`].
     *
     * See
     * [PQescapeIdentifier](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEIDENTIFIER).
     */
    pub fn escape_identifier(&self, str: &str) -> crate::errors::Result<PqString> {
        crate::escape::identifier(self, str)
    }

    /**
     * Escape string literals, much like `libpq::Connection::literal`.
     *
     * On success, this method returns [`String`].
     *
     * See
     * [PQescapeStringConn](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPESTRINGCONN).
     */
    pub fn escape_string(&self, from: &str) -> crate::errors::Result<String> {
        crate::escape::string_conn(self, from)
    }

    /**
     * Escapes binary data for use within an SQL command with the type bytea.
     *
     * On success, this method returns [`PqBytes`].
     *
     * See
     * [PQescapeByteaConn](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEBYTEACONN).
     */
    pub fn escape_bytea(&self, from: &[u8]) -> crate::errors::Result<PqBytes> {
        crate::escape::bytea_conn(self, from)
    }

    /**
     * Submits a request to close the specified prepared statement, and waits for completion.
     *
     * See
     * [PQclosePrepared](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCLOSEPREPARED).
     */
    #[cfg(feature = "v17")]
    pub fn close_prepared(&self, name: Option<&str>) -> crate::Result {
        log::trace!("Close prepared {:?}", name.unwrap_or_default());

        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        unsafe { pq_sys::PQclosePrepared(self.into(), c_name.as_ptr()) }.into()
    }

    /**
     * Submits a request to close the specified portal, and waits for completion.
     *
     * See
     * [PQclosePortal](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCLOSEPORTAL).
     */
    #[cfg(feature = "v17")]
    pub fn close_portal(&self, name: Option<&str>) -> crate::Result {
        log::trace!("Close portal {:?}", name.unwrap_or_default());

        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        unsafe { pq_sys::PQclosePortal(self.into(), c_name.as_ptr()) }.into()
    }
}
