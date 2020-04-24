#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Error,
    Warning,
    Success,
}

/// A SQLSTATE error code
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub code: &'static str,
    pub kind: Kind,
    pub name: &'static str,
    pub message: Option<&'static str>,
}

include!("gen.rs");
