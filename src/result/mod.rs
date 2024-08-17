mod attribute;
mod error_field;

pub use attribute::*;
pub use error_field::*;

use std::os::raw;

#[derive(Clone)]
pub struct PQResult {
    result: *mut pq_sys::PGresult,
}

impl PQResult {
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
    pub fn error_message(&self) -> crate::errors::Result<Option<String>> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQresultErrorMessage(self.into()) })
    }

    /**
     * Returns a reformatted version of the error message associated with a `libpq::Result` object.
     *
     * See [PQresultErrorField](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTERRORFIELD).
     */
    pub fn error_field(
        &self,
        field: crate::result::ErrorField,
    ) -> crate::errors::Result<Option<&'static str>> {
        unsafe {
            let ptr = pq_sys::PQresultErrorField(self.into(), field.into());

            if ptr.is_null() {
                return Ok(None);
            }

            crate::ffi::to_option_str(ptr)
        }
    }

    /**
     * Returns the number of rows (tuples) in the query result.
     *
     * See [PQntuples](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNTUPLES).
     */
    pub fn ntuples(&self) -> usize {
        unsafe { pq_sys::PQntuples(self.into()) as usize }
    }

    /**
     * Returns the number of columns (fields) in each row of the query result.
     *
     * See [PQnfields](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNFIELDS).
     */
    pub fn nfields(&self) -> usize {
        unsafe { pq_sys::PQnfields(self.into()) as usize }
    }

    /**
     * Returns the column name associated with the given column number.
     *
     * See [PQfname](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFNAME).
     */
    pub fn field_name(&self, number: usize) -> crate::errors::Result<Option<String>> {
        let raw = unsafe { pq_sys::PQfname(self.into(), number as i32) };

        if raw.is_null() {
            Ok(None)
        } else {
            Some(crate::ffi::to_string(raw)).transpose()
        }
    }

    /**
     * Returns the column number associated with the given column name.
     *
     * See [PQfnumber](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFNUMBER).
     */
    pub fn field_number(&self, name: &str) -> Option<usize> {
        let c_name = crate::ffi::to_cstr(name);
        let number = unsafe { pq_sys::PQfnumber(self.into(), c_name.as_ptr()) };

        if number == -1 {
            None
        } else {
            Some(number as usize)
        }
    }

    /**
     * Returns the OID of the table from which the given column was fetched.
     *
     * See [PQftable](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFTABLE).
     */
    pub fn field_table(&self, column: usize) -> Option<crate::Oid> {
        let oid = unsafe { pq_sys::PQftable(self.into(), column as i32) };

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
    pub fn field_tablecol(&self, column: usize) -> usize {
        unsafe { pq_sys::PQftablecol(self.into(), column as i32) as usize }
    }

    /**
     * Returns the format code indicating the format of the given column.
     *
     * See [PQfformat](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFFORMAT).
     */
    pub fn field_format(&self, column: usize) -> crate::Format {
        unsafe { pq_sys::PQfformat(self.into(), column as i32) }.into()
    }

    /**
     * Returns the data type associated with the given column number.
     *
     * See [PQftype](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFTYPE).
     */
    pub fn field_type(&self, column: usize) -> crate::Oid {
        unsafe { pq_sys::PQftype(self.into(), column as i32) }
    }

    /**
     * Returns the type modifier of the column associated with the given column number.
     *
     * See [PQfmod](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFMOD).
     */
    pub fn field_mod(&self, column: usize) -> Option<i32> {
        let raw = unsafe { pq_sys::PQfmod(self.into(), column as i32) };

        if raw < 0 {
            None
        } else {
            Some(raw)
        }
    }

    /**
     * Returns the size in bytes of the column associated with the given column number.
     *
     * `None` indicates the data type is variable-length.
     *
     * See [PQfsize](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFSIZE).
     */
    pub fn field_size(&self, column: usize) -> Option<usize> {
        let raw = unsafe { pq_sys::PQfsize(self.into(), column as i32) };

        if raw < 0 {
            None
        } else {
            Some(raw as usize)
        }
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
    pub fn value(&self, row: usize, column: usize) -> Option<&[u8]> {
        if self.is_null(row, column) {
            None
        } else {
            let slice = unsafe {
                let raw = pq_sys::PQgetvalue(self.into(), row as i32, column as i32) as *const u8;
                let length = self.length(row, column);

                std::slice::from_raw_parts(raw, length)
            };

            Some(slice)
        }
    }

    /**
     * Tests a field for a null value.
     *
     * See [PQgetisnull](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETISNULL).
     */
    pub fn is_null(&self, row: usize, column: usize) -> bool {
        unsafe { pq_sys::PQgetisnull(self.into(), row as i32, column as i32) == 1 }
    }

    /**
     * Returns the actual length of a field value in bytes.
     *
     * See [PQgetlength](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETLENGTH).
     */
    pub fn length(&self, row: usize, column: usize) -> usize {
        unsafe { pq_sys::PQgetlength(self.into(), row as i32, column as i32) as usize }
    }

    /**
     * Returns the number of parameters of a prepared statement.
     *
     * See [PQnparams](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNPARAMS).
     */
    pub fn nparams(&self) -> usize {
        unsafe { pq_sys::PQnparams(self.into()) as usize }
    }

    /**
     * Returns the data type of the indicated statement parameter.
     *
     * See [PQparamtype](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQPARAMTYPE).
     */
    pub fn param_type(&self, param: usize) -> Option<crate::Oid> {
        let oid = unsafe { pq_sys::PQparamtype(self.into(), param as i32) };

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
    #[cfg(unix)]
    pub fn print(&self, output: &dyn std::os::unix::io::AsRawFd, option: &crate::print::Options) {
        let c_mode = crate::ffi::to_cstr("w");

        let (_c_field_name, ptr_field_name) = crate::ffi::vec_to_nta(&option.field_name);

        let c_field_sep = crate::ffi::to_cstr(&option.field_sep);
        let c_table_opt = crate::ffi::to_cstr(&option.table_opt);
        let c_caption = crate::ffi::to_cstr(&option.caption);

        let c_option = pq_sys::_PQprintOpt {
            header: option.header as pq_sys::pqbool,
            align: option.align as pq_sys::pqbool,
            standard: option.standard as pq_sys::pqbool,
            html3: option.html3 as pq_sys::pqbool,
            expanded: option.expanded as pq_sys::pqbool,
            pager: option.pager as pq_sys::pqbool,
            fieldSep: c_field_sep.as_ptr() as *mut raw::c_char,
            tableOpt: c_table_opt.as_ptr() as *mut raw::c_char,
            caption: c_caption.as_ptr() as *mut raw::c_char,
            fieldName: ptr_field_name.as_ptr() as *mut *mut raw::c_char,
        };

        unsafe {
            let stream = libc::fdopen(output.as_raw_fd(), c_mode.as_ptr());

            pq_sys::PQprint(stream as *mut _, self.into(), &c_option);
        }
    }

    /**
     * Returns the command status tag from the SQL command that generated the `Result`.
     *
     * See [PQcmdStatus](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCMDSTATUS).
     */
    pub fn cmd_status(&self) -> crate::errors::Result<Option<String>> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQcmdStatus(self.into()) })
    }

    /**
     * Returns the number of rows affected by the SQL command.
     *
     * See [PQcmdTuples](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCMDTUPLES).
     */
    pub fn cmd_tuples(&self) -> crate::errors::Result<usize> {
        let ntuples = crate::ffi::to_string(unsafe { pq_sys::PQcmdTuples(self.into()) })?;

        Ok(ntuples.parse()?)
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
    pub fn oid_status(&self) -> crate::errors::Result<Option<String>> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQoidStatus(self.into()) })
    }

    /**
     * Makes a copy of a `Result` object.
     *
     * See
     * [PQcopyResult](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQCOPYRESULT).
     */
    pub fn copy(&self, flags: i32) -> crate::errors::Result<Self> {
        let raw = unsafe { pq_sys::PQcopyResult(self.into(), flags) };

        if raw.is_null() {
            Err(crate::errors::Error::Unknow)
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
    pub fn set_attrs(&mut self, attributes: &[&crate::result::Attribute]) -> crate::errors::Result {
        let mut attr = Vec::new();

        for attribute in attributes {
            attr.push(attribute.try_into()?);
        }

        let success = unsafe {
            pq_sys::PQsetResultAttrs(self.into(), attributes.len() as i32, attr.as_mut_ptr())
        };

        if success == 0 {
            Err(crate::errors::Error::Unknow)
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
        tuple: usize,
        field: usize,
        value: Option<&str>,
    ) -> crate::errors::Result {
        let (v, len) = if let Some(v) = value {
            let cstring = std::ffi::CString::new(v)?;
            (cstring.into_raw(), v.len() as i32)
        } else {
            (std::ptr::null_mut(), -1)
        };

        let success =
            unsafe { pq_sys::PQsetvalue(self.into(), tuple as i32, field as i32, v, len) };

        if success == 0 {
            Err(crate::errors::Error::Unknow)
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
    pub unsafe fn alloc(&mut self, nbytes: usize) -> crate::errors::Result<*mut core::ffi::c_void> {
        let space = pq_sys::PQresultAlloc(self.into(), nbytes);

        if space.is_null() {
            Err(crate::errors::Error::Unknow)
        } else {
            Ok(space)
        }
    }

    /**
     * Retrieves the number of bytes allocated for a `Result` object.
     *
     * See [PQresultMemorySize](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTMEMORYSIZE)
     */
    #[cfg(feature = "v12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v12")))]
    pub fn memory_size(&self) -> u64 {
        unsafe { pq_sys::PQresultMemorySize(self.into()) as u64 }
    }

    /**
     * Really old printing routines.
     */
    #[cfg(unix)]
    pub fn display_tuples(
        &self,
        file: std::fs::File,
        fill_align: bool,
        field_sep: Option<&str>,
        print_header: bool,
        quiet: bool,
    ) {
        use std::os::unix::io::IntoRawFd;

        let c_mode = crate::ffi::to_cstr("w");

        unsafe {
            let fp = libc::fdopen(file.into_raw_fd(), c_mode.as_ptr());

            let c_sep = field_sep.map(crate::ffi::to_cstr);
            let sep = if let Some(c_sep) = c_sep {
                c_sep.as_ptr()
            } else {
                std::ptr::null()
            };

            pq_sys::PQdisplayTuples(
                self.into(),
                fp as *mut _,
                fill_align as i32,
                sep,
                print_header as i32,
                quiet as i32,
            );
        }
    }
}

unsafe impl Send for PQResult {}

unsafe impl Sync for PQResult {}

impl Drop for PQResult {
    fn drop(&mut self) {
        unsafe { pq_sys::PQclear(self.into()) };
    }
}

#[doc(hidden)]
impl From<*mut pq_sys::PGresult> for PQResult {
    fn from(result: *mut pq_sys::PGresult) -> Self {
        PQResult { result }
    }
}

#[doc(hidden)]
impl From<&PQResult> for *mut pq_sys::PGresult {
    fn from(result: &PQResult) -> *mut pq_sys::PGresult {
        result.result
    }
}

#[doc(hidden)]
impl From<&mut PQResult> for *mut pq_sys::PGresult {
    fn from(result: &mut PQResult) -> *mut pq_sys::PGresult {
        result.result
    }
}

#[doc(hidden)]
impl From<&PQResult> for *const pq_sys::PGresult {
    fn from(result: &PQResult) -> *const pq_sys::PGresult {
        result.result
    }
}

impl std::fmt::Debug for PQResult {
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
