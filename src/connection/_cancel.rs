/**
 * [Canceling Queries in Progress](https://www.postgresql.org/docs/current/libpq-cancel.html)
 */
impl Connection {
    /**
     * Creates a data structure containing the information needed to cancel a command issued
     * through a particular database connection.
     *
     * See
     * [PQgetCancel](https://www.postgresql.org/docs/current/libpq-cancel.html#LIBPQ-PQGETCANCEL).
     */
    pub fn cancel(&self) -> crate::connection::Cancel {
        unsafe { pq_sys::PQgetCancel(self.into()) }.into()
    }
}
