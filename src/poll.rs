#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Failed = 0,
    Reading,
    Writing,
    Ok,
    Active,
}
