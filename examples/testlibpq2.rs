/*
 * Test of the asynchronous notification interface
 *
 * Start this program, then from psql in another window do
 * ```sql
 * NOTIFY TBL2;
 * ```
 * Repeat four times to get this program to exit.
 *
 * Or, if you want to get fancy, try this:
 * populate a database with the following commands
 * (provided in examples/testlibpq2.sql):
 *
 * ```sql
 * CREATE SCHEMA TESTLIBPQ2;
 * SET search_path = TESTLIBPQ2;
 * CREATE TABLE TBL1 (i int4);
 * CREATE TABLE TBL2 (i int4);
 * CREATE RULE r1 AS ON INSERT TO TBL1 DO
 *   (INSERT INTO TBL2 VALUES (new.i); NOTIFY TBL2);
 *```
 *
 * Start this program, then from psql do this four times:
 *
 * ```sql
 * INSERT INTO TESTLIBPQ2.TBL1 VALUES (10);
 *```
 * <https://github.com/postgres/postgres/blob/REL_16_0/src/test/examples/testlibpq2.c>
 */

#[cfg(unix)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
     * If the user supplies a parameter on the command line, use it as the
     * conninfo string; otherwise default to setting dbname=postgres and using
     * environment variables or defaults for all other connection parameters.
     */
    let conninfo = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "dbname = postgres".to_string());

    let conn = libpq::Connection::new(&conninfo)?;

    /* Set always-secure search path, so malicious users can't take control. */
    let res = conn.exec("SELECT pg_catalog.set_config('search_path', '', false)");
    if res.status() != libpq::Status::TuplesOk {
        panic!("SET failed: {:?}", conn.error_message());
    }

    /*
     * Issue LISTEN command to enable notifications from the rule's NOTIFY.
     */
    let res = conn.exec("LISTEN TBL2");
    if res.status() != libpq::Status::CommandOk {
        panic!("LISTEN command failed: {:?}", conn.error_message());
    }

    /* Quit after four notifies are received. */
    let mut nnotifies = 0;

    let sock = conn.socket()?;

    let mut poll = mio::Poll::new()?;
    let mut events = mio::Events::with_capacity(1);
    poll.registry().register(
        &mut mio::unix::SourceFd(&sock),
        mio::Token(0),
        mio::Interest::READABLE,
    )?;

    while nnotifies < 4 {
        /*
         * Sleep until something happens on the connection.
         */
        poll.poll(&mut events, None)?;

        /* Now check for input */
        conn.consume_input()?;
        while let Some(notify) = conn.notifies() {
            eprintln!(
                "ASYNC NOTIFY of '{}' received from backend PID {}",
                notify.relname()?,
                notify.be_pid()
            );
            nnotifies += 1;
            conn.consume_input()?;
        }
    }

    eprintln!("Done.");

    Ok(())
}

#[cfg(not(unix))]
fn main() {}
