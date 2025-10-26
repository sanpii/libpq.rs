#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    #[cfg(feature = "v11")]
    GssStartup,
    /** Check if we have a proper target connection */
    #[cfg(feature = "v11")]
    CheckTarget,
    /** Checking if server is in standby mode. */
    #[cfg(feature = "v14")]
    CheckStandby,
    /** Waiting for connection attempt to be started.  */
    #[cfg(feature = "v14")]
    Allocated,
    /** Authentication is in progress with some external system. */
    #[cfg(feature = "v18")]
    Authenticating,
}

impl From<pq_sys::ConnStatusType> for Status {
    fn from(status: pq_sys::ConnStatusType) -> Self {
        match status {
            pq_sys::ConnStatusType::CONNECTION_OK => Self::Ok,
            pq_sys::ConnStatusType::CONNECTION_BAD => Self::Bad,
            pq_sys::ConnStatusType::CONNECTION_STARTED => Self::Started,
            pq_sys::ConnStatusType::CONNECTION_MADE => Self::Made,
            pq_sys::ConnStatusType::CONNECTION_AWAITING_RESPONSE => Self::AwaitingResponse,
            pq_sys::ConnStatusType::CONNECTION_AUTH_OK => Self::AuthOk,
            pq_sys::ConnStatusType::CONNECTION_SETENV => Self::Setenv,
            pq_sys::ConnStatusType::CONNECTION_SSL_STARTUP => Self::SslStartup,
            pq_sys::ConnStatusType::CONNECTION_NEEDED => Self::Needed,
            pq_sys::ConnStatusType::CONNECTION_CHECK_WRITABLE => Self::CheckWritable,
            pq_sys::ConnStatusType::CONNECTION_CONSUME => Self::Consume,
            #[cfg(feature = "v11")]
            pq_sys::ConnStatusType::CONNECTION_GSS_STARTUP => Self::GssStartup,
            #[cfg(feature = "v11")]
            pq_sys::ConnStatusType::CONNECTION_CHECK_TARGET => Self::CheckTarget,
            #[cfg(feature = "v14")]
            pq_sys::ConnStatusType::CONNECTION_CHECK_STANDBY => Self::CheckStandby,
            #[cfg(feature = "v17")]
            pq_sys::ConnStatusType::CONNECTION_ALLOCATED => Self::Allocated,
            #[cfg(feature = "v18")]
            pq_sys::ConnStatusType::CONNECTION_AUTHENTICATING => Self::Authenticating,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
