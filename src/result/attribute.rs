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
impl Into<pq_sys::pgresAttDesc> for &&Attribute {
    fn into(self) -> pq_sys::pgresAttDesc {
        let name = std::ffi::CString::new(self.name.clone()).unwrap();

        pq_sys::pgresAttDesc {
            name: name.into_raw(),
            tableid: self.tableid,
            columnid: self.columnid,
            format: self.format,
            typid: self.typid,
            typlen: self.typlen,
            atttypmod: self.atttypmod,
        }
    }
}
