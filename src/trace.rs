bitflags::bitflags! {
    /**
     * Flags controlling trace output.
     */
    #[repr(transparent)]
    pub struct Flag : u32 {
        /** Omit timestamps from each line. */
        const SuppressTimestamps = pq_sys::PQTRACE_SUPPRESS_TIMESTAMPS;
        /** Redact portions of some messages, for testing frameworks. */
        const RegressMode = pq_sys::PQTRACE_REGRESS_MODE;
    }
}
