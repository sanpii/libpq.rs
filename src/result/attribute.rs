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

impl From<&mut crate::Payload> for Attribute {
    fn from(payload: &mut crate::Payload) -> Self {
        Self {
            name: payload.next(),
            tableid: payload.next(),
            columnid: payload.next::<i16>() as i32,
            typid: payload.next(),
            typlen: payload.next::<i16>() as i32,
            atttypmod: payload.next(),
            format: payload.next::<i16>() as i32,
        }
    }
}
