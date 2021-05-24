#[derive(Clone, Copy, Debug, PartialEq)]
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
    Sqlstate,
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::Default
    }
}
