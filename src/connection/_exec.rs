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
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::Result {
        let (values, formats, lengths) =
            Self::transform_params(param_values, param_formats);

        if log::log_enabled!(log::Level::Debug) {
            use std::convert::TryFrom;

            let mut p = Vec::new();

            for (x, value) in param_values.iter().enumerate() {
                let v = if let Some(s) = value {
                    String::from_utf8(s.to_vec()).unwrap_or_else(|_| "?".to_string())
                } else {
                    "null".to_string()
                };
                let default_type = crate::types::TEXT;
                let t = crate::Type::try_from(
                    *param_types.get(x).unwrap_or(&default_type.oid)
                ).unwrap_or(default_type);

                p.push(format!("'{}'::{}", v, t.name));
            }

            log::debug!("Execute query '{}' with params [{}]", command, p.join(", "));
        }

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
    ) -> crate::Result {
        log::debug!(
            "Prepare {} query '{}' with param types [{}]",
            name.unwrap_or("anonymous"),
            query,
            param_types
                .iter()
                .map(|oid| {
                    use std::convert::TryFrom;

                    let t = crate::Type::try_from(*oid)
                        .unwrap_or(crate::types::UNKNOWN);

                    t.name
                })
                .collect::<Vec<_>>()
                .join(", ")
        );

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

        let (values, formats, lengths) =
            Self::transform_params(param_values, param_formats);

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
    pub fn describe_prepared(&self, name: Option<&str>) -> crate::Result {
        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        unsafe { pq_sys::PQdescribePrepared(self.into(), c_name.as_ptr()) }
            .into()
    }

    /**
     * Submits a request to obtain information about the specified portal, and waits for completion.
     *
     * See [PQdescribePortal](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPORTAL).
     */
    pub fn describe_portal(&self, name: Option<&str>) -> crate::Result {
        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        unsafe { pq_sys::PQdescribePortal(self.into(), c_name.as_ptr()) }
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
