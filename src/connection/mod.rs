mod buffer;
mod cancel;
mod info;
mod notify;
mod status;

pub use buffer::*;
pub use cancel::*;
pub use info::*;
pub use notify::*;
pub use status::*;

pub type NoticeProcessor = pq_sys::PQnoticeProcessor;
pub type NoticeReceiver = pq_sys::PQnoticeReceiver;

use std::os::raw;

#[derive(Clone)]
pub struct Connection {
    conn: *mut pq_sys::PGconn,
}

unsafe impl Send for Connection {}

include!("_async.rs");
include!("_cancel.rs");
include!("_connect.rs");
include!("_control.rs");
include!("_copy.rs");
include!("_exec.rs");
#[cfg(feature = "v12")]
include!("_gss.rs");
include!("_notice_processing.rs");
include!("_notify.rs");
include!("_single_row_mode.rs");
include!("_ssl.rs");
include!("_status.rs");
include!("_threading.rs");
include!("_trace.rs");

impl Connection {
    /**
     * Prepares the encrypted form of a PostgreSQL password.
     *
     * On success, this method returns [`PqString`].
     *
     * See [PQencryptPasswordConn](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQENCRYPTPASSWORDCONN).
     */
    pub fn encrypt_password(
        &self,
        passwd: &str,
        user: &str,
        algorithm: Option<&str>,
    ) -> crate::errors::Result<PqString> {
        let c_passwd = crate::ffi::to_cstr(passwd);
        let c_user = crate::ffi::to_cstr(user);

        unsafe {
            let ptr = if let Some(algorithm) = algorithm {
                let c_algorithm = crate::ffi::to_cstr(algorithm);
                pq_sys::PQencryptPasswordConn(
                    self.into(),
                    c_passwd.as_ptr(),
                    c_user.as_ptr(),
                    c_algorithm.as_ptr(),
                )
            } else {
                pq_sys::PQencryptPasswordConn(
                    self.into(),
                    c_passwd.as_ptr(),
                    c_user.as_ptr(),
                    std::ptr::null(),
                )
            };

            if ptr.is_null() {
                self.error()
            } else {
                Ok(PqString::from_raw(ptr))
            }
        }
    }

    fn transform_params(
        param_values: &[Option<&[u8]>],
        param_formats: &[crate::Format],
    ) -> (Vec<*const raw::c_char>, Vec<i32>, Vec<i32>) {
        if param_values.is_empty() {
            return Default::default();
        }

        let mut values = Vec::new();
        let mut formats = Vec::new();
        let mut lengths = Vec::new();

        for (x, value) in param_values.iter().enumerate() {
            let format = param_formats.get(x).unwrap_or(&crate::Format::Text);
            formats.push(format.into());

            if let Some(v) = value {
                if format == &crate::Format::Text && v.last() != Some(&b'\0') {
                    panic!("Param value as text should be null terminated");
                }
                values.push(v.as_ptr() as *const raw::c_char);
                lengths.push(v.len() as i32);
            } else {
                values.push(std::ptr::null());
                lengths.push(0);
            }
        }

        (values, formats, lengths)
    }

    fn trace_query(
        prefix: &str,
        command: &str,
        param_types: &[crate::Oid],
        param_values: &[Option<&[u8]>],
        param_formats: &[crate::Format],
    ) {
        use std::fmt::Write;

        if log::log_enabled!(log::Level::Trace) {
            let mut msg = prefix.to_string();

            let mut p = Vec::new();

            for (x, value) in param_values.iter().enumerate() {
                let v = if let Some(s) = value {
                    match param_formats.get(x) {
                        Some(crate::Format::Binary) => format!("{s:?}"),
                        _ => String::from_utf8(s.to_vec()).unwrap_or_else(|_| "�".to_string()),
                    }
                } else {
                    "null".to_string()
                };
                let default_type = crate::types::UNKNOWN;
                let t = crate::Type::try_from(*param_types.get(x).unwrap_or(&default_type.oid))
                    .unwrap_or(default_type);

                p.push(format!("'{v}'::{}", t.name));
            }

            if !command.is_empty() {
                write!(msg, " query '{command}'").ok();
            }

            if !p.is_empty() {
                write!(msg, " with params [{}]", p.join(", ")).ok();
            }

            log::trace!("{}", msg);
        }
    }

    pub(crate) fn error<T>(&self) -> crate::errors::Result<T> {
        Err(self
            .error_message()
            .map(|x| crate::errors::Error::Backend(x.to_string()))
            .unwrap_or(crate::errors::Error::Unknow))
    }
}

#[doc(hidden)]
impl From<&Connection> for *mut pq_sys::pg_conn {
    fn from(connection: &Connection) -> *mut pq_sys::pg_conn {
        connection.conn
    }
}

