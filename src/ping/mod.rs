#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    /** The server is running and appears to be accepting connections. */
    Ok,
    /**
     * The server is running but is in a state that disallows connections (startup, shutdown, or
     * crash recovery).
     */
    Reject,
    /**
     * The server could not be contacted. This might indicate that the server is not running, or
     * that there is something wrong with the given connection parameters (for example, wrong port
     * number), or that there is a network connectivity problem (for example, a firewall blocking
     * the connection request).
     */
    NoResponse,
    /**
     * No attempt was made to contact the server, because the supplied parameters were obviously
     * incorrect or there was some client-side problem (for example, out of memory).
     */
    NoAttempt,
}
