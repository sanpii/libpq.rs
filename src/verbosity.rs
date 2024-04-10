#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verbosity {
    /**  returned messages include severity, primary text, and position only. */
    Terse,
    /**
     * messages that include the above plus any detail, hint, or context fields (these might span
     * multiple lines).
     */
    Default,
    /** includes all available fields. */
    Verbose,
    /** only error severity and SQLSTATE code */
    #[cfg(feature = "v11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v11")))]
    Sqlstate,
}

#[doc(hidden)]
impl From<pq_sys::PGVerbosity> for Verbosity {
    fn from(verbosity: pq_sys::PGVerbosity) -> Self {
        match verbosity {
            pq_sys::PGVerbosity::PQERRORS_TERSE => Self::Terse,
            pq_sys::PGVerbosity::PQERRORS_DEFAULT => Self::Default,
            pq_sys::PGVerbosity::PQERRORS_VERBOSE => Self::Verbose,
            #[cfg(feature = "v11")]
            #[cfg_attr(docsrs, doc(cfg(feature = "v11")))]
            pq_sys::PGVerbosity::PQERRORS_SQLSTATE => Self::Sqlstate,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

#[doc(hidden)]
impl From<Verbosity> for pq_sys::PGVerbosity {
    fn from(verbosity: Verbosity) -> Self {
        match verbosity {
            Verbosity::Terse => pq_sys::PGVerbosity::PQERRORS_TERSE,
            Verbosity::Default => pq_sys::PGVerbosity::PQERRORS_DEFAULT,
            Verbosity::Verbose => pq_sys::PGVerbosity::PQERRORS_VERBOSE,
            #[cfg(feature = "v11")]
            #[cfg_attr(docsrs, doc(cfg(feature = "v11")))]
            Verbosity::Sqlstate => pq_sys::PGVerbosity::PQERRORS_SQLSTATE,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
