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
    pub fn send_query(&self, command: &str) -> std::result::Result<(), crate::Error> {
        log::debug!("Sending query '{}'", command);

        self.send_query_start()?;

        self.send(
            crate::Message::Query(command.to_string()),
            AsyncStatus::EXECUTE,
        )
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
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> std::result::Result<(), crate::Error> {
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

            log::debug!("Sending query '{}' with params [{}]", command, p.join(", "));
        }

        self.send_query_start()?;

        self.send(
            crate::Message::parse(None, command, param_types),
            AsyncStatus::PREPARE,
        )?;

        self.send_query_prepared(None, param_values, param_formats, result_format)
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
    ) -> std::result::Result<(), crate::Error> {
        log::debug!(
            "Sending prepare {} query '{}' with param types [{}]",
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

        self.send_query_start()?;

        self.send(
            crate::Message::parse(name, query, param_types),
            AsyncStatus::PREPARE,
        )?;

        self.sync()
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
    ) -> std::result::Result<(), crate::Error> {
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

        self.send(
            crate::Message::bind(name, param_formats, param_values, result_format),
            AsyncStatus::BIND,
        )?;

        self.send(
            crate::Message::DescribePortal(None),
            AsyncStatus::DESCRIBE_ROW,
        )?;

        self.send(
            crate::Message::Execute,
            AsyncStatus::EXECUTE,
        )?;

        self.sync()
    }

    /**
     * Submits a request to obtain information about the specified prepared statement, without
     * waiting for completion.
     *
     * See [PQsendDescribePrepared](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPREPARED).
     */
    pub fn send_describe_prepared(&self, name: Option<&str>) -> std::result::Result<(), crate::Error> {
        log::debug!(
            "Sending describe prepared query {}",
            name.unwrap_or("anonymous")
        );

        self.send(
            crate::Message::DescribeStatement(name.map(|x| x.to_string())),
            AsyncStatus::DESCRIBE_STATEMENT | AsyncStatus::DESCRIBE_ROW,
        )?;

        self.sync()
    }

    /**
     * Submits a request to obtain information about the specified portal, without waiting for completion.
     *
     * See
     * [PQsendDescribePortal](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPORTAL).
     */
    pub fn send_describe_portal(&self, name: Option<&str>) -> std::result::Result<(), crate::Error> {
        log::debug!("Sending describe portal {}", name.unwrap_or("anonymous"));

        self.send(
            crate::Message::DescribePortal(name.map(|x| x.to_string())),
            AsyncStatus::DESCRIBE_ROW,
        )?;

        self.sync()
    }

    /**
     * Waits for the next result a prior `send_*` call, and returns it.
     *
     * See [PQgetResult](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQGETRESULT).
     */
    pub fn result(&self) -> Option<crate::Result> {
        self.parse_input().unwrap_or_default()
    }

    pub(crate) fn parse_input(&self) -> std::result::Result<Option<crate::Result>, crate::Error> {
        use crate::Message::*;

        self.socket.flush()?;

        let mut async_status = self.state.read()?.async_status;
        let mut result = self.state.read()?.result.clone();
        let single_row_mode = self.state.read()?.single_row_mode;

        while async_status != AsyncStatus::IDLE {
            if let Some(message) = self.socket.receive()? {
                match message {
                    AuthentificationOk(_) => (),
                    BackendKeyData(be_pid, be_key) => {
                        self.state.write()?.be_pid = be_pid;
                        self.state.write()?.be_key = be_key;
                    }
                    BindComplete => async_status.remove(AsyncStatus::BIND),
                    CommandComplete(cmd_status) => {
                        get_or_insert_default(&mut result).cmd_status = Some(cmd_status.clone());
                        if cmd_status.starts_with("BEGIN") {
                            async_status.insert(AsyncStatus::BEGIN);
                        } else if cmd_status.starts_with("COMMIT") {
                            async_status.remove(AsyncStatus::BEGIN);
                        } else if cmd_status.starts_with("COPY ") {
                            async_status.remove(AsyncStatus::COPY_IN);
                            break;
                        } else {
                            async_status.remove(AsyncStatus::EXECUTE);
                        }
                    }
                    CopyInResponse(copy_options) => {
                        self.state.write()?.copy = Some(copy_options);
                        get_or_insert_default(&mut result).status = Some(crate::Status::CopyIn);
                        async_status = AsyncStatus::COPY_IN;
                        break;
                    }
                    CopyOut(_) => {
                        get_or_insert_default(&mut result).status = Some(crate::Status::CopyOut);
                        async_status = AsyncStatus::COPY_OUT;
                        break;
                    }
                    CopyData(_) => (),
                    DataRow(data_row) => {
                        get_or_insert_default(&mut result).add_row(data_row);

                        if single_row_mode {
                            break;
                        }
                    }
                    EmptyQuery => (),
                    ErrorResponse(error) => {
                        get_or_insert_default(&mut result).error_message = Some(error);
                        async_status = AsyncStatus::IDLE;
                    }
                    NoticeResponse(notice) => get_or_insert_default(&mut result).notices.push(notice),
                    NotificationResponse(notify) => self.state.write()?.notifies.push(notify),
                    ParseComplete => async_status.remove(AsyncStatus::PREPARE),
                    ParameterDescription(params) => {
                        get_or_insert_default(&mut result).params = Some(params);
                        async_status.remove(AsyncStatus::DESCRIBE_STATEMENT);
                    }
                    ParameterStatus(k, v) => {
                        self.state.write()?.parameters.insert(k, v);
                    }
                    ReadyForQuery(_) => async_status.remove(AsyncStatus::CONNECT),
                    RowDescription(description) => {
                        get_or_insert_default(&mut result).description = Some(description);
                        async_status.remove(AsyncStatus::DESCRIBE_ROW);
                    }
                    _ => unreachable!(),
                }
            }
        }

        self.state.write()?.async_status = async_status;

        if async_status == AsyncStatus::IDLE {
            self.state.write()?.result = None;
        } else {
            self.state.write()?.result = result.clone();
        }

        Ok(result)
    }

    /**
     * If input is available from the server, consume it.
     *
     * See
     * [PQconsumeInput](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQCONSUMEINPUT).
     */
    pub fn consume_input(&self) -> std::result::Result<(), crate::Error> {
        log::debug!("Consume input");

        while self.parse_input()?.is_some() {
        }

        Ok(())
    }

    /**
     * Returns `true` if a command is busy, that is, `Result` would block waiting for input.
     *
     * See [PQisBusy](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISBUSY).
     */
    pub fn is_busy(&self) -> bool {
        todo!()
    }

    /**
     * Sets the nonblocking status of the connection.
     *
     * See
     * [PQsetnonblocking](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSETNONBLOCKING).
     */
    pub fn set_non_blocking(&self, non_blocking: bool) -> std::result::Result<(), crate::Error> {
        if non_blocking {
            log::debug!("Set non blocking");
        } else {
            log::debug!("Set blocking");
        }

        self.state.write()?.non_blocking = non_blocking;

        Ok(())
    }

    /**
     * Returns the blocking status of the database connection.
     *
     * See
     * [PQisnonblocking](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISNONBLOCKING).
     */
    pub fn is_non_blocking(&self) -> bool {
        self.state.read().unwrap().non_blocking
    }

    /**
     * Attempts to flush any queued output data to the server.
     *
     * See [PQflush](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQFLUSH).
     */
    pub fn flush(&self) -> std::result::Result<(), crate::Error> {
        log::trace!("Flush");

        todo!()
    }
}

// #![feature(option_get_or_insert_default)]
fn get_or_insert_default(result: &mut Option<crate::Result>) -> &mut crate::Result {
    if let None = *result {
        *result = Some(crate::Result::default());
    }

    match result {
        Some(result) => result,
        None => unreachable!(),
    }
}
