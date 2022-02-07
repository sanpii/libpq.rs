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
impl TryFrom<&&Attribute> for pq_sys::pgresAttDesc {
    type Error = crate::errors::Error;

    fn try_from(attribute: &&Attribute) -> Result<Self, Self::Error> {
        let name = std::ffi::CString::new(attribute.name.clone())?;

        Ok(pq_sys::pgresAttDesc {
            name: name.into_raw(),
            tableid: attribute.tableid,
            columnid: attribute.columnid,
            format: attribute.format,
            typid: attribute.typid,
            typlen: attribute.typlen,
            atttypmod: attribute.atttypmod,
        })
    }
}
