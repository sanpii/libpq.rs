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
    Needed,
}

impl From<pq_sys::_bindgen_ty_2> for Status {
    fn from(status: pq_sys::_bindgen_ty_2) -> Self {
        match status {
            pq_sys::_bindgen_ty_2::CONNECTION_OK => Self::Ok,
            pq_sys::_bindgen_ty_2::CONNECTION_BAD => Self::Bad,
            pq_sys::_bindgen_ty_2::CONNECTION_STARTED => Self::Started,
            pq_sys::_bindgen_ty_2::CONNECTION_MADE => Self::Made,
            pq_sys::_bindgen_ty_2::CONNECTION_AWAITING_RESPONSE => Self::AwaitingResponse,
            pq_sys::_bindgen_ty_2::CONNECTION_AUTH_OK => Self::AuthOk,
            pq_sys::_bindgen_ty_2::CONNECTION_SETENV => Self::Setenv,
            pq_sys::_bindgen_ty_2::CONNECTION_SSL_STARTUP => Self::SslStartup,
            pq_sys::_bindgen_ty_2::CONNECTION_NEEDED => Self::Needed,
        }
    }
}
