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
        log::debug!("Connecting with params {:?}", params);

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
        log::debug!("Starting connection to '{}'", conninfo);
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
        log::debug!("Starting connection with params {:?}", params);

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
        log::debug!("Ping '{}'", dsn);

        unsafe { pq_sys::PQping(crate::cstr!(dsn)) }.into()
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
