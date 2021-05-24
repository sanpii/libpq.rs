#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    /** The server's response was not understood. */
    BadResponse,
    /** Successful completion of a command returning no data. */
    CommandOk,
    /**
     * Copy In/Out (to and from server) data transfer started. This feature is currently used only
     * for streaming replication, so this status should not occur in ordinary applications.
     */
    CopyBoth,
    /** Copy In (to server) data transfer started. */
    CopyIn,
    /** Copy Out (from server) data transfer started. */
    CopyOut,
    /** The string sent to the server was empty. */
    EmptyQuery,
    /** A fatal error occurred. */
    FatalError,
    /** A nonfatal error (a notice or warning) occurred. */
    NonFatalError,
    /**
     * The `libpq::Result` contains a single result tuple from the current command. This status
     * occurs only when single-row mode has been selected for the query
     */
    SingleTuble,
    /** Successful completion of a command returning data (such as a `SELECT` or `SHOW`). */
    TuplesOk,
}
