#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    /** currently idle */
    Idle,
    /** a command is in progress */
    Active,
    /** idle, in a valid transaction block */
    InTrans,
    /** idle, in a failed transaction block */
    InError,
    /** reported if the connection is bad */
    Unknow,
}

#[doc(hidden)]
impl From<pq_sys::_bindgen_ty_5> for Status {
    fn from(status: pq_sys::_bindgen_ty_5) -> Self {
        match status {
            pq_sys::_bindgen_ty_5::PQTRANS_IDLE => Self::Idle,
            pq_sys::_bindgen_ty_5::PQTRANS_ACTIVE => Self::Active,
            pq_sys::_bindgen_ty_5::PQTRANS_INTRANS => Self::InTrans,
            pq_sys::_bindgen_ty_5::PQTRANS_INERROR => Self::InError,
            pq_sys::_bindgen_ty_5::PQTRANS_UNKNOWN => Self::Unknow,
        }
    }
}
