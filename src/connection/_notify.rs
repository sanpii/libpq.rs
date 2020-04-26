/**
 * [Asynchronous Notification](https://www.postgresql.org/docs/current/libpq-notify.html)
 */
impl Connection {
    /**
     * Returns the next notification from a list of unhandled notification messages received from
     * the server.
     */
    pub fn notifies(&self) -> Option<crate::connection::Notify> {
        let raw = unsafe { pq_sys::PQnotifies(self.into()) };

        if raw.is_null() {
            None
        } else {
            Some(raw.into())
        }
    }
}
