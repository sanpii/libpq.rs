#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub async_status: AsyncStatus,
    pub be_pid: i32,
    pub be_key: i32,
    pub copy: Option<crate::message::CopyInOptions>,
    pub non_blocking: bool,
    pub notifies: Vec<crate::connection::Notify>,
    pub single_row_mode: bool,
    pub result: Option<crate::Result>,
    pub parameters: std::collections::HashMap<String, String>,
    pub verbosity: crate::Verbosity,
}

impl State {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

bitflags::bitflags! {
    pub(crate) struct AsyncStatus: u16 {
        const IDLE = 0;
        const PREPARE = 1 << 0;
        const BIND = 1 << 1;
        const EXECUTE = 1 << 2;
        const DESCRIBE_STATEMENT = 1 << 3;
        const DESCRIBE_ROW = 1 << 4;
        const COPY_IN = 1 << 5;
        const COPY_OUT = 1 << 6;
        const READY = 1 << 7;
        const CONNECT = 1 << 8;
        const BEGIN = 1 << 9;
    }
}

impl Default for AsyncStatus {
    fn default() -> Self {
        Self::CONNECT
    }
}
