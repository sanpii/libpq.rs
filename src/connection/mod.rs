mod cancel;
mod info;
mod notify;
mod status;

pub use cancel::*;
pub use info::*;
pub use notify::*;
pub use status::*;

use std::convert::TryInto;

pub struct Connection {
    conn: *mut pq_sys::PGconn,
}

impl Connection {
    /**
     * Makes a new connection to the database server.
     *
     * See
     * [PQconnectdb](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDB).
     */
    pub fn new(dsn: &str) -> std::result::Result<Self, String> {
        unsafe { pq_sys::PQconnectdb(crate::cstr!(dsn)) }.try_into()
    }

    /**
     * Makes a new connection to the database server.
     *
     * See [PQconnectdbParams](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS).
     */
    pub fn with_params(
        params: &std::collections::HashMap<&str, &str>,
        expand_dbname: bool,
    ) -> std::result::Result<Self, String> {
        let mut keywords = params.keys().map(|x| crate::cstr!(x)).collect::<Vec<_>>();
        keywords.push(std::ptr::null());

        let mut values = params.values().map(|x| crate::cstr!(x)).collect::<Vec<_>>();
        values.push(std::ptr::null());

        unsafe {
            pq_sys::PQconnectdbParams(keywords.as_ptr(), values.as_ptr(), expand_dbname as i32)
        }
        .try_into()
    }

    /**
     * Make a connection to the database server in a nonblocking manner.
     *
     * See [PQconnectStart](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTSTART).
     */
    pub fn start(conninfo: &str) -> std::result::Result<Self, String> {
        unsafe { pq_sys::PQconnectStart(crate::cstr!(conninfo)) }.try_into()
    }

    /**
     * Make a connection to the database server in a nonblocking manner.
     *
     * See [PQconnectStartParams](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTSTARTPARAMS).
     */
    pub fn start_params(
        params: &std::collections::HashMap<String, String>,
        expand_dbname: bool,
    ) -> std::result::Result<Self, String> {
        let mut keywords = params.keys().map(|x| crate::cstr!(x)).collect::<Vec<_>>();
        keywords.push(std::ptr::null());

        let mut values = params.values().map(|x| crate::cstr!(x)).collect::<Vec<_>>();
        values.push(std::ptr::null());

        unsafe {
            pq_sys::PQconnectStartParams(keywords.as_ptr(), values.as_ptr(), expand_dbname as i32)
        }
        .try_into()
    }

    /**
     * Makes a new connection to the database server.
     *
     * See
     * [PQsetdbLogin](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQSETDBLOGIN).
     */
    pub fn login(
        host: Option<&str>,
        port: Option<&str>,
        options: Option<&str>,
        tty: Option<&str>,
        db_name: Option<&str>,
        login: Option<&str>,
        pwd: Option<&str>,
    ) -> std::result::Result<Self, String> {
        unsafe {
            pq_sys::PQsetdbLogin(
                crate::cstr!(host.unwrap_or_default()),
                crate::cstr!(port.unwrap_or_default()),
                crate::cstr!(options.unwrap_or_default()),
                crate::cstr!(tty.unwrap_or_default()),
                crate::cstr!(db_name.unwrap_or_default()),
                crate::cstr!(login.unwrap_or_default()),
                crate::cstr!(pwd.unwrap_or_default()),
            )
        }
        .try_into()
    }

