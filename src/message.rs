use std::collections::HashMap;

#[derive(Debug)]
pub(crate) enum Message {
    AuthentificationOk(i32),
    BackendKeyData(i32, i32),
    Bind(BindOptions),
    BindComplete,
    CancelRequest(CancelOptions),
    CloseComplete,
    CommandComplete(String),
    CopyData(String),
    CopyDone,
    CopyFail(String),
    CopyInResponse(CopyInOptions),
    CopyOut(CopyOutOptions),
    DataRow(DataRow),
    DescribePortal(Option<String>),
    DescribeStatement(Option<String>),
    EmptyQuery,
    ErrorResponse(Notice),
    Execute,
    NoticeResponse(Notice),
    NotificationResponse(crate::connection::Notify),
    ParameterDescription(ParameterDescription),
    ParameterStatus(String, String),
    ParseComplete,
    Parse(ParseOptions),
    Query(String),
    ReadyForQuery(Status),
    RowDescription(RowDescription),
    Startup(crate::connection::Config),
    Sync,
    //AuthentificationRequest,
    //AuthentificationRequestMd5,
    //FunctionCall F
    //NoData n
    //CopyBoth w
    //CloseConnection X
    //??? p
}

impl Message {
    pub(crate) fn bind(name: Option<&str>, param_formats: &[crate::Format], param_values: &[Option<Vec<u8>>], result_format: crate::Format) -> Self {
        Self::Bind(BindOptions {
            name: name.map(|x| x.to_string()),
            param_formats: param_formats.to_vec(),
            param_values: param_values.to_vec(),
            result_format,
        })
    }

    pub(crate) fn cancel_request(cancelcode: i32, pid: i32, secret: i32) -> Self {
        Self::CancelRequest(CancelOptions {
            cancelcode,
            pid,
            secret,
        })
    }

    pub(crate) fn parse(name: Option<&str>, query: &str, param_types: &[crate::Oid]) -> Self {
        Self::Parse(ParseOptions {
            name: name.map(|x| x.to_string()),
            query: query.to_string(),
            param_types: param_types.to_vec(),
        })
    }
}

#[derive(Debug)]
pub struct BindOptions {
    name: Option<String>,
    param_formats: Vec<crate::Format>,
    param_values: Vec<Option<Vec<u8>>>,
    result_format: crate::Format,
}

#[derive(Clone, Debug)]
pub struct CopyInOptions {
    overall_format: i8,
    formats: Vec<i16>,
}

