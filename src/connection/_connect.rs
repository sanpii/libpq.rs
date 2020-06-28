/**
 * [Database Connection Control Functions](https://www.postgresql.org/docs/current/libpq-connect.html)
 */
impl Connection {
    /**
     * Makes a new connection to the database server.
     *
     * See
     * [PQconnectdb](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDB).
     */
    pub fn new(dsn: &str) -> std::result::Result<Self, String> {
        log::debug!("Connecting to '{}'", dsn);

        let c_dsn = crate::ffi::to_cstr(dsn);

        unsafe { pq_sys::PQconnectdb(c_dsn.as_ptr()) }.try_into()
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
        log::debug!("Connecting with params {:?}", params);

        let (_c_keywords, ptr_keywords) = crate::ffi::vec_to_nta(&params.keys().collect::<Vec<_>>());
        let (_c_values, ptr_values) = crate::ffi::vec_to_nta(&params.values().collect::<Vec<_>>());

        unsafe {
            pq_sys::PQconnectdbParams(ptr_keywords.as_ptr(), ptr_values.as_ptr(), expand_dbname as i32)
        }
        .try_into()
    }

    /**
     * Make a connection to the database server in a nonblocking manner.
     *
     * See [PQconnectStart](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTSTART).
     */
    pub fn start(conninfo: &str) -> std::result::Result<Self, String> {
        log::debug!("Starting connection to '{}'", conninfo);

        let c_conninfo = crate::ffi::to_cstr(conninfo);

        unsafe { pq_sys::PQconnectStart(c_conninfo.as_ptr()) }.try_into()
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
        log::debug!("Starting connection with params {:?}", params);

        let (_c_keywords, ptr_keywords) = crate::ffi::vec_to_nta(&params.keys().collect::<Vec<_>>());
        let (_c_values, ptr_values) = crate::ffi::vec_to_nta(&params.values().collect::<Vec<_>>());

        unsafe {
            pq_sys::PQconnectStartParams(ptr_keywords.as_ptr(), ptr_values.as_ptr(), expand_dbname as i32)
        }
        .try_into()
    }

    /**
     * Makes a new connection to the database server.
     *
     * See [PQsetdb](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQSETDB).
     */
    pub fn set_db(
        host: Option<&str>,
        port: Option<&str>,
        options: Option<&str>,
        tty: Option<&str>,
        db_name: Option<&str>,
    ) -> std::result::Result<Self, String> {
        Self::login(host, port, options, tty, db_name, None, None)
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
        let c_host = crate::ffi::to_cstr(host.unwrap_or_default());
        let c_port = crate::ffi::to_cstr(port.unwrap_or_default());
        let c_options = crate::ffi::to_cstr(options.unwrap_or_default());
        let c_tty = crate::ffi::to_cstr(tty.unwrap_or_default());
        let c_db_name = crate::ffi::to_cstr(db_name.unwrap_or_default());
        let c_login = crate::ffi::to_cstr(login.unwrap_or_default());
        let c_pwd = crate::ffi::to_cstr(pwd.unwrap_or_default());

        unsafe {
            pq_sys::PQsetdbLogin(
                c_host.as_ptr(),
                c_port.as_ptr(),
                c_options.as_ptr(),
                c_tty.as_ptr(),
                c_db_name.as_ptr(),
                c_login.as_ptr(),
                c_pwd.as_ptr(),
            )
        }
        .try_into()
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
        log::debug!("Ping with params {:?}", params);

        let (_c_keywords, ptr_keywords) = crate::ffi::vec_to_nta(&params.keys().collect::<Vec<_>>());
        let (_c_values, ptr_values) = crate::ffi::vec_to_nta(&params.values().collect::<Vec<_>>());

        unsafe { pq_sys::PQpingParams(ptr_keywords.as_ptr(), ptr_values.as_ptr(), expand_dbname as i32) }
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
        log::debug!("Ping '{}'", dsn);

        let c_dsn = crate::ffi::to_cstr(dsn);

        unsafe { pq_sys::PQping(c_dsn.as_ptr()) }.into()
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

}
