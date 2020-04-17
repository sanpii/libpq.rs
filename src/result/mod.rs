mod attribute;
mod error_field;
mod format;

pub use attribute::*;
pub use error_field::*;
pub use format::*;

pub struct Result {
    result: *mut pq_sys::PGresult,
}

impl Result {
    /**
     * Constructs an empty `Result` object with the given status.
     *
     * See
     * [PQmakeEmptyPGresult](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQmakeEmptyPGresult).
     */
    pub fn new(conn: &crate::Connection, status: crate::Status) -> Self {
        let result = unsafe { pq_sys::PQmakeEmptyPGresult(conn.into(), status.into()) };

        result.into()
    }

    /**
     * Returns the result status of the command.
     *
     * See [PQresultStatus](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTSTATUS).
     */
    pub fn status(&self) -> crate::Status {
        unsafe { pq_sys::PQresultStatus(self.into()) }.into()
    }

    /**
     * Returns the error message associated with the command, or an empty string if there was no error.
     *
     * See [PQresultErrorMessage](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTERRORMESSAGE).
     */
    pub fn error_message(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQresultErrorMessage(self.into()) })
    }

    /**
     * Returns a reformatted version of the error message associated with a `libpq::Result` object.
     *
     * See [PQresultErrorField](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTERRORFIELD).
     */
    pub fn error_field(&self, field: crate::result::ErrorField) -> Option<String> {
        crate::ffi::to_option_string(unsafe {
            pq_sys::PQresultErrorField(self.into(), field.into())
        })
    }

    /**
     * Returns the number of rows (tuples) in the query result.
     *
     * See [PQntuples](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNTUPLES).
     */
    pub fn ntuples(&self) -> i32 {
        unsafe { pq_sys::PQntuples(self.into()) }
    }

    /**
     * Returns the number of columns (fields) in each row of the query result.
     *
     * See [PQnfields](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNFIELDS).
     */
    pub fn nfields(&self) -> i32 {
        unsafe { pq_sys::PQnfields(self.into()) }
    }

    /**
     * Returns the column name associated with the given column number.
     *
     * See [PQfname](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFNAME).
     */
    pub fn field_name(&self, number: i32) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQfname(self.into(), number) })
    }

    /**
     * Returns the column number associated with the given column name.
     *
     * See [PQfnumber](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFNUMBER).
     */
    pub fn field_number(&self, name: &str) -> Option<i32> {
        let number = unsafe { pq_sys::PQfnumber(self.into(), crate::cstr!(name)) };

        if number == -1 {
            None
        } else {
            Some(number)
        }
    }

    /**
     * Returns the OID of the table from which the given column was fetched.
     *
     * See [PQftable](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFTABLE).
     */
    pub fn field_table(&self, column: i32) -> Option<crate::Oid> {
        let oid = unsafe { pq_sys::PQftable(self.into(), column) };

        if oid == crate::oid::INVALID {
            None
        } else {
            Some(oid)
        }
    }

    /**
     * Returns the column number (within its table) of the column making up the specified query
     * result column.
     *
     * See [PQftablecol](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFTABLECOL).
     */
    pub fn field_tablecol(&self, column: i32) -> i32 {
        unsafe { pq_sys::PQftablecol(self.into(), column) }
    }

    /**
     * Returns the format code indicating the format of the given column.
     *
     * See [PQfformat](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFFORMAT).
     */
    pub fn field_format(&self, column: i32) -> crate::result::Format {
        unsafe { pq_sys::PQfformat(self.into(), column) }
    }

    /**
     * Returns the data type associated with the given column number.
     *
     * See [PQftype](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFTYPE).
     */
    pub fn field_type(&self, column: i32) -> Option<crate::Oid> {
        let oid = unsafe { pq_sys::PQftype(self.into(), column) };

        if oid == crate::oid::INVALID {
            None
        } else {
            Some(oid)
        }
    }

    /**
     * Returns the type modifier of the column associated with the given column number.
     *
     * See [PQfmod](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFMOD).
     */
    pub fn field_mod(&self, column: i32) -> i32 {
        unsafe { pq_sys::PQfmod(self.into(), column) }
    }

    /**
     * Returns the size in bytes of the column associated with the given column number.
     *
     * See [PQfsize](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFSIZE).
     */
    pub fn field_size(&self, column: i32) -> i32 {
        unsafe { pq_sys::PQfsize(self.into(), column) }
    }

    /**
     * Returns `true` if the `Result` contains binary data and `false` if it contains text data.
     *
     * See
     * [PQbinaryTuples](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQBINARYTUPLES).
     */
    pub fn binary_tuples(&self) -> bool {
        unsafe { pq_sys::PQbinaryTuples(self.into()) == 1 }
    }

    /**
     * Returns a single field value of one row of a `Result`.
     *
     * See [PQgetvalue](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETVALUE).
     */
    pub fn value(&self, row: i32, column: i32) -> Option<String> {
        let raw = unsafe { pq_sys::PQgetvalue(self.into(), row, column) };

        if raw.is_null() {
            None
        } else {
            Some(crate::ffi::to_string(raw))
        }
    }

    /**
     * Tests a field for a null value.
     *
     * See [PQgetisnull](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETISNULL).
     */
    pub fn is_null(&self, row: i32, column: i32) -> bool {
        unsafe { pq_sys::PQgetisnull(self.into(), row, column) == 1 }
    }

    /**
     * Returns the actual length of a field value in bytes.
     *
     * See [PQgetlength](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETLENGTH).
     */
    pub fn length(&self, row: i32, column: i32) -> i32 {
        unsafe { pq_sys::PQgetlength(self.into(), row, column) }
    }

    /**
     * Returns the number of parameters of a prepared statement.
     *
     * See [PQnparams](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNPARAMS).
     */
    pub fn nparams(&self) -> i32 {
        unsafe { pq_sys::PQnparams(self.into()) }
    }

    /**
     * Returns the data type of the indicated statement parameter.
     *
     * See [PQparamtype](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQPARAMTYPE).
     */
    pub fn param_type(&self, param: i32) -> Option<crate::Oid> {
        let oid = unsafe { pq_sys::PQparamtype(self.into(), param) };

        if oid == crate::oid::INVALID {
            None
        } else {
            Some(oid)
        }
    }

    /**
     * Prints out all the rows and, optionally, the column names to the specified output stream.
     *
     * See [PQprint](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQPRINT).
     */
    pub fn print(&self, _file: std::fs::File) {
        unimplemented!();
    }

    /**
     * Returns the command status tag from the SQL command that generated the `Result`.
     *
     * See [PQcmdStatus](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCMDSTATUS).
     */
    pub fn cmd_status(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQcmdStatus(self.into()) })
    }

    /**
     * Returns the number of rows affected by the SQL command.
     *
     * See [PQcmdTuples](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCMDTUPLES).
     */
    pub fn cmd_tuples(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQcmdTuples(self.into()) })
    }

    /**
     * Returns the OID of the inserted row.
     *
     * See [PQoidValue](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQOIDVALUE).
     */
    pub fn oid_value(&self) -> Option<crate::Oid> {
        let oid = unsafe { pq_sys::PQoidValue(self.into()) };

        if oid == crate::oid::INVALID {
            None
        } else {
            Some(oid)
        }
    }

    /**
     * See [PQoidStatus](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQOIDSTATUS).
     */
    #[deprecated(
        note = "This function is deprecated in favor of `libpq::Result::oid_value` and is not thread-safe."
    )]
    pub fn oid_status(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQoidStatus(self.into()) })
    }

    /**
     * Makes a copy of a `Result` object.
     *
     * See
     * [PQcopyResult](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQCOPYRESULT).
     */
    pub fn copy(&self, flags: i32) -> std::result::Result<Self, ()> {
        let raw = unsafe { pq_sys::PQcopyResult(self.into(), flags) };

        if raw.is_null() {
            Err(())
        } else {
            Ok(raw.into())
        }
    }

    /**
     * Sets the attributes of a PGresult object.
     *
     * See
     * [PQsetResultAttrs](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQSETRESULTATTRS).
     */
    pub fn set_attrs(
        &mut self,
        attributes: &[&crate::result::Attribute],
    ) -> std::result::Result<(), ()> {
        let mut attr = attributes.iter().map(|x| x.into()).collect::<Vec<_>>();

        let success = unsafe {
            pq_sys::PQsetResultAttrs(self.into(), attributes.len() as i32, attr.as_mut_ptr())
        };

        if success == 0 {
            Err(())
        } else {
            Ok(())
        }
    }

    /**
     * Sets a tuple field value of a `Result` object.
     *
     * See [PQsetvalue](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQSETVALUE).
     */
    pub fn set_value(
        &mut self,
        tuple: i32,
        field: i32,
        value: Option<&str>,
    ) -> std::result::Result<(), ()> {
        let (v, len) = if let Some(v) = value {
            let cstring = std::ffi::CString::new(v).unwrap();
            (cstring.into_raw(), v.len() as i32)
        } else {
            (std::ptr::null_mut(), -1)
        };

        let success = unsafe { pq_sys::PQsetvalue(self.into(), tuple, field, v, len as i32) };

        if success == 0 {
            Err(())
        } else {
            Ok(())
        }
    }

    /**
     * Allocate subsidiary storage for a `Result` object.
     *
     * See
     * [PQresultAlloc](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTALLOC).
     *
     * # Safety
     *
     * This function return a `void*` pointer.
     */
    pub unsafe fn alloc(
        &mut self,
        nbytes: usize,
    ) -> std::result::Result<*mut core::ffi::c_void, ()> {
        let space = pq_sys::PQresultAlloc(self.into(), nbytes);

        if space.is_null() {
            Err(())
        } else {
            Ok(space)
        }
    }
}

