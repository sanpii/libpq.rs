/*
 * Test out-of-line parameters and binary I/O.
 *
 * Before running this, populate a database with the following commands
 * (provided in examples/testlibpq3.sql):
 *
 * ```sql
 * CREATE SCHEMA testlibpq3;
 * SET search_path = testlibpq3;
 * SET standard_conforming_strings = ON;
 * CREATE TABLE test1 (i int4, t text, b bytea);
 * INSERT INTO test1 values (1, 'joe''s place', '\000\001\002\003\004');
 * INSERT INTO test1 values (2, 'ho there', '\004\003\002\001\000');
 * ```
 *
 * The expected output is:
 *
 * ```
 * tuple 0: got
 *	i = (4 bytes) 1
 *	t = (11 bytes) 'joe's place'
 *	b = (5 bytes) \000\001\002\003\004
 *
 * tuple 0: got
 *	i = (4 bytes) 2
 *	t = (8 bytes) 'ho there'
 *	b = (5 bytes) \004\003\002\001\000
 * ```
 *
 * <https://github.com/postgres/postgres/blob/REL_16_0/src/test/examples/testlibpq3.c>
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let res = conn.exec("SET search_path = testlibpq3");
    if res.status() != libpq::Status::CommandOk {
        panic!("SET failed: {:?}", conn.error_message());
    }

    /*
     * The point of this program is to illustrate use of PQexecParams() with
     * out-of-line parameters, as well as binary transmission of data.
     *
     * This first example transmits the parameters as text, but receives the
     * results in binary format.  By using out-of-line parameters we can avoid
     * a lot of tedious mucking about with quoting and escaping, even though
     * the data is text.  Notice how we don't have to do anything special with
     * the quote mark in the parameter value.
     */

    /* Here is our out-of-line parameter value */
    let param_values = vec![Some(b"joe's place\0".to_vec())];

    let res = conn.exec_params(
        "SELECT * FROM test1 WHERE t = $1",
        &[], /* let the backend deduce param type */
        &param_values,
        &[],                   /* default to all text params */
        libpq::Format::Binary, /* ask for binary results */
    );

    if res.status() != libpq::Status::TuplesOk {
        panic!("SELECT failed: {:?}", conn.error_message());
    }

    show_binary_results(&res)?;

    /*
     * In this second example we transmit an integer parameter in binary form,
     * and again retrieve the results in binary form.
     *
     * Although we tell PQexecParams we are letting the backend deduce
     * parameter type, we really force the decision by casting the parameter
     * symbol in the query text.  This is a good safety measure when sending
     * binary parameters.
     */

    /* Convert integer value "2" to network byte order */
    let binary_int_val = htonl(2);

    /* Set up parameter arrays for PQexecParams */
    let param_values = vec![Some(binary_int_val)];
    let param_formats = vec![libpq::Format::Binary];

    let res = conn.exec_params(
        "SELECT * FROM test1 WHERE i = $1::int4",
        &[], /* let the backend deduce param type */
        &param_values,
        &param_formats,
        libpq::Format::Binary, /* ask for binary results */
    );

    if res.status() != libpq::Status::TuplesOk {
        panic!("SELECT failed: {:?}", conn.error_message());
    }

    show_binary_results(&res)?;

    Ok(())
}

/*
 * This function prints a query result that is a binary-format fetch from
 * a table defined as in the comment above.  We split it out because the
 * main() function uses it twice.
 */
fn show_binary_results(res: &libpq::Result) -> Result<(), Box<dyn std::error::Error>> {
    /* Use PQfnumber to avoid assumptions about field order in result */
    let i_fnum = res.field_number("i").unwrap();
    let t_fnum = res.field_number("t").unwrap();
    let b_fnum = res.field_number("b").unwrap();

    for i in 0..res.ntuples() {
        /* Get the field values (we ignore possibility they are null!) */
        let iptr = res.value(i, i_fnum).unwrap();
        let tptr = res.value(i, t_fnum).unwrap();
        let bptr = res.value(i, b_fnum).unwrap();

        /*
         * The binary representation of INT4 is in network byte order, which
         * we'd better coerce to the local byte order.
         */
        let ival = ntohl(iptr)?;

        /*
         * The binary representation of TEXT is, well, text, and since libpq
         * was nice enough to append a zero byte to it, it'll work just fine
         * as a C string.
         *
         * The binary representation of BYTEA is a bunch of bytes, which could
         * include embedded nulls so we have to pay attention to field length.
         */
        let blen = res.length(i, b_fnum);

        println!("tuple {i}: got");
        println!(" i = ({} bytes) {ival}", res.length(i, i_fnum));
        println!(
            " t = ({} bytes) '{}'",
            res.length(i, t_fnum),
            String::from_utf8(tptr.to_vec())?
        );
        print!(" b = ({blen} bytes) ");
        for j in 0..blen {
            print!("\\{:03o}", bptr[j]);
        }
        println!("\n");
    }

    Ok(())
}

fn ntohl(netlong: &[u8]) -> Result<i32, std::array::TryFromSliceError> {
    netlong[..4].try_into().map(i32::from_be_bytes)
}

fn htonl(hostlong: i32) -> Vec<u8> {
    hostlong.to_be_bytes().to_vec()
}
