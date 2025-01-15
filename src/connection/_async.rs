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
    pub fn send_query(&self, command: &str) -> crate::errors::Result {
        log::trace!("Sending query '{command}'");

        let c_command = crate::ffi::to_cstr(command);

        let success = unsafe { pq_sys::PQsendQuery(self.into(), c_command.as_ptr()) };

        if success == 1 {
            Ok(())
        } else {
            self.error()
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
        param_types: &[crate::Oid],
        param_values: &[Option<&[u8]>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::errors::Result {
        let (values, formats, lengths) = Self::transform_params(param_values, param_formats);

        Self::trace_query("Sending", command, param_types, param_values, param_formats);

        let c_command = crate::ffi::to_cstr(command);

        let success = unsafe {
            pq_sys::PQsendQueryParams(
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
        };

        if success == 1 {
            Ok(())
        } else {
            self.error()
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
        param_types: &[crate::Oid],
    ) -> crate::errors::Result {
        let prefix = format!("Sending prepare {}", name.unwrap_or("anonymous"));
        Self::trace_query(&prefix, query, param_types, &[], &[]);

        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());
        let c_query = crate::ffi::to_cstr(query);

        let success = unsafe {
            pq_sys::PQsendPrepare(
                self.into(),
                c_name.as_ptr(),
                c_query.as_ptr(),
                param_types.len() as i32,
                param_types.as_ptr(),
            )
        };

        if success == 1 {
            Ok(())
        } else {
            self.error()
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
        param_values: &[Option<&[u8]>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::errors::Result {
        let prefix = format!("Send {} prepared query", name.unwrap_or("anonymous"));
        Self::trace_query(&prefix, "", &[], param_values, param_formats);

        let (values, formats, lengths) = Self::transform_params(param_values, param_formats);

        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        let success = unsafe {
            pq_sys::PQsendQueryPrepared(
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
        };

        if success == 1 {
            Ok(())
        } else {
            self.error()
        }
    }

    /**
     * Submits a request to obtain information about the specified prepared statement, without waiting for completion.
     *
     * See [PQsendDescribePortal](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPORTAL).
     */
    pub fn send_describe_prepared(&self, name: Option<&str>) -> crate::errors::Result {
        log::trace!(
            "Sending describe prepared query {}",
            name.unwrap_or("anonymous")
        );

        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        let success = unsafe { pq_sys::PQsendDescribePrepared(self.into(), c_name.as_ptr()) };

        if success == 1 {
            Ok(())
        } else {
            self.error()
        }
    }

    /**
     * Submits a request to obtain information about the specified portal, without waiting for completion.
     *
     * See
     * [PQsendDescribePortal](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPORTAL).
     */
    pub fn send_describe_portal(&self, name: Option<&str>) -> crate::errors::Result {
        log::trace!("Sending describe portal {}", name.unwrap_or("anonymous"));

        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        let success = unsafe { pq_sys::PQsendDescribePortal(self.into(), c_name.as_ptr()) };

        if success == 1 {
            Ok(())
        } else {
            self.error()
        }
    }

    /**
     * Waits for the next result a prior `send_*` call, and returns it.
     *
     * See [PQgetResult](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQGETRESULT).
     */
    pub fn result(&self) -> Option<crate::PQResult> {
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
    pub fn consume_input(&self) -> crate::errors::Result {
        log::trace!("Consume input");

        let success = unsafe { pq_sys::PQconsumeInput(self.into()) };

        if success == 1 {
            Ok(())
        } else {
            self.error()
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
    pub fn set_non_blocking(&self, non_blocking: bool) -> crate::errors::Result {
        if non_blocking {
            log::trace!("Set non blocking");
        } else {
            log::trace!("Set blocking");
        }

        let status = unsafe { pq_sys::PQsetnonblocking(self.into(), non_blocking as i32) };

        if status < 0 {
            self.error()
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
    pub fn flush(&self) -> crate::errors::Result {
        log::trace!("Flush");

        let status = unsafe { pq_sys::PQflush(self.into()) };

        if status == 0 {
            Ok(())
        } else {
            self.error()
        }
    }

    /**
     * Submits a request to close the specified prepared statement, without waiting for completion.
     *
     * See
     * [PQsendClosePrepared](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDCLOSEPREPARED).
     */
    #[cfg(feature = "v17")]
    pub fn send_close_prepared(&self, name: Option<&str>) -> crate::errors::Result {
        log::trace!("Send close prepared {:?}", name.unwrap_or_default());
        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        let status = unsafe { pq_sys::PQsendClosePrepared(self.into(), c_name.as_ptr()) };

        if status == 0 {
            Ok(())
        } else {
            self.error()
        }
    }

    /**
     * Submits a request to close specified portal, without waiting for completion.
     *
     * See
     * [PQsendClosePortal](https://www.postgresql.org/docs/currencurrentt/libpq-async.html#LIBPQ-PQSENDCLOSEPORTAL).
     */
    #[cfg(feature = "v17")]
    pub fn send_close_portal(&self, name: Option<&str>) -> crate::errors::Result {
        log::trace!("Send close portal {:?}", name.unwrap_or_default());

        let c_name = crate::ffi::to_cstr(name.unwrap_or_default());

        let status = unsafe { pq_sys::PQsendClosePortal(self.into(), c_name.as_ptr()) };

        if status == 1 {
            Ok(())
        } else {
            self.error()
        }
    }
}
