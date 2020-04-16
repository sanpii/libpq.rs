pub type Oid = postgres_types::Oid;

// @see https://github.com/postgres/postgres/blob/REL_12_2/src/include/postgres_ext.h#L34
pub(crate) const INVALID: Oid = 0;
