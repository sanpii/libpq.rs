use std::convert::TryInto;

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
    pub fn new(dsn: &str) -> std::result::Result<Self, crate::Error> {
        log::debug!("Connecting to '{dsn}'");

        let connection = Self::start_with_config(&dsn.parse()?)?;
        connection.parse_input()?;

        Ok(connection)
    }

    /**
     * Makes a new connection to the database server.
     *
     * See [PQconnectdbParams](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS).
     */
    pub fn with_params(
        params: &std::collections::HashMap<String, String>,
        expand_dbname: bool,
    ) -> std::result::Result<Self, crate::Error> {
        log::debug!("Connecting with params {params:?}");

        let connection = Self::start_with_config(&params.try_into()?)?;
        connection.parse_input()?;

        Ok(connection)
    }

    /**
     * Make a connection to the database server in a nonblocking manner.
     *
     * See [PQconnectStart](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTSTART).
     */
    pub fn start(conninfo: &str) -> std::result::Result<Self, crate::Error> {
        log::debug!("Starting connection to '{conninfo}'");

        Self::start_with_config(&conninfo.parse()?)
    }

    /**
     * Make a connection to the database server in a nonblocking manner.
     *
     * See [PQconnectStartParams](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTSTARTPARAMS).
     */
    pub fn start_params(
        params: &std::collections::HashMap<String, String>,
        expand_dbname: bool,
    ) -> std::result::Result<Self, crate::Error> {
        log::debug!("Starting connection with params {params:?}");

        Self::start_with_config(&params.try_into()?)
    }

    fn start_with_config(config: &Config) -> Result<Self, crate::Error> {
        let connection = Self {
            config: config.clone(),
            socket: Socket::new(
                config.host.as_deref(),
                config.hostaddr.as_deref(),
                config.port.as_deref(),
            )?,
            state: std::sync::RwLock::new(State::new()),
        };

        connection.socket.send(crate::Message::Startup(config.clone()))?;

        Ok(connection)
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
        todo!()
    }

    /**
     * See [PQconnectPoll](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTPOLL).
     */
    pub fn poll(&self) -> crate::poll::Status {
        todo!()
    }

    /**
     * Resets the communication channel to the server.
     *
     * See [PQreset](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQRESET).
     */
    pub fn reset(&self) {
        self.reset_start();
        self.parse_input().ok();
    }

    /**
     * Reset the communication channel to the server, in a nonblocking manner.
     *
     * See [PQresetStart](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQRESETSTART).
     */
    pub fn reset_start(&self) {
        self.socket.reset();

        if let Ok(mut state) = self.state.write() {
            *state = State::default();
        }
    }

    /**
     * See
     * [PQresetPoll](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQRESETPOLL).
     */
    pub fn reset_poll(&self) -> crate::poll::Status {
        todo!()
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
        log::debug!("Ping with params {params:?}");

        match Self::with_params(params, expand_dbname) {
            Ok(_) => crate::ping::Status::Ok,
            Err(_) => crate::ping::Status::NoAttempt,
        }
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
        log::debug!("Ping '{dsn}'");

        match Self::new(dsn) {
            Ok(_) => crate::ping::Status::Ok,
            Err(_) => crate::ping::Status::NoAttempt,
        }
    }

    /**
     * Return the connection options used for the connection
     *
     * See
     * [PQconninfo](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFO).
     */
    pub fn info(&self) -> std::collections::HashMap<String, Info> {
        self.state.read().unwrap().parameters.clone();
        todo!();
    }
}
