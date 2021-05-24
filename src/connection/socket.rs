use std::io::Write;

#[derive(Debug)]
pub(crate) struct Socket {
    stream: std::sync::RwLock<std::net::TcpStream>,
}

impl Socket {
    pub fn new(host: Option<&str>, hostaddr: Option<&str>, port: Option<&str>) -> Result<Self, crate::Error> {
        let port = port.unwrap_or("5432")
            .parse()
            .map_err(|_| crate::Error::Connect(format!("Invalid port: {:?}", port)))?;

        let stream = Self::try_connect(host, hostaddr, port)?;

        let socket = Self {
            stream: std::sync::RwLock::new(stream)
        };

        Ok(socket)
    }

    fn try_connect(host: Option<&str>, hostaddr: Option<&str>, port: u16) -> Result<std::net::TcpStream, crate::Error> {
        let host = host.unwrap_or("/tmp");

        let addr = (hostaddr.unwrap_or(host), port);

        let stream = std::net::TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;

        Ok(stream)
    }

    pub fn send(&self, message: crate::Message) -> Result<(), crate::Error> {
        let mut stream = self.stream.write()
            .map_err(|_| crate::Error::RwLock)?;

        if let Some(ty) = message.ty() {
            log::trace!("To backend> Msg {}", ty);
        }

        let payload = message.to_bytes();
        stream.write_all(&payload)?;

        match message {
            crate::Message::Query(query) => log::trace!("To backend> {:?}", query),
            _ => (),
        }

        log::trace!("To backend> Msg complete, length {}", payload.len());

        Ok(())
    }

    pub fn receive(&self) -> Result<Option<crate::Message>, crate::Error> {
        if let Some(buf) = self.receive_exact(5)? {
            use std::convert::TryInto;

            let ty = buf[0] as char;
            log::trace!("From backend> {}", ty);
            let len = i32::from_be_bytes(buf[1..].try_into().unwrap()) - 4;
            log::trace!("From backend (#4)> {}", len + 4);

            let payload = self.receive_exact(len as usize)?.unwrap_or_default();
            let message = crate::Message::from(ty, &payload)?;

            Ok(Some(message))
        } else {
            Ok(None)
        }
    }

    pub fn receive_exact(&self, len: usize) -> Result<Option<Vec<u8>>, crate::Error> {
        use std::io::Read;

        let mut buf = vec![0; len];

        let mut stream = self.stream.write()?;

        loop {
            match stream.read_exact(&mut buf[..]) {
                Ok(_) => break,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(Some(buf))
    }

    pub(crate) fn peer_addr(&self) -> Result<std::net::SocketAddr, crate::Error>{
        let peer_addr = self.stream.read()?.peer_addr().unwrap();

        Ok(peer_addr)
    }

    pub(crate) fn flush(&self) -> Result<(), crate::Error> {
        self.stream.write()?.flush()?;

        Ok(())
    }

    #[cfg(unix)]
    pub(crate) fn fd(&self) -> Result<i32, crate::Error> {
        use std::os::unix::io::AsRawFd;

        Ok(self.stream.read()?.as_raw_fd())
    }

    #[cfg(windows)]
    pub(crate) fn fd(&self) -> Result<i32, crate::Error> {
        todo!()
    }

    #[cfg(target_os = "wasi")]
    pub(crate) fn fd(&self) -> Result<i32, crate::Error> {
        todo!()
    }

    pub(crate) fn reset(&self) {
        todo!()
    }
}
