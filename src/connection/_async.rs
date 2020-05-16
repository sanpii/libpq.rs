/**
 * [Asynchronous Command Processing](https://www.postgresql.org/docs/current/libpq-async.html)
 */
impl Connection {
    /**
     * Submits a command to the server without waiting for the result(s).
     *
     * See
     * [PQsendQuery](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERY).
     */
    pub fn send_query(&self, command: &str) -> std::result::Result<(), String> {
        log::debug!("Sending query '{}'", command);

        let success = unsafe { pq_sys::PQsendQuery(self.into(), crate::cstr!(command)) };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Submits a command and separate parameters to the server without waiting for the result(s).
     *
     * See
     * [PQsendQueryParams](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERYPARAMS).
     */
    pub fn send_query_params(
        &self,
        command: &str,
        param_types: &[crate::Type],
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> std::result::Result<(), String> {
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

            log::debug!("Sending query '{}' with params [{}]", command, p.join(", "));
        }

        let success = unsafe {
            pq_sys::PQsendQueryParams(
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
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Sends a request to create a prepared statement with the given parameters, without waiting
     * for completion.
     *
     * See
     * [PQsendPrepare](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDPREPARE).
     */
    pub fn send_prepare(
        &self,
        name: Option<&str>,
        query: &str,
        param_types: &[crate::Type],
    ) -> std::result::Result<(), String> {
        log::debug!(
            "Sending prepare {} query '{}' with param types [{}]",
            name.unwrap_or("anonymous"),
            query,
            param_types
                .iter()
                .map(|x| x.name)
                .collect::<Vec<_>>()
                .join(", ")
        );

        let types = param_types.iter().map(|x| x.oid).collect::<Vec<_>>();

        let success = unsafe {
            pq_sys::PQsendPrepare(
                self.into(),
                crate::cstr!(name.unwrap_or_default()),
                crate::cstr!(query),
                types.len() as i32,
                types.as_ptr(),
            )
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Sends a request to execute a prepared statement with given parameters, without waiting for the result(s).
     *
     * See [PQsendQueryPrepared](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERYPREPARED).
     */
    pub fn send_query_prepared(
        &self,
        name: Option<&str>,
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> std::result::Result<(), String> {
        log::debug!(
            "Send {} prepared query with params [{}]",
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

        let success = unsafe {
            pq_sys::PQsendQueryPrepared(
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
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Submits a request to obtain information about the specified prepared statement, without waiting for completion.
     *
     * See [PQsendDescribePortal](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPORTAL).
     */
    pub fn send_describe_prepared(&self, name: Option<&str>) -> std::result::Result<(), String> {
        log::debug!(
            "Sending describe prepared query {}",
            name.unwrap_or("anonymous")
        );

        let success = unsafe {
            pq_sys::PQsendDescribePrepared(self.into(), crate::cstr!(name.unwrap_or_default()))
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Submits a request to obtain information about the specified portal, without waiting for completion.
     *
     * See
     * [PQsendDescribePortal](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPORTAL).
     */
    pub fn send_describe_portal(&self, name: Option<&str>) -> std::result::Result<(), String> {
        log::debug!("Sending describe portal {}", name.unwrap_or("anonymous"));

        let success = unsafe {
            pq_sys::PQsendDescribePortal(self.into(), crate::cstr!(name.unwrap_or_default()))
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Waits for the next result a prior `send_*` call, and returns it.
     *
     * See [PQgetResult](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQGETRESULT).
     */
    pub fn result(&self) -> Option<crate::Result> {
        let raw = unsafe { pq_sys::PQgetResult(self.into()) };

        if raw.is_null() {
            None
        } else {
            Some(raw.into())
        }
    }

    /**
     * If input is available from the server, consume it.
     *
     * See
     * [PQconsumeInput](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQCONSUMEINPUT).
     */
    pub fn consume_input(&self) -> std::result::Result<(), String> {
        log::debug!("Consume input");

        let success = unsafe { pq_sys::PQconsumeInput(self.into()) };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Returns `true` if a command is busy, that is, `Result` would block waiting for input.
     *
     * See [PQisBusy](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISBUSY).
     */
    pub fn is_busy(&self) -> bool {
        unsafe { pq_sys::PQisBusy(self.into()) == 1 }
    }

    /**
     * Sets the nonblocking status of the connection.
     *
     * See
     * [PQsetnonblocking](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSETNONBLOCKING).
     */
    pub fn set_non_blocking(&self, non_blocking: bool) -> std::result::Result<(), ()> {
        if non_blocking {
            log::debug!("Set non blocking");
        } else {
            log::debug!("Set blocking");
        }

        let status = unsafe { pq_sys::PQsetnonblocking(self.into(), non_blocking as i32) };

        if status < 0 {
            Err(())
        } else {
            Ok(())
        }
    }

    /**
     * Returns the blocking status of the database connection.
     *
     * See
     * [PQisnonblocking](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISNONBLOCKING).
     */
    pub fn is_non_blocking(&self) -> bool {
        unsafe { pq_sys::PQisnonblocking(self.into()) == 1 }
    }

    /**
     * Attempts to flush any queued output data to the server.
     *
     * See [PQflush](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQFLUSH).
     */
    pub fn flush(&self) -> std::result::Result<(), ()> {
        log::debug!("Flush");

        let status = unsafe { pq_sys::PQflush(self.into()) };

        if status == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}
