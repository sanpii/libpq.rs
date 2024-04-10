bitflags::bitflags! {
    /**
     * Flags controlling trace output.
     */
    #[repr(transparent)]
    #[cfg_attr(docsrs, doc(cfg(feature = "v14")))]
    pub struct Flags : u32 {
        /** Omit timestamps from each line. */
        const SUPPRESS_TIMESTAMPS = pq_sys::PQTRACE_SUPPRESS_TIMESTAMPS;
        /** Redact portions of some messages, for testing frameworks. */
        const REGRESS_MODE = pq_sys::PQTRACE_REGRESS_MODE;
    }
}
