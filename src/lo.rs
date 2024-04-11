#[derive(Clone, Debug)]
pub struct LargeObject<'c> {
    fd: i32,
    conn: &'c crate::Connection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Seek {
    Set,
    Cur,
    End,
}

impl From<Seek> for i32 {
    fn from(value: Seek) -> Self {
        match value {
            Seek::Set => 0,
            Seek::Cur => 1,
            Seek::End => 2,
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Inv : i32 {
        const READ = 0x00040000;
        const WRITE = 0x00020000;
    }
}

/**
 * Creating a Large Object.
 *
 * See [lo_create](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-CREATE)
 */
pub fn create(conn: &crate::Connection, lobj_id: crate::Oid) -> crate::Oid {
    unsafe { pq_sys::lo_create(conn.into(), lobj_id) }
}

#[deprecated(note = "use lo::create()")]
pub fn creat(conn: &crate::Connection, mode: Inv) -> crate::Oid {
    unsafe { pq_sys::lo_creat(conn.into(), mode.bits()) }
}

/**
 * Importing a Large Object.
 *
 * See [lo_import](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-IMPORT)
 */
pub fn import(conn: &crate::Connection, filename: &str) -> crate::Oid {
    let c_filename = crate::ffi::to_cstr(filename);

    unsafe { pq_sys::lo_import(conn.into(), c_filename.as_ptr()) }
}

/**
 * Importing a Large Object.
 *
 * See [lo_import_with_oid](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-IMPORT)
 */
pub fn import_with_oid(
    conn: &crate::Connection,
    filename: &str,
    lobj_id: crate::Oid,
) -> crate::Oid {
    let c_filename = crate::ffi::to_cstr(filename);

    unsafe { pq_sys::lo_import_with_oid(conn.into(), c_filename.as_ptr(), lobj_id) }
}

/**
 * Exporting a Large Object.
 *
 * See [lo_export](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-EXPORT)
 */
pub fn export(
    conn: &crate::Connection,
    filename: &str,
    lobj_id: crate::Oid,
) -> crate::errors::Result {
    let c_filename = crate::ffi::to_cstr(filename);

    let success = unsafe { pq_sys::lo_export(conn.into(), lobj_id, c_filename.as_ptr()) };

    if success < 0 {
        Err(crate::errors::Error::LargeObject)
    } else {
        Ok(())
    }
}

/**
 * Removing a Large Object.
 *
 * See [lo_unlink](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-UNLINK)
 */
pub fn unlink(conn: &crate::Connection, lobj_id: crate::Oid) -> crate::errors::Result {
    let success = unsafe { pq_sys::lo_unlink(conn.into(), lobj_id) };

    if success < 0 {
        Err(crate::errors::Error::LargeObject)
    } else {
        Ok(())
    }
}

/**
 * Opening an Existing Large Object.
 *
 * See [lo_open](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-OPEN)
 */
pub fn open(
    conn: &crate::Connection,
    lobj_id: crate::Oid,
    mode: Inv,
) -> crate::errors::Result<LargeObject<'_>> {
    let fd = unsafe { pq_sys::lo_open(conn.into(), lobj_id, mode.bits()) };

    if fd < 0 {
        return Err(crate::errors::Error::Backend(format!(
            "cannot open large object {lobj_id}"
        )));
    }

    let lo = LargeObject { fd, conn };

    Ok(lo)
}

impl<'c> LargeObject<'c> {
    /**
     * Writing Data to a Large Object.
     *
     * See [lo_write](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-WRITE)
     */
    pub fn write(&self, buf: &str) -> crate::errors::Result<usize> {
        let c_buf = crate::ffi::to_cstr(buf).into_raw();
        let written = unsafe { pq_sys::lo_write(self.conn.into(), self.fd, c_buf, buf.len()) };

        if written < 0 {
            Err(crate::errors::Error::LargeObject)
        } else {
            Ok(written as usize)
        }
    }

    /**
     * Reading Data from a Large Object.
     *
     * See [lo_read](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-READ)
     */
    pub fn read(&self, len: usize) -> crate::errors::Result<String> {
        let buf = String::with_capacity(len);
        let c_buf = crate::ffi::to_cstr(&buf).into_raw();

        let read = unsafe { pq_sys::lo_read(self.conn.into(), self.fd, c_buf, len) };

        if read < 0 {
            Err(crate::errors::Error::LargeObject)
        } else {
            Ok(buf)
        }
    }

    /**
     * Seeking in a Large Object.
     *
     * See [lo_lseek](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-SEEK)
     */
    pub fn lseek(&self, offset: i32, whence: Seek) -> crate::errors::Result {
        let success = unsafe { pq_sys::lo_lseek(self.conn.into(), self.fd, offset, whence.into()) };

        if success < 0 {
            Err(crate::errors::Error::LargeObject)
        } else {
            Ok(())
        }
    }

    /**
     * Seeking in a Large Object.
     *
     * See [lo_lseek64](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-SEEK)
     */
    pub fn lseek64(&self, offset: i64, whence: Seek) -> crate::errors::Result {
        let success = unsafe {
            pq_sys::lo_lseek64(
                self.conn.into(),
                self.fd,
                offset as pq_sys::pg_int64,
                whence.into(),
            )
        };

        if success < 0 {
            Err(crate::errors::Error::LargeObject)
        } else {
            Ok(())
        }
    }

    /**
     * Obtaining the Seek Position of a Large Object.
     *
     * See [lo_tell](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-TELL)
     */
    pub fn tell(&self) -> crate::errors::Result<i32> {
        let success = unsafe { pq_sys::lo_tell(self.conn.into(), self.fd) };

        if success < 0 {
            Err(crate::errors::Error::LargeObject)
        } else {
            Ok(success)
        }
    }

    /**
     * Obtaining the Seek Position of a Large Object.
     *
     * See [lo_tell64](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-TELL)
     */
    pub fn tell64(&self) -> crate::errors::Result<i64> {
        let success = unsafe { pq_sys::lo_tell64(self.conn.into(), self.fd) };

        if success < 0 {
            Err(crate::errors::Error::LargeObject)
        } else {
            Ok(success as i64)
        }
    }

    /**
     * Truncating a Large Object.
     *
     * See [lo_truncate](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-TRUNCATE)
     */
    pub fn truncate(&self, len: usize) -> crate::errors::Result {
        let success = unsafe { pq_sys::lo_truncate(self.conn.into(), self.fd, len) };

        if success < 0 {
            Err(crate::errors::Error::LargeObject)
        } else {
            Ok(())
        }
    }

    /**
     * Truncating a Large Object.
     *
     * See [lo_truncate64](https://www.postgresql.org/docs/current/lo-interfaces.html#LO-TRUNCATE)
     */
    pub fn truncate64(&self, len: i64) -> crate::errors::Result {
        let success =
            unsafe { pq_sys::lo_truncate64(self.conn.into(), self.fd, len as pq_sys::pg_int64) };

        if success < 0 {
            Err(crate::errors::Error::LargeObject)
        } else {
            Ok(())
        }
    }
}

impl<'c> Drop for LargeObject<'c> {
    fn drop(&mut self) {
        unsafe { pq_sys::lo_close(self.conn.into(), self.fd) };
    }
}
