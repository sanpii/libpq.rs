#[derive(Clone, Debug)]
pub struct Cancel {
    raddr: std::net::SocketAddr,
    be_pid: i32,
    be_key: i32,
}

const CANCEL_REQUEST_CODE: i32 = 1234 << 16 | 5678;

impl Cancel {
    pub(crate) fn from(connection: &crate::Connection) -> Result<Self, crate::Error> {
        let cancel = Self {
            raddr: connection.socket.peer_addr()?,
            be_pid: connection.state.read()?.be_pid,
            be_key: connection.state.read()?.be_key,
        };

        Ok(cancel)
    }

    /**
     * Requests that the server abandon processing of the current command.
     *
     * See [PQcancel](https://www.postgresql.org/docs/current/libpq-cancel.html#LIBPQ-PQCANCEL).
     */
    pub fn request(&self) -> std::result::Result<(), crate::Error> {
        log::trace!("Canceling");

        use std::io::Write;

        let message = crate::Message::cancel_request(CANCEL_REQUEST_CODE, self.be_pid, self.be_key);

        let mut socket = std::net::TcpStream::connect(self.raddr)?;
        socket.write_all(&message.to_bytes())?;

        Ok(())
    }
}
