/**
 * [SSL Support](https://www.postgresql.org/docs/current/libpq-ssl.html)
 */
impl Connection {
    /**
     * Allows applications to select which security libraries to initialize.
     *
     * See [PQinitOpenSSL](https://www.postgresql.org/docs/current/libpq-ssl.html#LIBPQ-PQINITOPENSSL).
     */
    pub fn init_openssl(do_ssl: bool, do_crypto: bool) {
        todo!()
    }

    /**
     * Allows applications to select which security libraries to initialize.
     *
     * See [PQinitSSL](https://www.postgresql.org/docs/current/libpq-ssl.html#LIBPQ-PQINITSSL).
     */
    pub fn init_ssl(do_ssl: bool) {
        todo!()
    }
}
