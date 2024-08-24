/*
 * This test program shows to use LIBPQ to make multiple backend connections
 *
 * <https://github.com/postgres/postgres/blob/REL_16_0/src/test/examples/testlibpq4.c>
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();

    if args.len() < 4 {
        panic!(
            "usage: {} table_name db_name1 db_name2\n      compares two tables in two databases",
            args.nth(0).unwrap()
        );
    }

    let _tbl_name = args.nth(1).unwrap();
    let db_name1 = args.next().unwrap();
    let db_name2 = args.next().unwrap();

    /*
     * begin, by setting the parameters for a backend connection if the
     * parameters are None, then the system will try to use reasonable
     * defaults by looking up environment variables or, failing that, using
     * hardwired constants
     */

    /* make a connection to the database */
    let conn1 = libpq::Connection::set_db(None, None, None, None, Some(&db_name1))?;
    check_prepare_conn(&conn1, &db_name1);

    let conn2 = libpq::Connection::set_db(None, None, None, None, Some(&db_name2))?;
    check_prepare_conn(&conn2, &db_name2);

    /* start a transaction block */
    let res1 = conn1.exec("BEGIN");
    if res1.status() != libpq::Status::CommandOk {
        panic!("BEGIN command failed");
    }

    /*
     * fetch instances from the pg_database, the system catalog of databases
     */
    let res1 = conn1.exec("DECLARE myportal CURSOR FOR select * from pg_database");
    if res1.status() != libpq::Status::CommandOk {
        panic!("DECLARE CURSOR command failed");
    }

    let res1 = conn1.exec("FETCH ALL in myportal");
    if res1.status() != libpq::Status::TuplesOk {
        panic!("FETCH ALL command didn't return tuples properly");
    }

    /* first, print out the attribute names */
    let nfields = res1.nfields();
    for i in 0..nfields {
        print!("{:15}", res1.field_name(i)?.unwrap_or_default());
    }
    println!("\n");

    /* next, print out the instances */
    for i in 0..res1.ntuples() {
        for j in 0..nfields {
            let s = res1
                .value(i, j)
                .map(|x| String::from_utf8(x.to_vec()).unwrap())
                .unwrap_or_default();
            print!("{s:15}");
        }
        println!("");
    }

    /* close the portal */
    conn1.exec("CLOSE myportal");

    /* end the transaction */
    conn1.exec("END");

    Ok(())
}

fn check_prepare_conn(conn: &libpq::Connection, _db_name: &str) {
    if conn.status() != libpq::connection::Status::Ok {
        panic!("{:?}", conn.error_message());
    }

    /* Set always-secure search path, so malicious users can't take control. */
    let res = conn.exec("SELECT pg_catalog.set_config('search_path', '', false)");
    if res.status() != libpq::Status::TuplesOk {
        panic!("SET failed: {:?}", conn.error_message());
    }
}
