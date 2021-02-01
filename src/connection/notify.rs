#[derive(Clone)]
pub struct Notify {
    notify: *mut pq_sys::pgNotify,
}

impl Notify {
    /**
     * notification channel name
     */
    pub fn relname(&self) -> String {
        crate::ffi::to_string(unsafe { (*self.notify).relname })
    }

    /**
     * process ID of notifying server process
     */
    pub fn be_pid(&self) -> u32 {
        unsafe { (*self.notify).be_pid as u32 }
    }

    /**
     * notification payload string
     */
    pub fn extra(&self) -> String {
        crate::ffi::to_string(unsafe { (*self.notify).extra })
    }
}

#[doc(hidden)]
impl From<*mut pq_sys::pgNotify> for Notify {
    fn from(notify: *mut pq_sys::pgNotify) -> Self {
        Self { notify }
    }
}

#[doc(hidden)]
impl Into<*mut pq_sys::pgNotify> for &Notify {
    fn into(self) -> *mut pq_sys::pgNotify {
        self.notify
    }
}

impl Drop for Notify {
    fn drop(&mut self) {
        unsafe {
            pq_sys::PQfreemem(self.notify as *mut std::ffi::c_void);
        }
    }
}

impl std::fmt::Debug for Notify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Result")
            .field("inner", &self.notify)
            .field("relname", &self.relname())
            .field("be_pid", &self.be_pid())
            .field("extra", &self.extra())
            .finish()
    }
}
