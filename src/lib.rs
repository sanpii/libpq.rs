#![warn(warnings)]

pub mod connection;
pub mod escape;
pub mod ping;
pub mod poll;
pub mod print;
pub mod result;
pub mod ssl;
pub mod state;
pub mod transaction;
pub mod types;

mod encoding;
mod error;
mod format;
mod message;
mod oid;
mod payload;
mod status;
mod verbosity;

pub use connection::Connection;
pub use encoding::*;
pub use error::*;
pub use format::*;
pub use oid::*;
pub use result::Result;
pub use state::State;
pub use status::*;
pub use types::Type;
pub use verbosity::*;

use message::Message;
use payload::Payload;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

/**
 * Get the version of the libpq library in use.
 *
 * See [PQlibVersion](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQLIBVERSION).
 */
pub fn version() -> i32 {
    PG_VERSION_NUM
}

#[cfg(test)]
mod test {
    static INIT: std::sync::Once = std::sync::Once::new();

    pub fn dsn() -> String {
        std::env::var("PQ_DSN").unwrap_or_else(|_| "host=localhost".to_string())
    }

    pub fn new_conn() -> crate::Connection {
        INIT.call_once(|| {
            dotenv::dotenv().ok();
            flexi_logger::Logger::try_with_env().unwrap().start().ok();
        });

        crate::Connection::new(&dsn()).unwrap()
    }

    #[test]
    fn version() {
        assert!(crate::version() > 0);
    }
}
