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