#[doc(hidden)]
impl From<&mut Connection> for *mut pq_sys::pg_conn {
    fn from(connection: &mut Connection) -> *mut pq_sys::pg_conn {
        connection.conn
    }
}

#[doc(hidden)]
impl From<&Connection> for *const pq_sys::pg_conn {
    fn from(connection: &Connection) -> *const pq_sys::pg_conn {
        connection.conn
    }
}

#[doc(hidden)]
impl TryFrom<*mut pq_sys::pg_conn> for Connection {
    type Error = crate::errors::Error;

    fn try_from(conn: *mut pq_sys::pg_conn) -> std::result::Result<Self, Self::Error> {
        let s = Self { conn };

        if s.status() == crate::connection::Status::Bad {
            s.error()
        } else {
            Ok(s)
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            pq_sys::PQfinish(self.into());
        }
    }
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("inner", &self.conn)
            .field("db", &self.db())
            .field("user", &self.user())
            .field("pass", &self.pass())
            .field("host", &self.host())
            .field("port", &self.port())
            .field("options", &self.options())
            .field("status", &self.status())
            .field("transaction_status", &self.transaction_status())
            .field("protocol_version", &self.protocol_version())
            .field("server_version", &self.server_version())
            .field("error_message", &self.error_message())
            .field("socket", &self.socket())
            .field("backend_pid", &self.backend_pid())
            .field("info", &self.info())
            .field("needs_password", &self.needs_password())
            .field("used_password", &self.used_password())
            .field("ssl_in_use", &self.ssl_in_use())
            .finish()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn thread() {
        let conn = crate::test::new_conn();
        assert!(crate::Connection::is_thread_safe());

        let thread = std::thread::spawn(move || {
            assert_eq!(conn.exec("SELECT 1").status(), crate::Status::TuplesOk)
        });

        thread.join().ok();
    }

    #[test]
    fn reset() {
        let conn = crate::test::new_conn();
        conn.reset();
    }

    #[test]
    fn poll() {
        let dsn = std::env::var("PQ_DSN").unwrap_or_else(|_| "host=localhost".to_string());
        let conn = crate::Connection::start(&dsn).unwrap();
        assert_eq!(conn.poll(), crate::poll::Status::Writing);
        conn.reset_start();
        assert_eq!(conn.reset_poll(), crate::poll::Status::Writing);
    }

    #[test]
    fn exec() {
        let conn = crate::test::new_conn();
        let results = conn.exec("SELECT 1 as one, 2 as two from generate_series(1,2)");
        assert_eq!(results.status(), crate::Status::TuplesOk);
        assert_eq!(results.ntuples(), 2);
        assert_eq!(results.nfields(), 2);

        assert_eq!(results.value(0, 0), Some(&b"1"[..]));
        assert_eq!(results.value(0, 1), Some(&b"2"[..]));
    }

    #[test]
    fn exec_null() {
        let conn = crate::test::new_conn();
        let results = conn.exec("SELECT null");

        assert_eq!(results.value(0, 0), None);
    }

    #[test]
    fn exec_params() {
        let conn = crate::test::new_conn();
        let results = conn.exec_params(
            "SELECT $1",
            &[crate::types::INT4.oid],
            &[Some(b"1\0")],
            &[],
            crate::Format::Text,
        );
        assert_eq!(results.status(), crate::Status::TuplesOk);

        assert_eq!(results.value(0, 0), Some(&b"1"[..]));
    }

    #[test]
    fn exec_invalid_type() {
        let conn = crate::test::new_conn();
        let results = conn.exec_params(
            "SELECT $1",
            &[crate::types::INT4.oid],
            &[Some(b"foo\0")],
            &[],
            crate::Format::Text,
        );
        assert_eq!(results.status(), crate::Status::FatalError);
    }

    #[test]
    #[should_panic]
    fn exec_text() {
        let conn = crate::test::new_conn();
        let _ = conn.exec_params(
            "SELECT $1",
            &[],
            &[Some(b"foo")],
            &[],
            crate::Format::Text,
        );
    }

    #[test]
    fn exec_prepared() {
        let conn = crate::test::new_conn();
        let results = conn.prepare(Some("test1"), "SELECT $1", &[crate::types::TEXT.oid]);
        assert_eq!(results.status(), crate::Status::CommandOk);

        let results = conn.describe_prepared(Some("test1"));
        assert_eq!(results.nfields(), 1);

        let results = conn.exec_prepared(
            Some("test1"),
            &[Some(b"fooo\0")],
            &[],
            crate::Format::Text,
        );
        assert_eq!(results.value(0, 0), Some(&b"fooo"[..]));
    }

