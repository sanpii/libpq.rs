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
    pub fn set_single_row_mode(&self) -> std::result::Result<(), crate::Error> {
        log::debug!("Set single row mode");

        let mut state = self.state.write()?;

        if state.async_status.contains(AsyncStatus::PREPARE)
            || state.async_status.contains(AsyncStatus::EXECUTE)
        {
            state.single_row_mode = true;

            Ok(())
        } else {
            Err(
                crate::Error::InvalidState("Change single row mode only after query launched and before result received".to_string())
            )
        }

    }
}
