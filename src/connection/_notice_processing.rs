/**
 * [Notice Processing](https://www.postgresql.org/docs/current/libpq-notice-processing.html)
 */
impl Connection {
    /**
     * # Safety
     *
     * This function takes a `void*` pointer as argument.
     */
    pub unsafe fn set_notice_processor(
        &self,
        proc: NoticeProcessor,
        arg: *mut raw::c_void,
    ) -> NoticeProcessor {
        unsafe {
            pq_sys::PQsetNoticeProcessor(self.into(), proc, arg)
        }
    }

    /**
     * # Safety
     *
     * This function takes a `void*` pointer as argument.
     */
    pub unsafe fn set_notice_receiver(
        &self,
        proc: NoticeReceiver,
        arg: *mut raw::c_void,
    ) -> NoticeReceiver {
        unsafe {
            pq_sys::PQsetNoticeReceiver(self.into(), proc, arg)
        }
    }
}