    #[test]
    fn send_query() {
        let conn = crate::test::new_conn();
        conn.send_query("SELECT 1 as one, 2 as two from generate_series(1,2)")
            .unwrap();

        loop {
            if let Some(result) = conn.result() {
                assert_eq!(result.value(0, 0), Some(&b"1"[..]));
            } else {
                break;
            }
        }
    }

    #[test]
    fn send_query_params() {
        let conn = crate::test::new_conn();
        assert!(conn.set_single_row_mode().is_err());

        conn.send_query_params(
            "SELECT $1",
            &[crate::types::TEXT.oid],
            &[Some(b"fooo\0")],
            &[],
            crate::Format::Text,
        )
        .unwrap();

        assert!(conn.set_single_row_mode().is_ok());

        let result = conn.result().unwrap();
        assert_eq!(result.value(0, 0), Some(&b"fooo"[..]));

        #[cfg(unix)]
        {
            let options = crate::print::Options {
                header: true,
                align: true,
                standard: false,
                html3: false,
                expanded: false,
                pager: false,
                field_sep: "|".to_string(),
                table_opt: String::new(),
                caption: String::new(),
                field_name: Vec::new(),
            };

            result.print(&std::io::stdout(), &options);
        }
    }

    #[test]
    fn send_prepare() {
        let conn = crate::test::new_conn();
        conn.send_prepare(None, "SELECT $1", &[crate::types::TEXT.oid])
            .unwrap();
        while conn.result().is_some() {}

        conn.send_query_prepared(None, &[Some(b"fooo\0")], &[], crate::Format::Text)
            .unwrap();
        assert_eq!(conn.result().unwrap().value(0, 0), Some(&b"fooo"[..]));
        assert!(conn.result().is_none());

        conn.send_describe_prepared(None).unwrap();
        assert_eq!(conn.result().unwrap().nfields(), 1);
    }

    #[test]
    fn send_error() {
        let conn = crate::test::new_conn();
        conn.send_prepare(None, "SELECT $1", &[crate::types::TEXT.oid])
            .unwrap();
        let result = conn.send_prepare(None, "SELECT $1", &[crate::types::TEXT.oid]);
        assert_eq!(
            result,
            Err(crate::errors::Error::Backend(
                "another command is already in progress\n".to_string()
            ))
        );
    }

    #[test]
    fn client_encoding() {
        let conn = crate::test::new_conn();
        assert_eq!(conn.client_encoding(), crate::Encoding::UTF8);
    }

    #[test]
    fn set_client_encoding() {
        let conn = crate::test::new_conn();
        conn.set_client_encoding(crate::Encoding::SQL_ASCII);
        assert_eq!(conn.client_encoding(), crate::Encoding::SQL_ASCII);
    }

    #[test]
    fn info() {
        let conn = crate::test::new_conn();
        let _ = conn.info();
    }

    #[test]
    fn ping() {
        assert_eq!(
            crate::Connection::ping(&crate::test::dsn()),
            crate::ping::Status::Ok
        );
    }

    #[test]
    fn ssl_attribute_names() {
        let conn = crate::test::new_conn();

        assert_eq!(
            conn.ssl_attribute_names(),
            Ok(vec![
                crate::ssl::Attribute::Library,
                crate::ssl::Attribute::KeyBits,
                crate::ssl::Attribute::Cipher,
                crate::ssl::Attribute::Compression,
                crate::ssl::Attribute::Protocol,
            ])
        );
    }

    #[test]
    fn blocking() {
        let conn = crate::test::new_conn();
        assert_eq!(conn.is_non_blocking(), false);
        conn.set_non_blocking(true).unwrap();
        assert_eq!(conn.is_non_blocking(), true);
    }

    #[test]
    fn cancel() {
        let conn = crate::test::new_conn();
        conn.exec("SELECT 1");

        let cancel = conn.cancel();
        assert!(cancel.request().is_ok());
    }

    #[test]
    fn notifies() {
        let conn = crate::test::new_conn();
        assert!(conn.notifies().is_none());

        conn.exec("LISTEN test");
        conn.exec("NOTIFY test, 'foo'");

        let notify = conn.notifies().unwrap();
        assert_eq!(notify.relname(), Ok("test".to_string()));
        assert_eq!(notify.extra(), Ok("foo".to_string()));
    }

