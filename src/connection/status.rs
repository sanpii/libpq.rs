#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Ok,
    Bad,
    /** Waiting for connection to be made. */
    Started,
    /** Connection OK; waiting to send. */
    Made,
    /** Waiting for a response from the server. */
    AwaitingResponse,
    /** Received authentication; waiting for backend start-up to finish. */
    AuthOk,
    /** Negotiating environment-driven parameter settings. */
    Setenv,
    /** Negotiating SSL encryption. */
    SslStartup,
    /** Internal state: connect() needed */
    Needed,
    /** Check if we could make a writable connection. */
    CheckWritable,
    /** Wait for any pending message and consume them. */
    Consume,
    /** Negotiating GSSAPI. */
    GssStartup,
    /** Check if we have a proper target connection */
    CheckTarget,
}
