#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Verbosity {
    /**  returned messages include severity, primary text, and position only. */
    Terse,
    /**
     * messages that include the above plus any detail, hint, or context fields (these might span
     * multiple lines).
     */
    Default,
    /** includes all available fields. */
    Verbose,
}

#[doc(hidden)]
impl From<pq_sys::_bindgen_ty_6> for Verbosity {
    fn from(verbosity: pq_sys::_bindgen_ty_6) -> Self {
        match verbosity {
            pq_sys::_bindgen_ty_6::PQERRORS_TERSE => Self::Terse,
            pq_sys::_bindgen_ty_6::PQERRORS_DEFAULT => Self::Default,
            pq_sys::_bindgen_ty_6::PQERRORS_VERBOSE => Self::Verbose,
        }
    }
}

#[doc(hidden)]
impl Into<pq_sys::_bindgen_ty_6> for Verbosity {
    fn into(self) -> pq_sys::_bindgen_ty_6 {
        match self {
            Self::Terse => pq_sys::_bindgen_ty_6::PQERRORS_TERSE,
            Self::Default => pq_sys::_bindgen_ty_6::PQERRORS_DEFAULT,
            Self::Verbose => pq_sys::_bindgen_ty_6::PQERRORS_VERBOSE,
        }
    }
}