    /**
     * Returns the database name of the connection.
     *
     * See [PQdb](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQDB).
     */
    pub fn db(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQdb(self.into()) })
    }

    /**
     * Returns the user name of the connection.
     *
     * See [PQuser](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQUSER).
     */
    pub fn user(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQuser(self.into()) })
    }

    /**
     * Returns the password of the connection.
     *
     * See [PQpass](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQPASS).
     */
    pub fn pass(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQpass(self.into()) })
    }

    /**
     * Returns the server host name of the active connection.
     *
     * This can be a host name, an IP address, or a directory path if the connection is via Unix
     * socket. (The path case can be distinguished because it will always be an absolute path,
     * beginning with /.)
     *
     * See [PQhost](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQHOST).
     */
    pub fn host(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQhost(self.into()) })
    }

    /**
     * Returns the port of the active connection.
     *
     * See [PQport](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQPORT).
     */
    pub fn port(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQport(self.into()) })
    }

    /**
     * Returns the debug TTY of the connection.
     *
     * See [PQtty](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQTTY).
     */
    #[deprecated(
        note = "the server no longer pays attention to the TTY setting, but the function remains for backward compatibility."
    )]
    pub fn tty(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQtty(self.into()) })
    }

    /**
     * Returns the command-line options passed in the connection request.
     *
     * See [PQoptions](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQOPTIONS).
     */
    pub fn options(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQoptions(self.into()) })
    }

    /**
     * Returns the status of the connection.
     *
     * See [PQstatus](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSTATUS).
     */
    pub fn status(&self) -> crate::connection::Status {
        unsafe { pq_sys::PQstatus(self.into()) }.into()
    }

    /**
     * Returns the current in-transaction status of the server.
     *
     * See [PQtransactionStatus](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQTRANSACTIONSTATUS).
     */
    pub fn transaction_status(&self) -> crate::transaction::Status {
        unsafe { pq_sys::PQtransactionStatus(self.into()) }.into()
    }

    /**
     * Looks up a current parameter setting of the server.
     *
     * See [PQparameterStatus](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQPARAMETERSTATUS).
     */
    pub fn parameter_status(&self, param: &str) -> String {
        crate::ffi::to_string(unsafe {
            pq_sys::PQparameterStatus(self.into(), crate::cstr!(param))
        })
    }

    /**
     * Interrogates the frontend/backend protocol being used.
     *
     * See [PQprotocolVersion](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQPROTOCOLVERSION).
     */
    pub fn protocol_version(&self) -> i32 {
        unsafe { pq_sys::PQprotocolVersion(self.into()) }
    }

    /**
     * Returns an integer representing the server version.
     *
     * See [PQserverVersion](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSERVERVERSION).
     */
    pub fn server_version(&self) -> i32 {
        unsafe { pq_sys::PQserverVersion(self.into()) }
    }

    /**
     * Returns the error message most recently generated by an operation on the connection.
     *
     * See [PQerrorMessage](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQERRORMESSAGE).
     */
    pub fn error_message(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQerrorMessage(self.into()) })
    }

    /**
     * Obtains the file descriptor number of the connection socket to the server.
     *
     * See [PQsocket](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSOCKET).
     */
    pub fn socket(&self) -> std::result::Result<i32, ()> {
        let socket = unsafe { pq_sys::PQsocket(self.into()) };

        if socket < 0 {
            Err(())
        } else {
            Ok(socket)
        }
    }

    /**
     * Returns the process ID (PID) of the backend process handling this connection.
     *
     * See [PQbackendPID](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQBACKENDPID).
     */
    pub fn backend_pid(&self) -> u32 {
        unsafe { pq_sys::PQbackendPID(self.into()) as u32 }
    }

    /**
     * Returns `true` if the connection authentication method required a password, but none was
     * available. Returns `false` if not.
     *
     * See [PQconnectionNeedsPassword](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQCONNECTIONNEEDSPASSWORD).
     */
    pub fn needs_password(&self) -> bool {
        unsafe { pq_sys::PQconnectionNeedsPassword(self.into()) == 1 }
    }

    /**
     * Returns `true` if the connection authentication method used a password. Returns `false` if
     * not.
     *
     * See [PQconnectionUsedPassword](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQCONNECTIONUSEDPASSWORD).
     */
    pub fn used_password(&self) -> bool {
        unsafe { pq_sys::PQconnectionUsedPassword(self.into()) == 1 }
    }

    /**
     * Returns `true` if the connection uses SSL, `false` if not.
     *
     * See [PQsslInUse](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLINUSE).
     */
    pub fn ssl_in_use(&self) -> bool {
        unsafe { pq_sys::PQsslInUse(self.into()) == 1 }
    }

    /**
     * Returns SSL-related information about the connection.
     *
     * See [PQsslAttribute](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLATTRIBUTE).
     */
    pub fn ssl_attribute(&self, attribute: crate::ssl::Attribute) -> Option<String> {
        let raw =
            unsafe { pq_sys::PQsslAttribute(self.into(), crate::cstr!(&attribute.to_string())) };

        if raw.is_null() {
            None
        } else {
            crate::ffi::to_option_string(raw)
        }
    }

    /**
     * Return an array of SSL attribute names available.
     *
     * See [PQsslAttributeNames](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLATTRIBUTENAMES).
     */
    pub fn ssl_attribute_names(&self) -> Vec<crate::ssl::Attribute> {
        let raw = unsafe { pq_sys::PQsslAttributeNames(self.into()) };

        crate::ffi::vec_from_nta(raw)
            .iter()
            .map(|x| x.into())
            .collect()
    }

    /**
     * Return a pointer to an SSL-implementation-specific object describing the connection.
     *
     * See [PQsslStruct](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLSTRUCT).
     *
     * # Safety
     *
     * This function return a `void*` pointer.
     */
    pub unsafe fn ssl_struct(&self, struct_name: &str) -> *const std::ffi::c_void {
        pq_sys::PQsslStruct(self.into(), crate::cstr!(struct_name))
    }

    /**
     * Returns the SSL structure used in the connection, or null if SSL is not in use.
     *
     * See [PQgetssl](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQGETSSL).
     *
     * # Safety
     *
     * This function return a `void*` pointer.
     */
    pub unsafe fn ssl(&self) -> *const std::ffi::c_void {
        pq_sys::PQgetssl(self.into())
    }

    /**
     * See [PQconnectPoll](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTPOLL).
     */
    pub fn poll(&self) -> crate::poll::Status {
        unsafe { pq_sys::PQconnectPoll(self.into()) }.into()
    }

    /**
     * Resets the communication channel to the server.
     *
     * See [PQreset](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQRESET).
     */
    pub fn reset(&self) {
        unsafe { pq_sys::PQreset(self.into()) };
    }

    /**
     * Reset the communication channel to the server, in a nonblocking manner.
     *
     * See [PQresetStart](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQRESETSTART).
     */
    pub fn reset_start(&self) {
        unsafe { pq_sys::PQresetStart(self.into()) };
    }

    /**
     * See
     * [PQresetPoll](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQRESETPOLL).
     */
    pub fn reset_poll(&self) -> crate::poll::Status {
        unsafe { pq_sys::PQresetPoll(self.into()) }.into()
    }

    /**
     * Reports the status of the server.
     *
     * It accepts connection parameters identical to those of `libpq::Connection::with_params`. It
     * is not necessary to supply correct user name, password, or database name values to obtain
     * the server status; however, if incorrect values are provided, the server will log a failed
     * connection attempt.
     *
     * See [PQpingParams](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQPINGPARAMS).
     */
    pub fn ping_params(
        params: &std::collections::HashMap<String, String>,
        expand_dbname: bool,
    ) -> crate::ping::Status {
        let mut keywords = params.keys().map(|x| crate::cstr!(x)).collect::<Vec<_>>();
        keywords.push(std::ptr::null());

        let mut values = params.values().map(|x| crate::cstr!(x)).collect::<Vec<_>>();
        values.push(std::ptr::null());

        unsafe { pq_sys::PQpingParams(keywords.as_ptr(), values.as_ptr(), expand_dbname as i32) }
            .into()
    }

    /**
     * Reports the status of the server.
     *
     * It accepts connection parameters identical to those of `libpq::Connection::new`. It is not
     * necessary to supply correct user name, password, or database name values to obtain the
     * server status; however, if incorrect values are provided, the server will log a failed
     * connection attempt.
     *
     * See [PQping](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQPING).
     */
    pub fn ping(dsn: &str) -> crate::ping::Status {
        unsafe { pq_sys::PQping(crate::cstr!(dsn)) }.into()
    }

    /**
     * Submits a command to the server and waits for the result.
     *
     * See [PQexec](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXEC).
     */
    pub fn exec(&self, query: &str) -> crate::Result {
        unsafe { pq_sys::PQexec(self.into(), crate::cstr!(query)) }.into()
    }

    /**
     * Submits a command to the server and waits for the result, with the ability to pass
     * parameters separately from the SQL command text.
     *
     * See [PQexecParams](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXECPARAMS).
     */
    pub fn exec_params(
        &self,
        command: &str,
        param_types: &[crate::Type],
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::Result {
        let types = Self::param_types(param_types);
        let param_lengths = Self::param_lengths(param_values);
        let param_formats = Self::param_formats(param_formats);

        unsafe {
            pq_sys::PQexecParams(
                self.into(),
                crate::cstr!(command),
                param_values.len() as i32,
                if types.is_empty() {
                    std::ptr::null()
                } else {
                    types.as_ptr()
                },
                Self::param_values(param_values).as_ptr(),
                if param_lengths.is_empty() {
                    std::ptr::null()
                } else {
                    param_lengths.as_ptr()
                },
                if param_formats.is_empty() {
                    std::ptr::null()
                } else {
                    param_formats.as_ptr()
                },
                result_format as i32,
            )
        }
        .into()
    }

    /**
     * Submits a request to create a prepared statement with the given parameters, and waits for completion.
     *
     * See [PQprepare](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQPREPARE).
     */
    pub fn prepare(
        &self,
        name: Option<&str>,
        query: &str,
        param_types: &[crate::Type],
    ) -> crate::Result {
        let types = Self::param_types(param_types);

        unsafe {
            pq_sys::PQprepare(
                self.into(),
                crate::cstr!(name.unwrap_or_default()),
                crate::cstr!(query),
                types.len() as i32,
                types.as_ptr(),
            )
        }
        .into()
    }

    fn param_types(param_types: &[crate::Type]) -> Vec<u32> {
        param_types.iter().map(|x| x.oid()).collect()
    }

    fn param_formats(param_formats: &[crate::Format]) -> Vec<i32> {
        param_formats.iter().map(|x| x.into()).collect()
    }

    /**
     * Sends a request to execute a prepared statement with given parameters, and waits for the
     * result.
     *
     * See [PQexecPrepared](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXECPREPARED).
     */
    pub fn exec_prepared(
        &self,
        name: Option<&str>,
        params: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> crate::Result {
        let param_lengths = Self::param_lengths(params);
        let param_formats = Self::param_formats(param_formats);

        unsafe {
            pq_sys::PQexecPrepared(
                self.into(),
                crate::cstr!(name.unwrap_or_default()),
                params.len() as i32,
                Self::param_values(params).as_ptr(),
                if param_lengths.is_empty() {
                    std::ptr::null()
                } else {
                    param_lengths.as_ptr()
                },
                if param_formats.is_empty() {
                    std::ptr::null()
                } else {
                    param_formats.as_ptr()
                },
                result_format as i32,
            )
        }
        .into()
    }

    fn param_values(param_values: &[Option<Vec<u8>>]) -> Vec<*const i8> {
        param_values
            .iter()
            .map(|x| {
                x.as_ref()
                    .map(|x| x.as_ptr() as *const i8)
                    .unwrap_or(std::ptr::null())
            })
            .collect()
    }

    fn param_lengths(param_values: &[Option<Vec<u8>>]) -> Vec<i32> {
        param_values
            .iter()
            .map(|x| x.as_ref().map(|x| x.len() as i32).unwrap_or(0))
            .collect()
    }

    /**
     * Submits a request to obtain information about the specified prepared statement, and waits
     * for completion.
     *
     * See [PQdescribePrepared](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPREPARED).
     */
    pub fn describe_prepared(&self, name: Option<&str>) -> crate::Result {
        unsafe { pq_sys::PQdescribePrepared(self.into(), crate::cstr!(name.unwrap_or_default())) }
            .into()
    }

    /**
     * Submits a request to obtain information about the specified portal, and waits for completion.
     *
     * See [PQdescribePortal](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPORTAL).
     */
    pub fn describe_portal(&self, name: Option<&str>) -> crate::Result {
        unsafe { pq_sys::PQdescribePortal(self.into(), crate::cstr!(name.unwrap_or_default())) }
            .into()
    }

    /**
     * Escape a string for use within an SQL command.
     *
     * See
     * [PQescapeLiteral](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPELITERAL).
     */
    pub fn escape_literal(&self, str: &str) -> std::result::Result<String, String> {
        crate::escape::literal(&self, str)
    }

    /**
     * Escapes a string for use as an SQL identifier, such as a table, column, or function name.
     *
     * See
     * [PQescapeIdentifier](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEIDENTIFIER).
     */
    pub fn escape_identifier(&self, str: &str) -> std::result::Result<String, String> {
        crate::escape::identifier(&self, str)
    }

    /**
     * Escape string literals, much like `libpq::Connection::literal`.
     *
     * See
     * [PQescapeStringConn](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPESTRINGCONN).
     */
    pub fn escape_string(&self, from: &str) -> std::result::Result<String, String> {
        crate::escape::string_conn(&self, from)
    }

    /**
     * Escapes binary data for use within an SQL command with the type bytea.
     *
     * See
     * [PQescapeByteaConn](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQESCAPEBYTEACONN).
     */
    pub fn escape_bytea(&self, from: &[u8]) -> std::result::Result<Vec<u8>, String> {
        crate::escape::bytea_conn(&self, from)
    }

    /**
     * Returns the client encoding.
     *
     * See
     * [PQclientEncoding](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQCLIENTENCODING).
     */
    pub fn client_encoding(&self) -> crate::Encoding {
        unsafe { pq_sys::PQclientEncoding(self.into()) }.into()
    }

    /**
     * Sets the client encoding.
     *
     * See [PQsetClientEncoding](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQSETCLIENTENCODING).
     */
    pub fn set_client_encoding(&self, encoding: crate::Encoding) {
        unsafe {
            pq_sys::PQsetClientEncoding(self.into(), crate::cstr!(&encoding.to_string()));
        }
    }

    /**
     * Determines the verbosity of messages returned by `libpq::Connection::error_message` and
     * `libpq::Result::error_message`.
     *
     * See [PQsetErrorVerbosity](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQSETERRORVERBOSITY).
     */
    pub fn set_error_verbosity(&self, verbosity: crate::Verbosity) -> crate::Verbosity {
        unsafe { pq_sys::PQsetErrorVerbosity(self.into(), verbosity.into()) }.into()
    }

    /**
     * Enables tracing of the client/server communication to a debugging file stream.
     *
     * See [PQtrace](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQTRACE).
     */
    pub fn trace(&self, file: std::fs::File) {
        use std::os::unix::io::IntoRawFd;

        unsafe {
            let stream = libc::fdopen(file.into_raw_fd(), crate::cstr!("w"));
            pq_sys::PQtrace(self.into(), stream as *mut pq_sys::__sFILE);
        }
    }

    /**
     * Disables tracing started by `libpq::Connection::trace`.
     *
     * See [PQuntrace](https://www.postgresql.org/docs/current/libpq-control.html#LIBPQ-PQUNTRACE).
     */
    pub fn untrace(&self) {
        unsafe {
            pq_sys::PQuntrace(self.into());
        }
    }

    /**
     * Return the connection options used for the connection
     *
     * See
     * [PQconninfo](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFO).
     */
    pub fn info(&self) -> crate::connection::Info {
        unsafe {
            let raw = pq_sys::PQconninfo(self.into());
            let info = raw.into();
            pq_sys::PQconninfoFree(raw);

            info
        }
    }

    /**
     * Submits a command to the server without waiting for the result(s).
     *
     * See
     * [PQsendQuery](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERY).
     */
    pub fn send_query(&self, command: &str) -> std::result::Result<(), String> {
        let success = unsafe { pq_sys::PQsendQuery(self.into(), crate::cstr!(command)) };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Submits a command and separate parameters to the server without waiting for the result(s).
     *
     * See
     * [PQsendQueryParams](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERYPARAMS).
     */
    pub fn send_query_params(
        &self,
        command: &str,
        param_types: &[crate::Type],
        param_values: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> std::result::Result<(), String> {
        let types = Self::param_types(param_types);
        let param_lengths = Self::param_lengths(param_values);
        let param_formats = Self::param_formats(param_formats);

        let success = unsafe {
            pq_sys::PQsendQueryParams(
                self.into(),
                crate::cstr!(command),
                param_values.len() as i32,
                if types.is_empty() {
                    std::ptr::null()
                } else {
                    types.as_ptr()
                },
                Self::param_values(param_values).as_ptr(),
                if param_lengths.is_empty() {
                    std::ptr::null()
                } else {
                    param_lengths.as_ptr()
                },
                if param_formats.is_empty() {
                    std::ptr::null()
                } else {
                    param_formats.as_ptr()
                },
                result_format as i32,
            )
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Sends a request to create a prepared statement with the given parameters, without waiting
     * for completion.
     *
     * See
     * [PQsendPrepare](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDPREPARE).
     */
    pub fn send_prepare(
        &self,
        name: Option<&str>,
        query: &str,
        param_types: &[crate::Type],
    ) -> std::result::Result<(), String> {
        let types = Self::param_types(param_types);

        let success = unsafe {
            pq_sys::PQsendPrepare(
                self.into(),
                crate::cstr!(name.unwrap_or_default()),
                crate::cstr!(query),
                types.len() as i32,
                types.as_ptr(),
            )
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Sends a request to execute a prepared statement with given parameters, without waiting for the result(s).
     *
     * See [PQsendQueryPrepared](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERYPREPARED).
     */
    pub fn send_query_prepared(
        &self,
        name: Option<&str>,
        params: &[Option<Vec<u8>>],
        param_formats: &[crate::Format],
        result_format: crate::Format,
    ) -> std::result::Result<(), String> {
        let param_lengths = Self::param_lengths(params);
        let param_formats = Self::param_formats(param_formats);

        let success = unsafe {
            pq_sys::PQsendQueryPrepared(
                self.into(),
                crate::cstr!(name.unwrap_or_default()),
                params.len() as i32,
                Self::param_values(params).as_ptr(),
                if param_lengths.is_empty() {
                    std::ptr::null()
                } else {
                    param_lengths.as_ptr()
                },
                if param_formats.is_empty() {
                    std::ptr::null()
                } else {
                    param_formats.as_ptr()
                },
                result_format as i32,
            )
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Submits a request to obtain information about the specified prepared statement, without waiting for completion.
     *
     * See [PQsendDescribePortal](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPORTAL).
     */
    pub fn send_describe_prepared(&self, name: Option<&str>) -> std::result::Result<(), String> {
        let success = unsafe {
            pq_sys::PQsendDescribePrepared(self.into(), crate::cstr!(name.unwrap_or_default()))
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Submits a request to obtain information about the specified portal, without waiting for completion.
     *
     * See
     * [PQsendDescribePortal](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPORTAL).
     */
    pub fn send_describe_portal(&self, name: Option<&str>) -> std::result::Result<(), String> {
        let success = unsafe {
            pq_sys::PQsendDescribePortal(self.into(), crate::cstr!(name.unwrap_or_default()))
        };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Waits for the next result a prior `send_*` call, and returns it.
     *
     * See [PQgetResult](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQGETRESULT).
     */
    pub fn result(&self) -> Option<crate::Result> {
        let raw = unsafe { pq_sys::PQgetResult(self.into()) };

        if raw.is_null() {
            None
        } else {
            Some(raw.into())
        }
    }

    /**
     * If input is available from the server, consume it.
     *
     * See
     * [PQconsumeInput](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQCONSUMEINPUT).
     */
    pub fn consume_input(&self) -> std::result::Result<(), String> {
        let success = unsafe { pq_sys::PQconsumeInput(self.into()) };

        if success == 1 {
            Ok(())
        } else {
            Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
        }
    }

    /**
     * Returns `true` if a command is busy, that is, `Result` would block waiting for input.
     *
     * See [PQisBusy](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISBUSY).
     */
    pub fn is_busy(&self) -> bool {
        unsafe { pq_sys::PQisBusy(self.into()) == 1 }
    }

    /**
     * Sets the nonblocking status of the connection.
     *
     * See
     * [PQsetnonblocking](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSETNONBLOCKING).
     */
    pub fn set_non_blocking(&self, non_blocking: bool) -> std::result::Result<(), ()> {
        let status = unsafe { pq_sys::PQsetnonblocking(self.into(), non_blocking as i32) };

        if status < 0 {
            Err(())
        } else {
            Ok(())
        }
    }

    /**
     * Returns the blocking status of the database connection.
     *
     * See
     * [PQisnonblocking](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISNONBLOCKING).
     */
    pub fn is_non_blocking(&self) -> bool {
        unsafe { pq_sys::PQisnonblocking(self.into()) == 1 }
    }

    /**
     * Attempts to flush any queued output data to the server.
     *
     * See [PQflush](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQFLUSH).
     */
    pub fn flush(&self) -> std::result::Result<(), ()> {
        let status = unsafe { pq_sys::PQflush(self.into()) };

        if status == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    /**
     * Select single-row mode for the currently-executing query.
     *
     * See
     * [PQsetSingleRowMode](https://www.postgresql.org/docs/current/libpq-single-row-mode.html#LIBPQ-PQSETSINGLEROWMODE).
     */
    pub fn set_single_row_mode(&self) -> std::result::Result<(), ()> {
        let success = unsafe { pq_sys::PQsetSingleRowMode(self.into()) };

        if success == 1 {
            Ok(())
        } else {
            Err(())
        }
    }

    /**
     * Creates a data structure containing the information needed to cancel a command issued
     * through a particular database connection.
     *
     * See
     * [PQgetCancel](https://www.postgresql.org/docs/current/libpq-cancel.html#LIBPQ-PQGETCANCEL).
     */
    pub fn cancel(&self) -> crate::connection::Cancel {
        unsafe { pq_sys::PQgetCancel(self.into()) }.into()
    }

    /**
     * Returns the next notification from a list of unhandled notification messages received from
     * the server.
     *
     * See [PQnotifies](https://www.postgresql.org/docs/current/libpq-notify.html).
     */
    pub fn notifies(&self) -> Option<crate::connection::Notify> {
        let raw = unsafe { pq_sys::PQnotifies(self.into()) };

        if raw.is_null() {
            None
        } else {
            Some(raw.into())
        }
    }

    /**
     * Sends data to the server during `libpq::Status::CopyIn` state.
     *
     * See
     * [PQputCopyData](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQPUTCOPYDATA).
     */
    pub fn put_copy_data(&self, buffer: &str) -> std::result::Result<(), String> {
        let success = unsafe {
            pq_sys::PQputCopyData(self.into(), crate::cstr!(buffer), buffer.len() as i32)
        };

        match success {
            -1 => Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string())),
            0 => Err("Full buffers".to_string()),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    /**
     * Sends end-of-data indication to the server during `libpq::Status::CopyIn` state.
     *
     * See
     * [PQputCopyEnd](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQPUTCOPYEND).
     */
    pub fn put_copy_end(&self, errormsg: Option<&str>) -> std::result::Result<(), String> {
        let cstr = if let Some(msg) = errormsg {
            crate::cstr!(msg)
        } else {
            std::ptr::null()
        };

        let success = unsafe { pq_sys::PQputCopyEnd(self.into(), cstr) };

        match success {
            -1 => Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string())),
            0 => Err("Full buffers".to_string()),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    /**
     * Receives data from the server during `libpq::Status::CopyOut` state.
     *
     * See
     * [PQgetCopyData](https://www.postgresql.org/docs/current/libpq-copy.html#LIBPQ-PQGETCOPYDATA).
     */
    pub fn copy_data(&self, r#async: bool) -> std::result::Result<String, String> {
        let mut buffer = std::ffi::CString::new("").unwrap().into_raw();

        let success = unsafe { pq_sys::PQgetCopyData(self.into(), &mut buffer, r#async as i32) };

        match success {
            -2 => Err(self
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string())),
            -1 => Err("COPY is done".to_string()),
            0 => Err("COPY still in progress".to_string()),
            _ => Ok(crate::ffi::to_string(buffer)),
        }
    }
}

#[doc(hidden)]
impl Into<*mut pq_sys::pg_conn> for &Connection {
    fn into(self) -> *mut pq_sys::pg_conn {
        self.conn
    }
}

#[doc(hidden)]
impl Into<*mut pq_sys::pg_conn> for &mut Connection {
    fn into(self) -> *mut pq_sys::pg_conn {
        self.conn
    }
}

#[doc(hidden)]
impl Into<*const pq_sys::pg_conn> for &Connection {
    fn into(self) -> *const pq_sys::pg_conn {
        self.conn
    }
}

#[doc(hidden)]
impl std::convert::TryFrom<*mut pq_sys::pg_conn> for Connection {
    type Error = String;

    fn try_from(conn: *mut pq_sys::pg_conn) -> std::result::Result<Self, Self::Error> {
        let s = Self { conn };

        if s.status() == crate::connection::Status::Bad {
            Err(s
                .error_message()
                .unwrap_or_else(|| "Unknow error".to_string()))
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
        assert_eq!(results.status(), crate::Status::TupplesOk);
        assert_eq!(results.ntuples(), 2);
        assert_eq!(results.nfields(), 2);

        assert_eq!(results.value(0, 0), Some("1".to_string()));
        assert_eq!(results.value(0, 1), Some("2".to_string()));
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
            &[crate::Type::TEXT],
            &[Some(b"fooo\0".to_vec())],
            &[],
            crate::result::Format::Text,
        );
        assert_eq!(results.status(), crate::Status::TupplesOk);

        assert_eq!(results.value(0, 0), Some("fooo".to_string()));
    }

    #[test]
    fn exec_prepared() {
        let conn = crate::test::new_conn();
        let results = conn.prepare(Some("test1"), "SELECT $1", &[crate::Type::TEXT]);
        assert_eq!(results.status(), crate::Status::CommandOk);

        let results = conn.describe_prepared(Some("test1"));
        assert_eq!(results.nfields(), 1);

        let results = conn.exec_prepared(
            Some("test1"),
            &[Some(b"fooo\0".to_vec())],
            &[],
            crate::Format::Text,
        );
        assert_eq!(results.value(0, 0), Some("fooo".to_string()));
    }

    #[test]
    fn send_query() {
        let conn = crate::test::new_conn();
        conn.send_query("SELECT 1 as one, 2 as two from generate_series(1,2)")
            .unwrap();

        loop {
            if let Some(result) = conn.result() {
                assert_eq!(result.value(0, 0), Some("1".to_string()));
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
            &[crate::Type::TEXT],
            &[Some(b"fooo\0".to_vec())],
            &[],
            crate::Format::Text,
        )
        .unwrap();

        assert!(conn.set_single_row_mode().is_ok());

        assert_eq!(conn.result().unwrap().value(0, 0), Some("fooo".to_string()));
    }

    #[test]
    fn send_prepare() {
        let conn = crate::test::new_conn();
        conn.send_prepare(None, "SELECT $1", &[crate::Type::TEXT])
            .unwrap();
        while conn.result().is_some() {}

        conn.send_query_prepared(None, &[Some(b"fooo\0".to_vec())], &[], crate::Format::Text)
            .unwrap();
        assert_eq!(conn.result().unwrap().value(0, 0), Some("fooo".to_string()));
        assert!(conn.result().is_none());

        conn.send_describe_prepared(None).unwrap();
        assert_eq!(conn.result().unwrap().nfields(), 1);
    }

    #[test]
    fn send_error() {
        let conn = crate::test::new_conn();
        conn.send_prepare(None, "SELECT $1", &[crate::Type::TEXT])
            .unwrap();
        let result = conn.send_prepare(None, "SELECT $1", &[crate::Type::TEXT]);
        assert_eq!(
            result,
            Err("another command is already in progress\n".to_string())
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
    fn trace() {
        let conn = crate::test::new_conn();
        let file = std::fs::File::create("trace.txt").unwrap();

        conn.trace(file);
        conn.exec("SELECT 1");
        conn.untrace();
        conn.exec("SELECT 1");

        use std::io::Read;
        let mut file = std::fs::File::open("trace.txt").unwrap();
        let mut trace = String::new();
        file.read_to_string(&mut trace).unwrap();
        assert_eq!(
            trace,
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
From backend> Z
From backend (#4)> 5
From backend> I
"#
        );
    }
}
