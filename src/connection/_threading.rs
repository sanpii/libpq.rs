/**
 * [Behavior in Threaded Programs](https://www.postgresql.org/docs/current/libpq-threading.html)
 */
impl Connection {
    /**
     * Returns the thread safety status of the libpq library.
     *
     * See
     * [PQisthreadsafe](https://www.postgresql.org/docs/current/libpq-threading.html#LIBPQ-PQISTHREADSAFE).
     */
    pub fn is_thread_safe() -> bool {
        true
    }
}
