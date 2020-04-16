#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Failed = 0,
    Reading,
    Writing,
    Ok,
    Active,
}

#[doc(hidden)]
impl From<pq_sys::_bindgen_ty_3> for Status {
    fn from(status: pq_sys::_bindgen_ty_3) -> Self {
        match status {
            pq_sys::PGRES_POLLING_FAILED => Self::Failed,
            pq_sys::PGRES_POLLING_READING => Self::Reading,
            pq_sys::PGRES_POLLING_WRITING => Self::Writing,
            pq_sys::PGRES_POLLING_OK => Self::Ok,
            pq_sys::PGRES_POLLING_ACTIVE => Self::Active,
        }
    }
}
