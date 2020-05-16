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
        log::debug!("Execute query '{}'", query);

        unsafe { pq_sys::PQexec(self.into(), crate::cstr!(query)) }.into()
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
        param_types: &[crate::Type],
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::Result {
        let (types, values, formats, lengths) =
            Self::transform_params(param_types, param_values, param_formats);

        if log::log_enabled!(log::Level::Debug) {
            let mut p = Vec::new();

            for (x, value) in param_values.iter().enumerate() {
                let v = if let Some(s) = value {
                    String::from_utf8(s.to_vec()).unwrap_or_else(|_| "?".to_string())
                } else {
                    "null".to_string()
                };
                let t = param_types.get(x).unwrap_or_else(|| &crate::types::TEXT);

                p.push(format!("'{}'::{}", v, t.name));
            }

            log::debug!("Execute query '{}' with params [{}]", command, p.join(", "));
        }

        unsafe {
            pq_sys::PQexecParams(
                self.into(),
                crate::cstr!(command),
                values.len() as i32,
                if types.is_empty() {
                    std::ptr::null()
                } else {
                    types.as_ptr()
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
        param_types: &[crate::Type],
    ) -> crate::Result {
        log::debug!(
            "Prepare {} query '{}' with param types [{}]",
            name.unwrap_or("anonymous"),
            query,
            param_types
                .iter()
                .map(|x| x.name)
                .collect::<Vec<_>>()
                .join(", ")
        );

        let types = param_types.iter().map(|x| x.oid).collect::<Vec<_>>();

        unsafe {
            pq_sys::PQprepare(
                self.into(),
                crate::cstr!(name.unwrap_or_default()),
                crate::cstr!(query),
                types.len() as i32,
                types.as_ptr(),
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
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::Result {
        log::debug!(
            "Execute {} prepared query with params [{}]",
            name.unwrap_or("anonymous"),
            param_values
                .iter()
                .map(|x| if let Some(s) = x {
                    match String::from_utf8(s.to_vec()) {
                        Ok(str) => format!("'{}'", str),
                        Err(_) => "?".to_string(),
                    }
                } else {
                    "null".to_string()
                })
                .collect::<Vec<_>>()
                .join(", ")
        );

        let (_, values, formats, lengths) =
            Self::transform_params(&[], param_values, param_formats);

        unsafe {
            pq_sys::PQexecPrepared(
                self.into(),
                crate::cstr!(name.unwrap_or_default()),
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
    pub fn describe_prepared(&self, name: Option<&str>) -> crate::Result {
        unsafe { pq_sys::PQdescribePrepared(self.into(), crate::cstr!(name.unwrap_or_default())) }
            .into()
    }

    /**
     * Submits a request to obtain information about the specified portal, and waits for completion.
     *
     * See [PQdescribePortal](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPORTAL).
     */
    pub fn describe_portal(&self, name: Option<&str>) -> crate::Result {
        unsafe { pq_sys::PQdescribePortal(self.into(), crate::cstr!(name.unwrap_or_default())) }
            .into()
    }

    /**
     * Escape a string for use within an SQL command.
     *
     * See
     * [PQescapeLiteral](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPELITERAL).
     */
    pub fn escape_literal(&self, str: &str) -> std::result::Result<String, String> {
        crate::escape::literal(&self, str)
    }

    /**
     * Escapes a string for use as an SQL identifier, such as a table, column, or function name.
     *
     * See
     * [PQescapeIdentifier](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEIDENTIFIER).
     */
    pub fn escape_identifier(&self, str: &str) -> std::result::Result<String, String> {
        crate::escape::identifier(&self, str)
    }

    /**
     * Escape string literals, much like `libpq::Connection::literal`.
     *
     * See
     * [PQescapeStringConn](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPESTRINGCONN).
     */
    pub fn escape_string(&self, from: &str) -> std::result::Result<String, String> {
        crate::escape::string_conn(&self, from)
    }

    /**
     * Escapes binary data for use within an SQL command with the type bytea.
     *
     * See
     * [PQescapeByteaConn](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEBYTEACONN).
     */
    pub fn escape_bytea(&self, from: &[u8]) -> std::result::Result<Vec<u8>, String> {
        crate::escape::bytea_conn(&self, from)
    }
}
