mod uri;
mod dsn;

use std::collections::HashMap;

pub(crate) fn parse(dsn: &str) -> Result<HashMap<String, String>, crate::Error> {
    if dsn.starts_with("postgresql://") || dsn.starts_with("postgres://") {
        uri::parse(dsn)
    } else {
        dsn::parse(dsn)
    }
}
