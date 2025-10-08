#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ContextVisibility {
    Never,
    Errors,
    Always,
}

impl From<ContextVisibility> for pq_sys::PGContextVisibility {
    fn from(value: ContextVisibility) -> Self {
        match value {
            ContextVisibility::Never => Self::PQSHOW_CONTEXT_NEVER,
            ContextVisibility::Errors => Self::PQSHOW_CONTEXT_ERRORS,
            ContextVisibility::Always => Self::PQSHOW_CONTEXT_ALWAYS,
        }
    }
}

impl From<pq_sys::PGContextVisibility> for ContextVisibility {
    fn from(value: pq_sys::PGContextVisibility) -> Self {
        match value {
            pq_sys::PGContextVisibility::PQSHOW_CONTEXT_NEVER => Self::Never,
            pq_sys::PGContextVisibility::PQSHOW_CONTEXT_ERRORS => Self::Errors,
            pq_sys::PGContextVisibility::PQSHOW_CONTEXT_ALWAYS => Self::Always,
        }
    }
}
