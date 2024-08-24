/*
 * Test the Rust version of libpq, the PostgreSQL frontend library.
 *
 * <https://github.com/postgres/postgres/blob/REL_16_0/src/test/examples/testlibpq.c>
 */
fn main() -> libpq::errors::Result {
    /*
     * If the user supplies a parameter on the command line, use it as the
     * conninfo string; otherwise default to setting dbname=postgres and using
     * environment variables or defaults for all other connection parameters.
     */
    let conninfo = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "dbname = postgres".to_string());

    /* Make a connection to the database */
    let conn = libpq::Connection::new(&conninfo)?;

    /* Set always-secure search path, so malicious users can't take control. */
    let res = conn.exec("SELECT pg_catalog.set_config('search_path', '', false)");
    if res.status() != libpq::Status::TuplesOk {
        panic!("SET failed: {:?}", conn.error_message());
    }

    /*
     * Our test case here involves using a cursor, for which we must be inside
     * a transaction block.  We could do the whole thing with a single
     * PQexec() of "select * from pg_database", but that's too trivial to make
     * a good example.
     */

    /* Start a transaction block */
    let res = conn.exec("BEGIN");
    if res.status() != libpq::Status::CommandOk {
        panic!("BEGIN command failed: {:?}", conn.error_message());
    }

    /*
     * Fetch rows from pg_database, the system catalog of databases
     */
    let res = conn.exec("DECLARE myportal CURSOR FOR select * from pg_database");
    if res.status() != libpq::Status::CommandOk {
        panic!("DECLARE CURSOR failed: {:?}", conn.error_message());
    }

    let res = conn.exec("FETCH ALL in myportal");
    if res.status() != libpq::Status::TuplesOk {
        panic!("FETCH ALL failed: {:?}", conn.error_message());
    }

    /* first, print out the attribute names */
    let nfields = res.nfields();
    for i in 0..nfields {
        print!("{:15}", res.field_name(i)?.unwrap_or_default());
    }
    println!("\n");

    /* next, print out the rows */
    for i in 0..res.ntuples() {
        for j in 0..nfields {
            let s = res
                .value(i, j)
                .map(|x| String::from_utf8(x.to_vec()).unwrap())
                .unwrap_or_default();
            print!("{s:15}");
        }
        println!("");
    }

    /* close the portal ... we don't bother to check for errors ... */
    conn.exec("CLOSE myportal");

    /* end the transaction */
    conn.exec("END");

    Ok(())
}
