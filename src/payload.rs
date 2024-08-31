#[derive(Debug)]
pub(crate) struct Payload {
    buf: Vec<u8>,
    current_position: usize,
}

impl Payload {
    pub fn new() -> Self {
        Self::from(&[])
    }

    pub fn from(buf: &[u8]) -> Self {
        Self {
            buf: buf.to_vec(),
            current_position: 0,
        }
    }

    pub fn next<T: FromPayload>(&mut self) -> T {
        T::from_payload(self)
    }

    pub fn len(&self) -> usize {
        self.buf.len() - self.current_position
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn take(&mut self, n: usize) -> &[u8] {
        let start = self.current_position;
        let end = self.current_position + n;

        &self.buf[start..end]
    }

    pub fn eat(&mut self, n: usize) -> &[u8] {
        let start = self.current_position;
        self.current_position += n;
        let end = self.current_position;

        &self.buf[start..end]
    }

    pub fn extend<T: ToPayload>(&mut self, data: T) {
        self.buf
            .extend_from_slice(ToPayload::to_payload(&data).as_slice());
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buf.as_slice()
    }

    fn find(&mut self, c: u8) -> Option<usize> {
        self.buf[self.current_position..]
            .iter()
            .position(|x| x == &c)
    }
}

pub(crate) trait ToPayload {
    fn to_payload(&self) -> Vec<u8>;
}

impl ToPayload for char {
    fn to_payload(&self) -> Vec<u8> {
        log::trace!("To backend (#1)> {self:?}");

        vec![*self as u8]
    }
}

impl ToPayload for i16 {
    fn to_payload(&self) -> Vec<u8> {
        log::trace!("To backend (#2)> {self}");

        self.to_be_bytes().to_vec()
    }
}

impl ToPayload for i32 {
    fn to_payload(&self) -> Vec<u8> {
        log::trace!("To backend (#4)> {self}");

        self.to_be_bytes().to_vec()
    }
}

impl ToPayload for &str {
    fn to_payload(&self) -> Vec<u8> {
        log::trace!("To backend> {self:?}");

        self.as_bytes().to_vec()
    }
}

impl ToPayload for &String {
    fn to_payload(&self) -> Vec<u8> {
        log::trace!("To backend> {self:?}");

        self.as_bytes().to_vec()
    }
}

impl ToPayload for String {
    fn to_payload(&self) -> Vec<u8> {
        log::trace!("To backend> {self:?}");

        self.as_bytes().to_vec()
    }
}

impl ToPayload for Vec<u8> {
    fn to_payload(&self) -> Vec<u8> {
        self.clone()
    }
}

impl ToPayload for &Vec<u8> {
    fn to_payload(&self) -> Vec<u8> {
        (*self).clone()
    }
}

impl<T: ToPayload> ToPayload for &Option<T> {
    fn to_payload(&self) -> Vec<u8> {
        if let Some(data) = self {
            data.to_payload()
        } else {
            Vec::new()
        }
    }
}

pub(crate) trait FromPayload {
    fn from_payload(payload: &mut Payload) -> Self;
}

impl FromPayload for u8 {
    fn from_payload(payload: &mut Payload) -> Self {
        let x = payload.eat(1)[0];

        log::trace!("From backend (#1)> {x}");

        x
    }
}

impl FromPayload for i8 {
    fn from_payload(payload: &mut Payload) -> Self {
        let x = payload.eat(1)[0] as i8;

        log::trace!("From backend (#1)> {x}");

        x
    }
}

impl FromPayload for char {
    fn from_payload(payload: &mut Payload) -> Self {
        let x = payload.eat(1)[0] as char;

        log::trace!("From backend> {x}");

        x
    }
}

impl FromPayload for i16 {
    fn from_payload(payload: &mut Payload) -> Self {
        use std::convert::TryInto;

        let x = Self::from_be_bytes(payload.eat(2).try_into().unwrap());

        log::trace!("From backend (#2)> {x}");

        x
    }
}

impl FromPayload for i32 {
    fn from_payload(payload: &mut Payload) -> Self {
        use std::convert::TryInto;

        let x = Self::from_be_bytes(payload.eat(4).try_into().unwrap());

        log::trace!("From backend (#4)> {x}");

        x
    }
}

impl FromPayload for u32 {
    fn from_payload(payload: &mut Payload) -> Self {
        use std::convert::TryInto;

        let x = Self::from_be_bytes(payload.eat(4).try_into().unwrap());

        log::trace!("From backend (#4)> {x}");

        x
    }
}

impl FromPayload for String {
    fn from_payload(payload: &mut Payload) -> Self {
        let n = match payload.find(0) {
            Some(n) => n,
            None => return String::new(),
        };
        let x = String::from_utf8(payload.eat(n).to_vec()).unwrap();
        log::trace!("From backend> {x:?}");

        payload.eat(1);

        x
    }
}

impl FromPayload for crate::connection::Notify {
    fn from_payload(payload: &mut Payload) -> Self {
        Self {
            pid: payload.next(),
            relname: payload.next(),
            extra: payload.next(),
        }
    }
}
