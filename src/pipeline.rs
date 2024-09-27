#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
pub enum Status {
    /** The libpq connection is *not* in pipeline mode. */
    Off,
    /** The libpq connection is in pipeline mode. */
    On,
    /**
     * The libpq connection is in pipeline mode and an error occurred while processing the current
     * pipeline. The aborted flag is cleared when PQgetResult returns a result of type
     * PGRES_PIPELINE_SYNC.
     */
    Aborted,
}

#[doc(hidden)]
impl From<pq_sys::PGpipelineStatus> for Status {
    fn from(status: pq_sys::PGpipelineStatus) -> Self {
        match status {
            pq_sys::PGpipelineStatus::PQ_PIPELINE_OFF => Self::Off,
            pq_sys::PGpipelineStatus::PQ_PIPELINE_ON => Self::On,
            pq_sys::PGpipelineStatus::PQ_PIPELINE_ABORTED => Self::Aborted,
        }
    }
}

/**
 * Causes a connection to enter pipeline mode if it is currently idle or already in pipeline mode.
 *
 * See
 * [PQenterPipelineMode](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQENTERPIPELINEMODE)
 */
#[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
pub fn enter(conn: &crate::Connection) -> crate::errors::Result {
    log::debug!("Enter pipeline mode");

    let success = unsafe { pq_sys::PQenterPipelineMode(conn.into()) };

    if success == 1 {
        Ok(())
    } else {
        Err(crate::errors::Error::Unknow)
    }
}

/**
 * Causes a connection to exit pipeline mode if it is currently in pipeline mode with an empty
 * queue and no pending results.
 *
 * See
 * [PQexitPipelineMode](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQEXITPIPELINEMODE)
 */
#[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
pub fn exit(conn: &crate::Connection) -> crate::errors::Result {
    log::debug!("Exit pipeline mode");

    let success = unsafe { pq_sys::PQexitPipelineMode(conn.into()) };

    if success == 1 {
        Ok(())
    } else {
        Err(crate::errors::Error::Unknow)
    }
}

/**
 * Returns the current pipeline mode status of the libpq connection.
 *
 * See
 * [PQpipelineStatus](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQPIPELINESTATUS)
 */
#[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
pub fn status(conn: &crate::Connection) -> Status {
    let status = unsafe { pq_sys::PQpipelineStatus(conn.into()) };

    status.into()
}

/**
 * Marks a synchronization point in a pipeline by sending a sync message and flushing the send
 * buffer.
 *
 * See
 * [PQpipelineSync](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQPIPELINESYNC)
 */
#[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
pub fn sync(conn: &crate::Connection) -> crate::errors::Result {
    let success = unsafe { pq_sys::PQpipelineSync(conn.into()) };

    if success == 1 {
        Ok(())
    } else {
        Err(crate::errors::Error::Unknow)
    }
}

/**
 * Sends a request for the server to flush its output buffer.
 *
 * See
 * [PQsendFlushRequest](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQSENDFLUSHREQUEST)
 */
#[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
pub fn flush_request(conn: &crate::Connection) -> crate::errors::Result {
    let success = unsafe { pq_sys::PQsendFlushRequest(conn.into()) };

    if success == 1 {
        Ok(())
    } else {
        Err(crate::errors::Error::Unknow)
    }
}

/**
 * Marks a synchronization point in a pipeline by sending a sync message without flushing the send buffer.
 *
 * See
 * [PQsendPipelineSync](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQSENDPIPELINESYNC).
 */
#[cfg(feature = "v17")]
#[cfg_attr(docsrs, doc(cfg(feature = "v17")))]
pub fn send_sync(conn: &crate::Connection) -> crate::errors::Result {
    let success = unsafe { pq_sys::PQsendPipelineSync(conn.into()) };

    if success == 1 {
        Ok(())
    } else {
        conn.error()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn enter() {
        let conn = crate::test::new_conn();

        assert!(crate::pipeline::enter(&conn).is_ok());
    }

    #[test]
    fn exit() {
        let conn = crate::test::new_conn();

        assert!(crate::pipeline::exit(&conn).is_ok());
    }

    #[test]
    fn status() {
        let conn = crate::test::new_conn();

        assert_eq!(crate::pipeline::status(&conn), crate::pipeline::Status::Off);

        crate::pipeline::enter(&conn).unwrap();
        assert_eq!(crate::pipeline::status(&conn), crate::pipeline::Status::On);

        crate::pipeline::exit(&conn).unwrap();
        assert_eq!(crate::pipeline::status(&conn), crate::pipeline::Status::Off);
    }

    #[test]
    fn sync() {
        let conn = crate::test::new_conn();

        assert!(crate::pipeline::sync(&conn).is_err());
        crate::pipeline::enter(&conn).unwrap();
        assert!(crate::pipeline::sync(&conn).is_ok());
    }

    #[test]
    fn flush_request() {
        let conn = crate::test::new_conn();

        assert!(crate::pipeline::flush_request(&conn).is_ok());
    }

    #[test]
    #[cfg(feature = "v17")]
    fn send_sync() {
        let conn = crate::test::new_conn();

        crate::pipeline::enter(&conn).unwrap();
        assert_eq!(crate::pipeline::send_sync(&conn), Ok(()));
    }
}
