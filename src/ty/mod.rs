#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Array(crate::Oid),
    Boolean,
    Composite,
    DateTime,
    Enum,
    Geometric,
    Network,
    Numeric,
    Pseudo,
    Range(crate::Oid),
    String,
    Timestamp,
    UserDefined,
    BitString,
    Unknow,
}

#[derive(Clone, Debug)]
pub struct Type {
    pub oid: crate::Oid,
    pub descr: &'static str,
    pub name: &'static str,
    pub kind: Kind,
}

include!("gen.rs");

impl Into<crate::Oid> for Type {
    fn into(self) -> crate::Oid {
        self.oid
    }
}
