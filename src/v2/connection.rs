use std::collections::HashMap;

pub fn put_copy_data(
    connection: &crate::Connection,
    buffer: &[u8],
) -> std::result::Result<(), String> {
    log::trace!("Sending copy data");

    let c_buffer = unsafe { std::ffi::CString::from_vec_unchecked(buffer.to_vec()) };

    let success =
        unsafe { pq_sys::PQputCopyData(connection.into(), c_buffer.as_ptr(), buffer.len() as i32) };

    match success {
        -1 => Err(connection
            .error_message()
            .unwrap_or_else(|| "Unknow error".to_string())),
        0 => Err("Full buffers".to_string()),
        1 => Ok(()),
        _ => unreachable!(),
    }
}

pub fn info(connection: &crate::Connection) -> HashMap<String, crate::connection::Info> {
    let mut infos = HashMap::new();

    unsafe {
        let mut i = 0;
        let raw = pq_sys::PQconninfo(connection.into());

        loop {
            let current = raw.offset(i);

            if (*current).keyword.is_null() {
                break;
            }

            let info: crate::connection::Info = current.into();
            infos.insert(info.keyword.clone(), info);
            i += 1;
        }

        pq_sys::PQconninfoFree(raw);
    }

    infos
}
