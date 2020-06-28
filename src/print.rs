pub struct Options {
    /* print output field headings and row count */
    pub header: bool,
    /* fill align the fields */
    pub align: bool,
    /* old brain dead format */
    pub standard: bool,
    /* output HTML tables */
    pub html3: bool,
    /* expand tables */
    pub expanded: bool,
    /* use pager for output if needed */
    pub pager: bool,
    /* field separator */
    pub field_sep: String,
    /* attributes for HTML table element */
    pub table_opt: String,
    /* HTML table caption */
    pub caption: String,
    /* null-terminated array of replacement field names */
    pub field_name: Vec<String>,
}