impl Drop for Result {
    fn drop(&mut self) {
        unsafe { pq_sys::PQclear(self.into()) };
    }
}

#[doc(hidden)]
impl From<*mut pq_sys::PGresult> for Result {
    fn from(result: *mut pq_sys::PGresult) -> Self {
        Result { result }
    }
}

#[doc(hidden)]
impl Into<*mut pq_sys::PGresult> for &Result {
    fn into(self) -> *mut pq_sys::PGresult {
        self.result
    }
}

#[doc(hidden)]
impl Into<*mut pq_sys::PGresult> for &mut Result {
    fn into(self) -> *mut pq_sys::PGresult {
        self.result
    }
}

#[doc(hidden)]
impl Into<*const pq_sys::PGresult> for &Result {
    fn into(self) -> *const pq_sys::PGresult {
        self.result
    }
}

impl std::fmt::Debug for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Result")
            .field("inner", &self.result)
            .field("status", &self.status())
            .field("error_message", &self.error_message())
            .field("ntuples", &self.ntuples())
            .field("nfields", &self.nfields())
            .field("cmd_status", &self.cmd_status())
            .field("cmd_tuples", &self.cmd_tuples())
            .field("oid_value", &self.oid_value())
            .field("nparams", &self.nparams())
            .finish()
    }
}
