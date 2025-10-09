#![warn(warnings)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
mod ffi;

pub mod connection;
pub mod encrypt;
pub mod errors;
pub mod escape;
pub mod lo;
pub mod ping;
#[cfg(feature = "v14")]
pub mod pipeline;
pub mod poll;
#[cfg(unix)]
pub mod print;
pub mod result;
pub mod ssl;
pub mod state;
pub mod transaction;
pub mod types;

#[cfg(feature = "v17")]
mod cancel;
mod control_visibility;
mod encoding;
mod format;
mod oid;
mod status;
#[cfg(feature = "v14")]
mod trace;
mod verbosity;

#[cfg(feature = "v17")]
pub use cancel::Cancel;
pub use connection::Connection;
pub use control_visibility::ContextVisibility;
pub use encoding::*;
pub use format::*;
pub use lo::LargeObject;
pub use oid::*;
#[deprecated(since = "4.1.0", note = "Uses PQResult instead")]
pub use result::PQResult as Result;
pub use result::PQResult;
pub use state::State;
pub use status::*;
pub use types::Type;
pub use verbosity::*;

/**
 * Get the version of the libpq library in use.
 *
 * See [PQlibVersion](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQLIBVERSION).
 */
pub fn version() -> i32 {
    unsafe { pq_sys::PQlibVersion() }
}

/**
 * Retrieves the current time, expressed as the number of microseconds since the Unix epoch (that is, time_t times 1 million).
 *
 * See [PQgetCurrentTimeUSec](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQGETCURRENTTIMEUSEC).
 */
#[cfg(feature = "v17")]
pub fn current_time_usec() -> i64 {
    unsafe { pq_sys::PQgetCurrentTimeUSec() }
}

#[cfg(test)]
mod test {
    static INIT: std::sync::Once = std::sync::Once::new();

    pub fn dsn() -> String {
        std::env::var("PQ_DSN").unwrap_or_else(|_| "host=localhost".to_string())
    }

    pub fn new_conn() -> crate::Connection {
        INIT.call_once(|| {
            env_logger::init();
        });

        crate::Connection::new(&dsn()).unwrap()
    }

    #[test]
    fn version() {
        assert!(crate::version() > 0);
    }
}
