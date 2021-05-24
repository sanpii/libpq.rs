#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub keyword: String,
    pub envvar: Option<String>,
    pub compiled: Option<String>,
    pub val: Option<String>,
    pub label: Option<String>,
    pub dispchar: String,
    pub dispsize: i32,
}

impl Info {
    /**
     * Returns the default connection options.
     *
     * See [PQconndefaults](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
     */
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Returns parsed connection options from the provided connection string.
     *
     * See
     * [PQconninfoParse](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE).
     */
    pub fn from(dsn: &str) -> std::result::Result<Vec<Self>, crate::Error> {
        todo!()
    }
}

impl Default for Info {
    fn default() -> Self {
        todo!()
    }
}
