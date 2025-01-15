/**
 * [Retrieving Query Results Row-by-Row](https://www.postgresql.org/docs/current/libpq-single-row-mode.html)
 */
impl Connection {
    /**
     * Select single-row mode for the currently-executing query.
     *
     * See
     * [PQsetSingleRowMode](https://www.postgresql.org/docs/current/libpq-single-row-mode.html#LIBPQ-PQSETSINGLEROWMODE).
     */
    pub fn set_single_row_mode(&self) -> crate::errors::Result {
        log::trace!("Set single row mode");

        let success = unsafe { pq_sys::PQsetSingleRowMode(self.into()) };

        if success == 1 {
            Ok(())
        } else {
            self.error()
        }
    }

    /**
     * Select chunked mode for the currently-executing query.
     *
     * See
     * [PQsetChunkedRowsMode](https://www.postgresql.org/docs/current/libpq-single-row-mode.html#LIBPQ-PQSETCHUNKEDROWSMODE).
     */
    #[cfg(feature = "v17")]
    pub fn set_chunked_rows_mode(&self, chunk_size: i32) -> crate::errors::Result {
        log::trace!("Set chunked rows mode with size of {chunk_size}");

        let success = unsafe { pq_sys::PQsetChunkedRowsMode(self.into(), chunk_size) };

        if success == 1 {
            Ok(())
        } else {
            self.error()
        }
    }
}
