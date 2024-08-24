/*
 * Test using large objects with libpq
 *
 * <https://github.com/postgres/postgres/blob/REL_16_0/src/test/examples/testlo.c>
 */

fn main() -> libpq::errors::Result {
    let mut args = std::env::args();

    if args.len() < 4 {
        panic!(
            "usage: {} database_name in_filename out_filename",
            args.nth(0).unwrap()
        );
    }

    let database = args.nth(1).unwrap();
    let in_filename = args.next().unwrap();
    let out_filename = args.next().unwrap();

    /*
     * set up the connection
     */
    let conn = libpq::Connection::set_db(None, None, None, None, Some(&database))?;

    /* Set always-secure search path, so malicious users can't take control. */
    let res = conn.exec("SELECT pg_catalog.set_config('search_path', '', false)");
    if res.status() != libpq::Status::TuplesOk {
        panic!("SET failed: {:?}", conn.error_message());
    }

    conn.exec("begin");
    println!("importing file \"{in_filename}\" ...");
    let lobj_oid = libpq::lo::import(&conn, &in_filename);

    println!("\tas large object {lobj_oid}.");

    println!("picking out bytes 1000-2000 of the large object");
    pickout(&conn, lobj_oid, 1_000, 1_000)?;

    println!("overwriting bytes 1000-2000 of the large object with X's");
    overwrite(&conn, lobj_oid, 1_000, 1_000)?;

    println!("exporting large object to file \"{out_filename}\" ...");
    libpq::lo::export(&conn, &out_filename, lobj_oid)?;

    conn.exec("end");

    Ok(())
}

fn pickout(
    conn: &libpq::Connection,
    lobj_id: libpq::Oid,
    start: i32,
    len: usize,
) -> libpq::errors::Result {
    let lobj = libpq::lo::open(conn, lobj_id, libpq::lo::Inv::READ)?;

    lobj.lseek(start, libpq::lo::Seek::Set)?;

    let mut nread = 0;

    while len - nread > 0 {
        let mut buf = lobj.read(len - nread)?;
        let nbytes = buf.len();
        buf.insert(nbytes, '\0');
        eprint!(">>> {buf}");
        nread += nbytes;
        if nbytes <= 0 {
            break; /* no more data? */
        }
    }
    eprintln!("");

    Ok(())
}

fn overwrite(
    conn: &libpq::Connection,
    lobj_id: libpq::Oid,
    start: i32,
    len: usize,
) -> libpq::errors::Result {
    let lobj = libpq::lo::open(conn, lobj_id, libpq::lo::Inv::WRITE)?;

    lobj.lseek(start, libpq::lo::Seek::Set)?;
    let mut buf = "X".repeat(len);
    buf.insert(len - 1, '\0');

    let mut nwritten = 0;
    while len - nwritten > 0 {
        let nbytes = lobj.write(&buf[nwritten..len - nwritten])?;
        nwritten += nbytes;
    }
    eprintln!("");

    Ok(())
}
