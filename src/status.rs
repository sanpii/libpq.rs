#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
     * The `libpq::PQResult` contains a single result tuple from the current command. This status
     * occurs only when single-row mode has been selected for the query
     */
    SingleTuple,
    /** Successful completion of a command returning data (such as a `SELECT` or `SHOW`). */
    TuplesOk,
    #[deprecated(since = "4.1.0", note = "Uses TuplesOk variant instead")]
    TupplesOk,

    /** Pipeline synchronization point. */
    #[cfg(feature = "v14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
    PipelineSync,

    /** Command didn't run because of an abort earlier in a pipeline. */
    #[cfg(feature = "v14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
    PipelineAborted,
}

#[doc(hidden)]
impl From<pq_sys::ExecStatusType> for Status {
    fn from(status: pq_sys::ExecStatusType) -> Self {
        match status {
            pq_sys::ExecStatusType::PGRES_BAD_RESPONSE => Self::BadResponse,
            pq_sys::ExecStatusType::PGRES_COMMAND_OK => Self::CommandOk,
            pq_sys::ExecStatusType::PGRES_COPY_BOTH => Self::CopyBoth,
            pq_sys::ExecStatusType::PGRES_COPY_IN => Self::CopyIn,
            pq_sys::ExecStatusType::PGRES_COPY_OUT => Self::CopyOut,
            pq_sys::ExecStatusType::PGRES_EMPTY_QUERY => Self::EmptyQuery,
            pq_sys::ExecStatusType::PGRES_FATAL_ERROR => Self::FatalError,
            pq_sys::ExecStatusType::PGRES_NONFATAL_ERROR => Self::NonFatalError,
            pq_sys::ExecStatusType::PGRES_SINGLE_TUPLE => Self::SingleTuple,
            pq_sys::ExecStatusType::PGRES_TUPLES_OK => Self::TuplesOk,
            #[cfg(feature = "v14")]
            #[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
            pq_sys::ExecStatusType::PGRES_PIPELINE_SYNC => Self::PipelineSync,
            #[cfg(feature = "v14")]
            #[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
            pq_sys::ExecStatusType::PGRES_PIPELINE_ABORTED => Self::PipelineAborted,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

#[doc(hidden)]
impl From<Status> for pq_sys::ExecStatusType {
    fn from(status: Status) -> pq_sys::ExecStatusType {
        (&status).into()
    }
}

#[doc(hidden)]
impl From<&Status> for pq_sys::ExecStatusType {
    fn from(status: &Status) -> Self {
        match *status {
            Status::BadResponse => pq_sys::ExecStatusType::PGRES_BAD_RESPONSE,
            Status::CommandOk => pq_sys::ExecStatusType::PGRES_COMMAND_OK,
            Status::CopyBoth => pq_sys::ExecStatusType::PGRES_COPY_BOTH,
            Status::CopyIn => pq_sys::ExecStatusType::PGRES_COPY_IN,
            Status::CopyOut => pq_sys::ExecStatusType::PGRES_COPY_OUT,
            Status::EmptyQuery => pq_sys::ExecStatusType::PGRES_EMPTY_QUERY,
            Status::FatalError => pq_sys::ExecStatusType::PGRES_FATAL_ERROR,
            Status::NonFatalError => pq_sys::ExecStatusType::PGRES_NONFATAL_ERROR,
            Status::SingleTuple => pq_sys::ExecStatusType::PGRES_SINGLE_TUPLE,
            Status::TuplesOk => pq_sys::ExecStatusType::PGRES_TUPLES_OK,
            #[allow(deprecated)]
            Status::TupplesOk => pq_sys::ExecStatusType::PGRES_TUPLES_OK,
            #[cfg(feature = "v14")]
            #[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
            Status::PipelineSync => pq_sys::ExecStatusType::PGRES_PIPELINE_SYNC,
            #[cfg(feature = "v14")]
            #[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
            Status::PipelineAborted => pq_sys::ExecStatusType::PGRES_PIPELINE_ABORTED,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        let status = unsafe { pq_sys::PQresStatus(self.into()) };

        crate::connection::PqString::from_raw(status)
            .to_string_lossy()
            .to_string()
    }
}
