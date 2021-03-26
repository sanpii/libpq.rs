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
impl From<pq_sys::PGTransactionStatusType> for Status {
    fn from(status: pq_sys::PGTransactionStatusType) -> Self {
        match status {
            pq_sys::PGTransactionStatusType::PQTRANS_IDLE => Self::Idle,
            pq_sys::PGTransactionStatusType::PQTRANS_ACTIVE => Self::Active,
            pq_sys::PGTransactionStatusType::PQTRANS_INTRANS => Self::InTrans,
            pq_sys::PGTransactionStatusType::PQTRANS_INERROR => Self::InError,
            pq_sys::PGTransactionStatusType::PQTRANS_UNKNOWN => Self::Unknow,
        }
    }
}