impl From<&mut crate::Payload> for CopyInOptions {
    fn from(payload: &mut crate::Payload) -> Self {
        let overall_format = payload.next();
        let numfields = payload.next::<i16>();

        let mut formats = Vec::new();

        for _ in 0..numfields {
            formats.push(payload.next());
        }

        Self {
            overall_format,
            formats,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CopyOutOptions {
    format: crate::Format,
    pub(crate) storage: Vec<crate::Format>,
}

impl From<&mut crate::Payload> for CopyOutOptions {
    fn from(payload: &mut crate::Payload) -> Self {
        let format = (payload.next::<u8>() as i32).into();
        let len = payload.next::<i16>();
        let mut storage = Vec::new();

        for _ in 0..len {
            let format = (payload.next::<i16>() as i32).into();
            storage.push(format);
        }

        Self {
            format,
            storage,
        }
    }
}

#[derive(Debug)]
pub struct ParseOptions {
    name: Option<String>,
    query: String,
    param_types: Vec<crate::Oid>,
}

#[derive(Debug)]
pub struct CancelOptions {
    cancelcode: i32,
    pid: i32,
    secret: i32,
}

#[derive(Clone, Debug)]
pub struct Notice(HashMap<crate::result::ErrorField, String>);

impl Notice {
    pub(crate) fn new(error_message: HashMap<crate::result::ErrorField, String>) -> Self {
        Self(error_message)
    }
}

impl std::ops::Deref for Notice {
    type Target = HashMap<crate::result::ErrorField, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&mut crate::Payload> for Notice {
    fn from(payload: &mut crate::Payload) -> Self {
        let mut hm = HashMap::new();

        while payload.take(1) != [0] {
            hm.insert(payload.next::<char>().into(), payload.next());
        }

        payload.eat(1);

        Self(hm)
    }
}

#[derive(Clone, Debug)]
pub struct RowDescription(Vec<crate::result::Attribute>);

impl RowDescription {
    pub(crate) fn from(attrs: &[&crate::result::Attribute]) -> Self {
        Self(attrs.iter().map(|x| (*x).clone()).collect())
    }

    pub(crate) fn nfields(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn binary_tuple(&self) -> bool {
        for attr in &self.0 {
            if attr.format == (&crate::Format::Binary).into() {
                return true;
            }
        }

        false
    }
}

impl std::ops::Deref for RowDescription {
    type Target = Vec<crate::result::Attribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&mut crate::Payload> for RowDescription {
    fn from(payload: &mut crate::Payload) -> Self {
        let mut rows = Vec::new();
        let numfields = payload.next::<i16>();

        for _ in 0..numfields {
            let row = crate::result::Attribute::from(&mut *payload);

            rows.push(row)
        }

        Self(rows)
    }
}

#[derive(Clone, Debug)]
pub struct ParameterDescription(Vec<crate::Type>);

impl std::ops::Deref for ParameterDescription {
    type Target = Vec<crate::Type>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&mut crate::Payload> for ParameterDescription {
    fn from(payload: &mut crate::Payload) -> Self {
        use std::convert::TryInto;

        let mut params = Vec::new();
        let numparams = payload.next::<i16>();

        for _ in 0..numparams {
            let typid = payload.next::<i32>() as u32;

            params.push(typid.try_into().unwrap())
        }

        Self(params)
    }
}

#[derive(Clone, Debug)]
pub struct DataRow(Vec<Option<Vec<u8>>>);

impl DataRow {
    pub(crate) fn set_value(&mut self, field: usize, value: Option<Vec<u8>>) {
        if let Some(column) = self.0.get_mut(field) {
            *column = value;
        } else {
            self.0.insert(field, value);
        }
    }
}

impl std::ops::Deref for DataRow {
    type Target = Vec<Option<Vec<u8>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&mut crate::Payload> for DataRow {
    fn from(payload: &mut crate::Payload) -> Self {
        let numfields = payload.next::<i16>();
        let mut data = Vec::new();

        for _ in 0..numfields {
            let fieldlen = payload.next::<i32>();

            if fieldlen >= 0 {
                let s = payload.eat(fieldlen as usize).to_vec();
                log::trace!("From backend ({})> {}", fieldlen, s.iter().fold(String::new(), |mut acc, x| {
                    acc.push(*x as char);
                    acc
                }));
                data.push(Some(s));
            } else {
                data.push(None)
            }
        }

        Self(data)
    }
}

#[derive(Debug)]
pub enum Status {
    Idle,
    InTrans,
    InError,
    Unknow,
}

impl From<&mut crate::Payload> for Status {
    fn from(payload: &mut crate::Payload) -> Self {
        if payload.len() != 1 {
            return Self::Unknow;
        }

        match payload.next() {
            'I' => Self::Idle,
            'T' => Self::InTrans,
            'E' => Self::InError,
            _ => Self::Unknow,
        }
    }
}

impl Message {
    pub fn from(ty: char, buf: &[u8]) -> Result<Self, crate::Error> {
        let mut payload = crate::Payload::from(buf);

        let message = match ty {
            '1' => Self::ParseComplete,
            '2' => Self::BindComplete,
            '3' => Self::CloseComplete,
            'A' => Self::NotificationResponse(payload.next()),
            'C' => Self::CommandComplete(payload.next()),
            'd' => {
                let c = payload.eat(payload.len()).to_vec();
                Self::CopyData(String::from_utf8(c).unwrap())
            }
            'D' => Self::DataRow((&mut payload).into()),
            'E' => Self::ErrorResponse((&mut payload).into()),
            'G' => Self::CopyInResponse((&mut payload).into()),
            'H' => Self::CopyOut((&mut payload).into()),
            'I' => Self::EmptyQuery,
            'K' => Self::BackendKeyData(payload.next(), payload.next()),
            'N' => Self::NoticeResponse((&mut payload).into()),
            'R' => Self::AuthentificationOk(payload.next()),
            'S' => Self::ParameterStatus(payload.next(), payload.next()),
            't' => Self::ParameterDescription((&mut payload).into()),
            'T' => Self::RowDescription((&mut payload).into()),
            'Z' => Self::ReadyForQuery((&mut payload).into()),
            _ => return Err(crate::Error::InvalidResponse(ty, buf.to_vec())),
        };

        assert!(payload.is_empty());

        Ok(message)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        if let Some(ty) = self.ty() {
            bytes.extend_from_slice(&[ty as u8]);
        }

        let payload = self.payload();

        let len = self.len(&payload);
        bytes.extend_from_slice(&len.to_be_bytes());

        bytes.extend_from_slice(&payload.as_slice());

        bytes
    }

    pub(crate) fn ty(&self) -> Option<char> {
        let ty = match self {
            Self::AuthentificationOk(_) => 'R',
            Self::BackendKeyData(_, _) => 'K',
            Self::Bind(_) => 'B',
            Self::BindComplete => '2',
            Self::CancelRequest(_) => return None,
            Self::CloseComplete => '3',
            Self::CommandComplete(_) => 'C',
            Self::CopyData(_) => 'd',
            Self::CopyDone => 'c',
            Self::CopyFail(_) => 'f',
            Self::CopyInResponse(_) => 'R',
            Self::CopyOut(_) => 'H',
            Self::DataRow(_) => 'D',
            Self::DescribePortal(_) | Self::DescribeStatement(_) => 'D',
            Self::EmptyQuery => 'I',
            Self::ErrorResponse(_) => 'E',
            Self::Execute => 'E',
            Self::NoticeResponse(_) => 'N',
            Self::NotificationResponse(_) => 'A',
            Self::ParameterDescription(_) => 't',
            Self::ParameterStatus(_, _) => 'S',
            Self::ParseComplete => '1',
            Self::Parse(_) => 'P',
            Self::Query(_) => 'Q',
            Self::ReadyForQuery(_) => 'Z',
            Self::RowDescription(_) => 'T',
            Self::Startup(_) => return None,
            Self::Sync => 'S',
            //Self::AuthentificationRequestMd5 => 'R',
            //Self::FunctionCall => 'F',
        };

        Some(ty)
    }

    fn len(&self, payload: &crate::Payload) -> i32 {
        payload.len() as i32 + 4
    }

    fn payload(&self) -> crate::Payload {
        match self {
            Self::Bind(BindOptions { name, param_values, param_formats, result_format }) => {
                let mut payload = crate::Payload::new();

                // Unnamed portal
                payload.extend('\0');

                payload.extend(name);
                payload.extend('\0');

                payload.extend(param_formats.len() as i16);

                for format in param_formats {
                    payload.extend(*format as i16);
                }

                payload.extend(param_values.len() as i16);

                for (n, param) in param_values.iter().enumerate() {
                    if let Some(param) = param {
                        // libpq required \0 for text format to use strlen but
                        // don’t send its.
                        let paramlen = if matches!(param_formats.get(n), Some(crate::Format::Binary)) {
                            param.len()
                        } else {
                            param.len() - 1
                        };
                        payload.extend(paramlen as i32);
                        payload.extend(param[..paramlen].to_vec());
                    } else {
                        payload.extend(-1_i32);
                    }
                }

                payload.extend(1_i16);
                payload.extend(*result_format as i16);

                payload
            }
            Self::CancelRequest(CancelOptions { cancelcode, pid, secret }) => {
                let mut payload = crate::Payload::new();
                payload.extend(*cancelcode);
                payload.extend(*pid);
                payload.extend(*secret);

                payload
            }
            Self::CloseComplete => crate::Payload::new(),
            Self::CopyData(data) => crate::Payload::from(data.as_bytes()),
            Self::CopyFail(errormsg) => {
                let mut payload = crate::Payload::from(errormsg.as_bytes());
                payload.extend('\0');

                payload
            }
            Self::CopyDone => crate::Payload::new(),
            Self::DescribeStatement(name) => {
                log::trace!("To backend> S");
                let mut payload = crate::Payload::from(&[b'S']);
                payload.extend(name);
                payload.extend('\0');
                payload
            }
            Self::DescribePortal(name) => {
                log::trace!("To backend> P");
                let mut payload = crate::Payload::from(&[b'P']);
                payload.extend(name);
                payload.extend('\0');
                payload
            }
            Self::Execute => {
                let mut payload = crate::Payload::new();

                // Unnamed portal
                payload.extend('\0');

                // No rowlimit
                payload.extend(0_i32);

                payload
            }
            Self::Parse(ParseOptions { name, query, param_types }) => {
                let mut payload = crate::Payload::new();

                payload.extend(name);
                payload.extend('\0');

                payload.extend(query);
                payload.extend('\0');

                payload.extend(param_types.len() as i16);

                for ty in param_types {
                    payload.extend(*ty as i32);
                }

                payload
            }
            Self::Query(s) => {
                let mut payload = crate::Payload::new();
                payload.extend(s);
                payload.extend('\0');

                payload
            }
            Self::Startup(config) => {
                let hm: std::collections::HashMap<_, _> = config.into();

                let mut payload = hm.iter().fold(crate::Payload::from(&[0, 3, 0, 0]), |mut acc, (k, v)| {
                    acc.extend(*k);
                    acc.extend('\0');
                    acc.extend(v);
                    acc.extend('\0');

                    acc
                });

                payload.extend('\0');
                payload
            }
            Self::Sync => crate::Payload::new(),
            _ => unreachable!(),
        }
    }
}
