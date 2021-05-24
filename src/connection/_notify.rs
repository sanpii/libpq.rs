/**
 * [Asynchronous Notification](https://www.postgresql.org/docs/current/libpq-notify.html)
 */
impl Connection {
    /**
     * Returns the next notification from a list of unhandled notification messages received from
     * the server.
     */
    pub fn notifies(&self) -> Option<crate::connection::Notify> {
        self.parse_input().ok();

        match self.state.write() {
            Ok(mut state) => state.notifies.pop(),
            Err(_) => None,
        }
    }
}
