impl Connection {
    /**
     * Return true if GSSAPI encryption is in use
     */
    pub fn gss_enc_in_use(&self) -> bool {
        todo!()
    }

    /**
     * Returns GSSAPI context if GSSAPI is in use
     *
     * # Safety
     *
     * This function returns a `void*` pointer.
     */
    pub fn gss_context(&self) -> *const std::ffi::c_void {
        todo!()
    }
}
