use std::collections::HashMap;

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
