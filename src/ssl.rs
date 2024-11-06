#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /**
     * Application protocol selected by the TLS Application-Layer Protocol Negotiation (ALPN)
     * extension. The only protocol supported by libpq is postgresql, so this is mainly useful for
     * checking whether the server supported ALPN or not. Empty string if ALPN was not used.
     */
    Alpn,
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{self:?}").to_lowercase();
        f.write_str(&s)
    }
}

#[doc(hidden)]
impl From<&String> for Attribute {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "library" => Self::Library,
            "protocol" => Self::Protocol,
            "key_bits" => Self::KeyBits,
            "cipher" => Self::Cipher,
            "compression" => Self::Compression,
            "alpn" => Self::Alpn,
            _ => unimplemented!(),
        }
    }
}
