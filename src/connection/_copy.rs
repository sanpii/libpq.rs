/**
 * [Functions Associated with the `COPY`
 * Command](https://www.postgresql.org/docs/current/libpq-copy.html)
 */
impl Connection {
    /**
     * Sends data to the server during `libpq::Status::CopyIn` state.
     *
     * See
     * [PQputCopyData](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQPUTCOPYDATA).
     */
    pub fn put_copy_data(&self, buffer: &str) -> std::result::Result<(), crate::Error> {
        log::debug!("Sending copy data");

        let state = &mut self.state.write()?;

        if state.async_status.contains(AsyncStatus::COPY_IN) {
            self.socket.send(crate::Message::CopyData(buffer.to_string()))
        } else {
            Err(crate::Error::InvalidState("no COPY in progress".to_string()))
        }
    }

    /**
     * Sends end-of-data indication to the server during `libpq::Status::CopyIn` state.
     *
     * See
     * [PQputCopyEnd](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQPUTCOPYEND).
     */
    pub fn put_copy_end(&self, errormsg: Option<&str>) -> std::result::Result<(), crate::Error> {
        log::debug!("End of copy");

        if !self.state.read()?.async_status.contains(AsyncStatus::COPY_IN) {
            return Err(crate::Error::InvalidState("no COPY in progress".to_string()))
        }

        if let Some(errormsg) = errormsg {
            self.socket.send(crate::Message::CopyFail(errormsg.to_string()))?;
        } else {
            self.socket.send(crate::Message::CopyDone)?;
        }

        self.parse_input()?;

        Ok(())
    }

    /**
     * Receives data from the server during `libpq::Status::CopyOut` state.
     *
     * See
     * [PQgetCopyData](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQGETCOPYDATA).
     */
    pub fn copy_data(&self, r#async: bool) -> std::result::Result<String, crate::Error> {
        let mut state = self.state.write()?;

        if !state.async_status.contains(AsyncStatus::COPY_OUT) {
            return Err(crate::Error::InvalidState("no COPY in progress".to_string()));
        }

        while state.async_status.contains(AsyncStatus::COPY_OUT) {
            if let Some(message) = self.socket.receive()? {
                match message {
                    crate::Message::CopyOut(_) => state.async_status.remove(AsyncStatus::COPY_OUT),
                    crate::Message::CopyData(data) => return Ok(data),
                    _ => unreachable!(),
                }
            }
        }

        unreachable!();
    }
}
