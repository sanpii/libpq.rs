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

    /**
     * Requests that the server abandon processing of the current command.
     *
     * See [PQrequestCancel](https://www.postgresql.org/docs/18/libpq-cancel.html#LIBPQ-PQREQUESTCANCEL).
     */
    #[deprecated(note = "Use Cancel::blocking() instead")]
    pub fn request_cancel(&self) -> crate::errors::Result {
        let sucess = unsafe { pq_sys::PQrequestCancel(self.into()) };

        if sucess == 1 {
            Ok(())
        } else {
            self.error()
        }
    }
}
