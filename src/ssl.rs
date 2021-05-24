#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Attribute {
    /** Name of the SSL implementation in use. (Currently, only "OpenSSL" is implemented) */
    Library,
    /**
     * SSL/TLS version in use. Common values are "TLSv1", "TLSv1.1" and "TLSv1.2", but an
     * implementation may return other strings if some other protocol is used.
     */
    Protocol,
    /** Number of key bits used by the encryption algorithm. */
    KeyBits,
    /**
     * A short name of the ciphersuite used, e.g. "DHE-RSA-DES-CBC3-SHA". The names are specific to
     * each SSL implementation.
     */
    Cipher,
    /**
     * If SSL compression is in use, returns the name of the compression algorithm, or "on" if
     * compression is used but the algorithm is not known. If compression is not in use, returns
     * "off".
     */
    Compression,
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
