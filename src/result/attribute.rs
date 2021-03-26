#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub tableid: crate::Oid,
    pub columnid: i32,
    pub format: i32,
    pub typid: crate::Oid,
    pub typlen: i32,
    pub atttypmod: i32,
}

#[doc(hidden)]
impl From<&&Attribute> for pq_sys::pgresAttDesc {
    fn from(attribute: &&Attribute) -> pq_sys::pgresAttDesc {
        let name = std::ffi::CString::new(attribute.name.clone()).unwrap();

        pq_sys::pgresAttDesc {
            name: name.into_raw(),
            tableid: attribute.tableid,
            columnid: attribute.columnid,
            format: attribute.format,
            typid: attribute.typid,
            typlen: attribute.typlen,
            atttypmod: attribute.atttypmod,
        }
    }
}
