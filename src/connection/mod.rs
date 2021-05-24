mod cancel;
mod info;
mod config;
mod notify;
mod socket;
mod state;
mod status;

pub use cancel::*;
pub use config::*;
pub use info::*;
pub use notify::*;
pub use status::*;

pub(crate) use socket::Socket;
pub(crate) use state::*;

#[derive(Debug)]
pub struct Connection {
    config: Config,
    socket: Socket,
    state: std::sync::RwLock<State>,
}

include!("_async.rs");
include!("_cancel.rs");
include!("_connect.rs");
include!("_control.rs");
include!("_copy.rs");
include!("_exec.rs");
include!("_gss.rs");
include!("_notify.rs");
include!("_single_row_mode.rs");
include!("_ssl.rs");
include!("_status.rs");
include!("_threading.rs");

impl Connection {
    fn send_query_start(&self) -> Result<(), crate::Error> {
        let async_status = self.state.read()?.async_status;

        if async_status != AsyncStatus::IDLE
            && async_status != AsyncStatus::COPY_IN
            && async_status != AsyncStatus::COPY_OUT
        {
            return Err(crate::Error::InvalidState("another command is already in progress".to_string()));
        }

        self.consume_input()?;
        self.state.write()?.single_row_mode = false;

        Ok(())
    }

    fn send(&self, message: crate::Message, new_status: AsyncStatus) -> Result<(), crate::Error> {
        self.socket.send(message)?;

        self.state.write()?.async_status.insert(new_status);

        Ok(())
    }

    fn sync(&self) -> Result<(), crate::Error> {
        self.socket.send(crate::Message::Sync)
    }

    pub(crate) fn std_strings(&self) -> std::result::Result<bool, crate::Error> {
        let std_strings = self.state.read()?.parameters
            .get(&"standard_conforming_strings".to_string()) == Some(&"on".to_string());

        Ok(std_strings)
    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        let mut connection = Self::start_with_config(&self.config).unwrap();
        connection.state = std::sync::RwLock::new(self.state.read().unwrap().clone());

        connection
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
            &[Some(b"1\0".to_vec())],
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
            &[Some(b"foo\0".to_vec())],
            &[],
            crate::Format::Text,
        );
        assert_eq!(results.status(), crate::Status::FatalError);
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
            &[Some(b"fooo\0".to_vec())],
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
            &[Some(b"fooo\0".to_vec())],
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

        conn.send_query_prepared(None, &[Some(b"fooo\0".to_vec())], &[], crate::Format::Text)
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
            result.err().unwrap().to_string(),
            "another command is already in progress".to_string()
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
            vec![
                crate::ssl::Attribute::Library,
                crate::ssl::Attribute::KeyBits,
                crate::ssl::Attribute::Cipher,
                crate::ssl::Attribute::Compression,
                crate::ssl::Attribute::Protocol,
            ]
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
        assert_eq!(notify.relname(), "test".to_string());
        assert_eq!(notify.extra(), "foo".to_string());
    }

    #[test]
    fn copy() {
        let conn = crate::test::new_conn();
        conn.exec("create temporary table tmp (id integer)");
        let result = conn.exec("copy tmp (id) from stdin;");
        assert_eq!(result.status(), crate::Status::CopyIn);
        conn.put_copy_data("1\n2\n3\n4\n5\n").unwrap();
        conn.put_copy_end(None).unwrap();
        let result = conn.exec("select * from tmp");
        assert_eq!(result.ntuples(), 5);

        conn.exec("copy tmp (id) from stdin;");
        conn.put_copy_end(Some("foo")).unwrap();
        assert!(conn.put_copy_data("1\n2\n3\n4\n5\n").is_err());

        let result = conn.exec("copy tmp to stdout");
        assert_eq!(result.status(), crate::Status::CopyOut);
        assert_eq!(conn.copy_data(false).unwrap(), "1\n".to_string());
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
    #[ignore = "Should be launch alone"]
    fn trace() {
        let dsn = std::env::var("PQ_DSN").unwrap_or_else(|_| "host=localhost".to_string());
        let conn = crate::Connection::new(&dsn).unwrap();

        flexi_logger::Logger::with_str("trace")
            .log_to_file()
            .print_message()
            .create_symlink("trace.txt")
            .format_for_files(|w, _, record| {
                write!(w, "{}", record.args())
            })
            .start()
            .unwrap();

        conn.exec("SELECT 1");

        use std::io::Read;
        let mut file = std::fs::File::open("trace.txt").unwrap();
        let mut trace = String::new();
        file.read_to_string(&mut trace).unwrap();
        assert_eq!(
            trace.trim_start_matches('\0'),
            r#"To backend> Msg Q
To backend> "SELECT 1"
To backend> Msg complete, length 14
From backend> T
From backend (#4)> 33
From backend (#2)> 1
From backend> "?column?"
From backend (#4)> 0
From backend (#2)> 0
From backend (#4)> 23
From backend (#2)> 4
From backend (#4)> -1
From backend (#2)> 0
From backend> D
From backend (#4)> 11
From backend (#2)> 1
From backend (#4)> 1
From backend (1)> 1
From backend> C
From backend (#4)> 13
From backend> "SELECT 1"
From backend> Z
From backend (#4)> 5
From backend> I
"#
        );
    }
}
