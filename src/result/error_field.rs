// @see https://github.com/postgres/postgres/blob/REL_12_2/src/include/postgres_ext.h#L55
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ErrorField {
    /** The severity. */
    Severity = 'S' as i32,
    /** The severity. */
    SeverityNonlocalized = 'V' as i32,
    /** The SQLSTATE code for the error. */
    Sqlstate = 'C' as i32,
    /** The primary human-readable error message. */
    MessagePrimary = 'M' as i32,
    /** An optional secondary error message carrying more detail about the problem. */
    MessageDetail = 'D' as i32,
    /** An optional suggestion what to do about the problem. */
    MessageHint = 'H' as i32,
    /**
     * A string containing a decimal integer indicating an error cursor position as an index into
     * the original statement string.
     */
    StatementPosition = 'P' as i32,
    /**
     * This is defined the same as the `StatementPosition` field, but it is used when the
     * cursor position refers to an internally generated command rather than the one submitted by
     * the client.
     */
    InternalPosition = 'p' as i32,
    /** The text of a failed internally-generated command. */
    InternalQuery = 'q' as i32,
    /** An indication of the context in which the error occurred. */
    Context = 'W' as i32,
    /**
     * If the error was associated with a specific database object, the name of the schema
     * containing that object, if any.
     */
    SchemaName = 's' as i32,
    /** If the error was associated with a specific table, the name of the table. */
    TableName = 't' as i32,
    /** If the error was associated with a specific table column, the name of the column. */
    ColumnName = 'c' as i32,
    /** If the error was associated with a specific data type, the name of the data type. */
    DatatypeName = 'd' as i32,
    /** If the error was associated with a specific constraint, the name of the constraint. */
    ConstraintName = 'n' as i32,
    /** The file name of the source-code location where the error was reported. */
    SourceFile = 'F' as i32,
    /** The line number of the source-code location where the error was reported. */
    SourceLine = 'L' as i32,
    /** The name of the source-code function reporting the error. */
    SourceFunction = 'R' as i32,
}

impl From<ErrorField> for i32 {
    fn from(error_field: ErrorField) -> i32 {
        unsafe { std::mem::transmute(error_field) }
    }
}