    #[test]
    fn copy() {
        let conn = crate::test::new_conn();
        conn.exec("create temporary table tmp (id integer)");
        let result = conn.exec("copy tmp (id) from stdin;");
        assert_eq!(result.status(), crate::Status::CopyIn);
        conn.put_copy_data(b"1\n2\n3\n4\n5\n").unwrap();
        conn.put_copy_end(None).unwrap();
        let result = conn.exec("select * from tmp");
        assert_eq!(result.ntuples(), 5);

        conn.exec("copy tmp (id) from stdin;");
        conn.put_copy_end(Some("foo")).unwrap();
        assert!(conn.put_copy_data(b"1\n2\n3\n4\n5\n").is_err());

        let result = conn.exec("copy tmp to stdout");
        assert_eq!(result.status(), crate::Status::CopyOut);
        assert_eq!(&*conn.copy_data(false).unwrap(), b"1\n");
    }

    #[test]
    fn copy_binary() {
        /* Construct binary data with only one tuple */
        /* see Binary Format in https://www.postgresql.org/docs/current/sql-copy.html */
        let binary_data = {
            let header = {
                let header_signature = b"PGCOPY\n\xFF\r\n\0";
                let header_flags = b"\0\0\0\0";
                let header_extension_length = b"\0\0\0\0";

                header_signature
                    .iter()
                    .chain(header_flags.iter())
                    .chain(header_extension_length.iter())
                    .cloned()
                    .collect::<Vec<u8>>()
            };
            let tuples = {
                let tuple_0_field_count = b"\x00\x01";
                let tuple_0_field_0_length = b"\x00\x00\x00\x07";
                let tuple_0_field_0_data = b"\xFF\x00\xFF\x00\xFF\x00\xFF";

                tuple_0_field_count
                    .iter()
                    .chain(tuple_0_field_0_length.iter())
                    .chain(tuple_0_field_0_data.iter())
                    .cloned()
                    .collect::<Vec<u8>>()
            };
            header
                .iter()
                .chain(tuples.iter())
                .cloned()
                .collect::<Vec<u8>>()
        };

        let conn = crate::test::new_conn();
        conn.exec("create temporary table tmp (code bytea);");

        let result = conn.exec("copy tmp (code) from stdin binary;");
        assert_eq!(result.status(), crate::Status::CopyIn);
        conn.put_copy_data(&binary_data).unwrap();
        conn.put_copy_end(None).unwrap();
        let result = conn.exec("select * from tmp");
        assert_eq!(result.ntuples(), 1);

        let result = conn.exec("copy tmp to stdout binary;");
        assert_eq!(result.status(), crate::Status::CopyOut);
        assert_eq!(&*conn.copy_data(false).unwrap(), binary_data);
    }

    #[test]
    fn verbosity() {
        let conn = crate::test::new_conn();

        assert_eq!(
            conn.set_error_verbosity(crate::Verbosity::Verbose),
            crate::Verbosity::Default
        );
        assert_eq!(
            conn.set_error_verbosity(crate::Verbosity::Terse),
            crate::Verbosity::Verbose
        );
    }

    #[test]
    #[cfg(unix)]
    fn trace() {
        let conn = crate::test::new_conn();
        let file = std::fs::File::create("trace.txt").unwrap();

        conn.trace(file);
        #[cfg(feature = "v14")]
        conn.trace_set_flags(crate::trace::Flags::SUPPRESS_TIMESTAMPS);

        conn.exec("SELECT 1");
        conn.untrace();
        conn.exec("SELECT 1");

        use std::io::Read;
        let mut file = std::fs::File::open("trace.txt").unwrap();
        let mut trace = String::new();
        file.read_to_string(&mut trace).unwrap();

        /*
         * https://github.com/postgres/postgres/commit/198b3716dba68544b55cb97bd120738a86d5df2d
         */
        #[cfg(feature = "v14")]
        assert_eq!(
            trace,
            r#"F	13	Query	 "SELECT 1"
B	33	RowDescription	 1 "?column?" 0 0 23 4 -1 0
B	11	DataRow	 1 1 '1'
B	13	CommandComplete	 "SELECT 1"
B	5	ReadyForQuery	 I
"#
        );
    }

    #[test]
    fn encrypt_password() {
        let conn = crate::test::new_conn();

        assert_eq!(
            conn.encrypt_password("1234", "postgres", Some("md5"))
                .unwrap()
                .to_string_lossy(),
            "md524bb002702969490e41e26e1a454036c"
        );
    }

    #[test]
    fn invalid_encrypt_password() {
        let conn = crate::test::new_conn();

        assert_eq!(
            conn.encrypt_password("1234", "postgres", Some("test"))
                .unwrap_err(),
            crate::errors::Error::Backend(
                "unrecognized password encryption algorithm \"test\"\n".to_string()
            ),
        );
    }

    #[test]
    #[cfg(feature = "v16")]
    fn used_gssapi() {
        let conn = crate::test::new_conn();

        assert_eq!(conn.used_gssapi(), false);
    }
}
