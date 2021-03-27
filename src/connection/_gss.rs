impl Connection {
    /**
     * Return true if GSSAPI encryption is in use
     */
    pub fn gss_enc_in_use(&self) -> bool {
        unsafe { pq_sys::PQgssEncInUse(self.into()) != 0 }
    }

    /**
     * Returns GSSAPI context if GSSAPI is in use
     *
     * # Safety
     *
     * This function returns a `void*` pointer.
     */
    pub fn gss_context(&self) -> *const std::ffi::c_void {
        unsafe { pq_sys::PQgetgssctx(self.into()) }
    }
}
