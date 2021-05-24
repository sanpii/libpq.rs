#[derive(Clone, Debug)]
pub struct Notify {
    pub(crate) pid: i32,
    pub(crate) relname: String,
    pub(crate) extra: String,
}

impl Notify {
    /**
     * notification channel name
     */
    pub fn relname(&self) -> String {
        self.relname.clone()
    }

    /**
     * process ID of notifying server process
     */
    pub fn be_pid(&self) -> u32 {
        self.pid as u32
    }

    /**
     * notification payload string
     */
    pub fn extra(&self) -> String {
        self.extra.clone()
    }
}
