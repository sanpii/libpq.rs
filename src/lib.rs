#![warn(rust_2018_idioms)]

#[macro_use]
mod ffi;

pub mod connection;
pub mod encrypt;
pub mod escape;
pub mod ping;
pub mod poll;
pub mod print;
pub mod result;
pub mod ssl;
pub mod state;
pub mod transaction;
pub mod ty;

mod encoding;
mod format;
mod oid;
mod status;
mod verbosity;

pub use connection::Connection;
pub use encoding::*;
pub use format::*;
pub use oid::*;
pub use result::Result;
pub use state::State;
pub use status::*;
pub use ty::Type;
pub use verbosity::*;

/**
 * Get the version of the libpq library in use.
 *
 * See [PQlibVersion](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQLIBVERSION).
 */
pub fn version() -> i32 {
    unsafe { pq_sys::PQlibVersion() }
}

#[cfg(test)]
mod test {
    static INIT: std::sync::Once = std::sync::Once::new();

    pub fn dsn() -> String {
        std::env::var("PQ_DSN").unwrap_or_else(|_| "host=localhost".to_string())
    }

    pub fn new_conn() -> crate::Connection {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        crate::Connection::new(&dsn()).unwrap()
    }

    #[test]
    fn version() {
        assert!(crate::version() > 0);
    }
}
