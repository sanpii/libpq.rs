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

#[doc(hidden)]
impl Into<pq_sys::_PQprintOpt> for &Options {
    fn into(self) -> pq_sys::_PQprintOpt {
        let mut field_name = self
            .field_name
            .iter()
            .map(|x| crate::cstr!(x) as *mut i8)
            .collect::<Vec<_>>();
        field_name.push(std::ptr::null_mut());

        pq_sys::_PQprintOpt {
            header: self.header as i8,
            align: self.align as i8,
            standard: self.standard as i8,
            html3: self.html3 as i8,
            expanded: self.expanded as i8,
            pager: self.pager as i8,
            fieldSep: crate::cstr!(&self.field_sep) as *mut i8,
            tableOpt: crate::cstr!(&self.table_opt) as *mut i8,
            caption: crate::cstr!(&self.caption) as *mut i8,
            fieldName: field_name.as_mut_ptr(),
        }
    }
}
