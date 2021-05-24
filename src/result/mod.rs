mod attribute;
mod error_field;

pub use attribute::*;
pub use error_field::*;

macro_rules! attr {
    ($result:ident [ $n:ident ] . $field:ident) => {
        if let Some(description) = &$result.description {
            description.get($n).map(|x| x.$field.clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Result {
    pub(crate) description: Option<crate::message::RowDescription>,
    pub(crate) params: Option<crate::message::ParameterDescription>,
    pub(crate) rows: Vec<crate::message::DataRow>,
    pub(crate) cmd_status: Option<String>,
    pub(crate) notices: Vec<crate::message::Notice>,
    pub(crate) error_message: Option<crate::message::Notice>,
    pub(crate) status: Option<crate::Status>,
}

impl Result {
    /**
     * Constructs an empty `Result` object with the given status.
     *
     * See
     * [PQmakeEmptyPGresult](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQmakeEmptyPGresult).
     */
    pub fn new(conn: Option<&crate::Connection>, status: crate::Status) -> Self {
        Self {
            status: Some(status),

            .. Default::default()
        }
    }

    pub(crate) fn add_row(&mut self, row: crate::message::DataRow) {
        self.rows.push(row);
    }

    /**
     * Returns the result status of the command.
     *
     * See [PQresultStatus](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTSTATUS).
     */
    pub fn status(&self) -> crate::Status {
        if let Some(status) = self.status {
            status
        } else if self.error_message.is_some() {
            crate::Status::FatalError
        } else if !self.rows.is_empty() {
            crate::Status::TuplesOk
        } else {
            crate::Status::CommandOk
        }
    }

    /**
     * Returns the error message associated with the command, or an empty string if there was no error.
     *
     * See [PQresultErrorMessage](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTERRORMESSAGE).
     */
    pub fn error_message(&self) -> Option<String> {
        self.error_field(crate::result::ErrorField::MessagePrimary).map(|x| x.to_string())
    }

    /**
     * Returns a reformatted version of the error message associated with a `libpq::Result` object.
     *
     * See [PQresultErrorField](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTERRORFIELD).
     */
    pub fn error_field(&self, field: crate::result::ErrorField) -> Option<&str> {
        if let Some(err) = &self.error_message {
            err.get(&field).map(|x| x.as_str())
        } else {
            None
        }
    }

    /**
     * Returns the number of rows (tuples) in the query result.
     *
     * See [PQntuples](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNTUPLES).
     */
    pub fn ntuples(&self) -> usize {
        self.rows.len()
    }

    /**
     * Returns the number of columns (fields) in each row of the query result.
     *
     * See [PQnfields](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNFIELDS).
     */
    pub fn nfields(&self) -> usize {
        if let Some(description) = &self.description {
            description.nfields()
        } else {
            0
        }
    }

    /**
     * Returns the column name associated with the given column number.
     *
     * See [PQfname](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFNAME).
     */
    pub fn field_name(&self, number: usize) -> Option<String> {
        attr!(self[number].name)
    }

    /**
     * Returns the column number associated with the given column name.
     *
     * See [PQfnumber](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFNUMBER).
     */
    pub fn field_number(&self, name: &str) -> Option<usize> {
        if let Some(description) = &self.description {
            description.iter().position(|x| x.name == name)
        } else {
            None
        }
    }

    /**
     * Returns the OID of the table from which the given column was fetched.
     *
     * See [PQftable](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFTABLE).
     */
    pub fn field_table(&self, column: usize) -> Option<crate::Oid> {
        attr!(self[column].tableid)
    }

    /**
     * Returns the column number (within its table) of the column making up the specified query
     * result column.
     *
     * See [PQftablecol](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFTABLECOL).
     */
    pub fn field_tablecol(&self, column: usize) -> usize {
        attr!(self[column].columnid).map(|x| x as usize).unwrap_or_default()
    }

    /**
     * Returns the format code indicating the format of the given column.
     *
     * See [PQfformat](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFFORMAT).
     */
    pub fn field_format(&self, column: usize) -> crate::Format {
        attr!(self[column].format).map(|x| x.into()).unwrap_or(crate::Format::Text)
    }

    /**
     * Returns the data type associated with the given column number.
     *
     * See [PQftype](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFTYPE).
     */
    pub fn field_type(&self, column: usize) -> crate::Oid {
        attr!(self[column].typid).unwrap_or(crate::oid::INVALID)
    }

    /**
     * Returns the type modifier of the column associated with the given column number.
     *
     * See [PQfmod](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFMOD).
     */
    pub fn field_mod(&self, column: usize) -> Option<i32> {
        attr!(self[column].atttypmod)
    }

    /**
     * Returns the size in bytes of the column associated with the given column number.
     *
     * `None` indicates the data type is variable-length.
     *
     * See [PQfsize](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFSIZE).
     */
    pub fn field_size(&self, column: usize) -> Option<usize> {
        attr!(self[column].typlen).map(|x| x as usize)
    }

    /**
     * Returns `true` if the `Result` contains binary data and `false` if it contains text data.
     *
     * See
     * [PQbinaryTuples](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQBINARYTUPLES).
     */
    pub fn binary_tuples(&self) -> bool {
        if let Some(description) = &self.description {
            description.binary_tuple()
        } else {
            false
        }
    }

    /**
     * Returns a single field value of one row of a `Result`.
     *
     * See [PQgetvalue](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETVALUE).
     */
    pub fn value(&self, row: usize, column: usize) -> Option<&[u8]> {
        if let Some(row) = self.rows.get(row) {
            if let Some(column) = row.get(column) {
                column.as_ref().map(|x| x.as_slice())
            } else {
                None
            }
        } else {
            None
        }
    }

    /**
     * Tests a field for a null value.
     *
     * See [PQgetisnull](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETISNULL).
     */
    pub fn is_null(&self, row: usize, column: usize) -> bool {
        if let Some(row) = self.rows.get(row) {
            row.get(column).map(|x| x.is_none()).unwrap_or(true)
        } else {
            true
        }
    }

    /**
     * Returns the actual length of a field value in bytes.
     *
     * See [PQgetlength](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETLENGTH).
     */
    pub fn length(&self, row: usize, column: usize) -> usize {
        self.value(row, column).unwrap_or_default().len()
    }

    /**
     * Returns the number of parameters of a prepared statement.
     *
     * See [PQnparams](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNPARAMS).
     */
    pub fn nparams(&self) -> usize {
        if let Some(params) = &self.params {
            params.len()
        } else {
            0
        }
    }

    /**
     * Returns the data type of the indicated statement parameter.
     *
     * See [PQparamtype](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQPARAMTYPE).
     */
    pub fn param_type(&self, param: usize) -> Option<crate::Oid> {
        if let Some(params) = &self.params {
            params.get(param).map(|x| x.oid)
        } else {
            None
        }
    }

    /**
     * Prints out all the rows and, optionally, the column names to the specified output stream.
     *
     * See [PQprint](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQPRINT).
     */
    #[cfg(unix)]
    pub fn print(&self, output: &dyn std::os::unix::io::AsRawFd, option: &crate::print::Options) {
        todo!()
    }

    /**
     * Returns the command status tag from the SQL command that generated the `Result`.
     *
     * See [PQcmdStatus](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCMDSTATUS).
     */
    pub fn cmd_status(&self) -> Option<String> {
        self.cmd_status.clone()
    }

    /**
     * Returns the number of rows affected by the SQL command.
     *
     * See [PQcmdTuples](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCMDTUPLES).
     */
    pub fn cmd_tuples(&self) -> usize {
        if let Some(description) = &self.description {
            description.len()
        } else {
            0
        }
    }

    /**
     * Returns the OID of the inserted row.
     *
     * See [PQoidValue](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQOIDVALUE).
     */
    pub fn oid_value(&self) -> Option<crate::Oid> {
        if let Some(cmd_status) = self.cmd_status() {
            cmd_status.strip_prefix("INSERT ")
                .map(|x| x.matches(char::is_numeric).collect::<String>())
                .map(|x| x.parse().unwrap_or(0))
        } else {
            None
        }
    }

    /**
     * Makes a copy of a `Result` object.
     *
     * See
     * [PQcopyResult](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQCOPYRESULT).
     */
    pub fn copy(&self, flags: i32) -> std::result::Result<Self, crate::Error> {
        Ok(self.clone())
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
    ) -> std::result::Result<(), crate::Error> {
        self.description = Some(crate::message::RowDescription::from(attributes));

        Ok(())
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
    ) -> std::result::Result<(), crate::Error> {
        if let Some(row) = self.rows.get_mut(tuple) {
            row.set_value(field, value.map(|x| x.as_bytes().to_vec()));
            Ok(())
        } else {
            Err(crate::Error::Unknow)
        }
    }

    /**
     * Retrieves the number of bytes allocated for a `Result` object.
     *
     * See [PQresultMemorySize](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTMEMORYSIZE)
     */
    pub fn memory_size(&self) -> u64 {
        std::mem::size_of::<Self>() as u64
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
        todo!()
    }
}

impl From<crate::Error> for Result {
    fn from(error: crate::Error) -> Self {
        let mut error_response = std::collections::HashMap::new();
        error_response.insert(crate::result::ErrorField::MessagePrimary, error.to_string());

        Self {
            error_message: Some(crate::message::Notice::new(error_response)),

            .. Default::default()
        }
    }
}
