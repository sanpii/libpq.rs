mod channel_binding;
mod gssencmode;
mod parser;
mod sslmode;
mod target_session_attrs;

pub use channel_binding::*;
pub use gssencmode::*;
pub use sslmode::*;
pub use target_session_attrs::*;

use std::collections::HashMap;

/**
 * Connection configuration.
 *
 * See <https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PARAMKEYWORDS>.
 */
#[derive(Clone, Debug, Default)]
pub struct Config {
    pub application_name: Option<String>,
    pub channel_binding: Option<ChannelBinding>,
    pub client_encoding: Option<String>,
    pub connect_timeout: Option<i32>,
    pub dbname: Option<String>,
    pub fallback_application_name: Option<String>,
    pub gssencmode: Option<GssEncMode>,
    pub gsslib: Option<String>,
    pub hostaddr: Option<String>,
    pub host: Option<String>,
    pub keepalives_count: Option<i32>,
    pub keepalives_idle: Option<i32>,
    pub keepalives_interval: Option<i32>,
    pub keepalives: Option<bool>,
    pub krbsrvname: Option<String>,
    pub options: Option<String>,
    pub passfile: Option<String>,
    pub password: Option<String>,
    pub port: Option<String>,
    pub replication: Option<String>,
    pub requirepeer: Option<String>,
    pub service: Option<String>,
    pub sslcert: Option<String>,
    pub sslcompression: Option<bool>,
    pub sslcrl: Option<String>,
    pub sslkey: Option<String>,
    pub ssl_max_protocol_version: Option<String>,
    pub ssl_min_protocol_version: Option<String>,
    pub sslmode: Option<SslMode>,
    pub sslpassword: Option<String>,
    pub sslrootcert: Option<String>,
    pub target_session_attrs: Option<TargetSessionAttrs>,
    pub tcp_user_timeout: Option<i32>,
    pub user: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user(&self) -> String {
        match &self.user {
            Some(user) => user.clone(),
            None => std::env::var("USER").unwrap(),
        }
    }

    pub fn database(&self) -> String {
        match &self.dbname {
            Some(dbname) => dbname.clone(),
            None => std::env::var("USER").unwrap(),
        }
    }
}

impl std::str::FromStr for Config {
    type Err = crate::Error;

    fn from_str(dsn: &str) -> Result<Self, Self::Err> {
        use std::convert::TryInto;

        let params = parser::parse(dsn)?;

        (&params).try_into()
    }
}

impl std::convert::TryFrom<&HashMap<String, String>> for Config {
    type Error = crate::Error;

    fn try_from(params: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let config = Self {
            application_name: params.get("application_name").cloned(),
            channel_binding: params.get("channel_binding").map(|x| x.parse()).transpose()?,
            client_encoding: params.get("client_encoding").cloned(),
            connect_timeout: params.get("connect_timeout").map(|x| x.parse()).transpose().map_err(|e| crate::Error::Parse(format!("Invalid connect_timeout: {}", e)))?,
            dbname: params.get("dbname").cloned(),
            fallback_application_name: params.get("fallback_application_name").cloned(),
            gssencmode: params.get("gssencmode").map(|x| x.parse()).transpose()?,
            gsslib: params.get("gsslib").cloned(),
            hostaddr: params.get("hostaddr").cloned(),
            host: params.get("host").cloned(),
            keepalives_count: params.get("keepalives_count").map(|x| x.parse()).transpose().map_err(|e| crate::Error::Parse(format!("Invalid keepalives_count: {}", e)))?,
            keepalives_idle: params.get("keepalives_idle").map(|x| x.parse()).transpose().map_err(|e| crate::Error::Parse(format!("Invalid keepalives_idle: {}", e)))?,
            keepalives_interval: params.get("keepalives_interval").map(|x| x.parse()).transpose().map_err(|e| crate::Error::Parse(format!("Invalid keepalives_interval: {}", e)))?,
            keepalives: params.get("keepalives").map(|x| x == "1"),
            krbsrvname: params.get("krbsrvname").cloned(),
            options: params.get("options").cloned(),
            passfile: params.get("passfile").cloned(),
            password: params.get("password").cloned(),
            port: params.get("port").cloned(),
            replication: params.get("replication").cloned(),
            requirepeer: params.get("requirepeer").cloned(),
            service: params.get("service").cloned(),
            sslcert: params.get("sslcert").cloned(),
            sslcompression: params.get("sslcompression").map(|x| x == "1"),
            sslcrl: params.get("sslcrl").cloned(),
            sslkey: params.get("sslkey").cloned(),
            ssl_max_protocol_version: params.get("ssl_max_protocol_version").cloned(),
            ssl_min_protocol_version: params.get("ssl_min_protocol_version").cloned(),
            sslmode: params.get("sslmode").map(|x| x.parse()).transpose()?,
            sslpassword: params.get("sslpassword").cloned(),
            sslrootcert: params.get("sslrootcert").cloned(),
            target_session_attrs: params.get("target_session_attrs").map(|x| x.parse()).transpose()?,
            tcp_user_timeout: params.get("tcp_user_timeout").map(|x| x.parse()).transpose().map_err(|e| crate::Error::Parse(format!("Invalid tcp_user_timeout: {}", e)))?,
            user: params.get("user").cloned(),
        };

        Ok(config)
    }
}

impl From<&Config> for HashMap<&'static str, String> {
    fn from(config: &Config) -> Self {
        let mut hm = HashMap::new();

        hm.insert("user", config.user());
        hm.insert("database", config.database());

        hm
    }
}

macro_rules! display {
    ($f:ident, $config:ident . $name:ident) => {
        if let Some($name) = &$config.$name {
            write!($f, "{}='{}' ", stringify!($name), $name)?;
        }
    };
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display!(f, self.application_name);
        display!(f, self.channel_binding);
        display!(f, self.client_encoding);
        display!(f, self.connect_timeout);
        display!(f, self.dbname);
        display!(f, self.fallback_application_name);
        display!(f, self.gssencmode);
        display!(f, self.gsslib);
        display!(f, self.hostaddr);
        display!(f, self.host);
        display!(f, self.keepalives_count);
        display!(f, self.keepalives_idle);
        display!(f, self.keepalives_interval);
        display!(f, self.keepalives);
        display!(f, self.krbsrvname);
        display!(f, self.options);
        display!(f, self.passfile);
        display!(f, self.password);
        display!(f, self.port);
        display!(f, self.replication);
        display!(f, self.requirepeer);
        display!(f, self.service);
        display!(f, self.sslcert);
        display!(f, self.sslcompression);
        display!(f, self.sslcrl);
        display!(f, self.sslkey);
        display!(f, self.ssl_max_protocol_version);
        display!(f, self.ssl_min_protocol_version);
        display!(f, self.sslmode);
        display!(f, self.sslpassword);
        display!(f, self.sslrootcert);
        display!(f, self.target_session_attrs);
        display!(f, self.tcp_user_timeout);
        display!(f, self.user);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        let tests = vec![
            ("host=host port=12345", Ok("host='host' port='12345' ")),
            //("postgresql://uri-user:secret@host:12345/db", Ok("user='uri-user' password='secret' dbname='db' host='host' port='12345' ")),
            //("postgresql://uri-user@host:12345/db", Ok("user='uri-user' dbname='db' host='host' port='12345' ")),
            //("postgresql://uri-user@host/db", Ok("user='uri-user' dbname='db' host='host' ")),
            //("postgresql://host:12345/db", Ok("dbname='db' host='host' port='12345' ")),
            //("postgresql://host/db", Ok("dbname='db' host='host' ")),
            //("postgresql://uri-user@host:12345/", Ok("user='uri-user' host='host' port='12345' ")),
            //("postgresql://uri-user@host/", Ok("user='uri-user' host='host' ")),
            //("postgresql://uri-user@", Ok("user='uri-user' ")),
            //("postgresql://host:12345/", Ok("host='host' port='12345' ")),
            //("postgresql://host:12345", Ok("host='host' port='12345' ")),
            //("postgresql://host/db", Ok("dbname='db' host='host' ")),
            //("postgresql://host/", Ok("host='host' ")),
            //("postgresql://host", Ok("host='host' ")),
            ("postgresql://", Ok("")),
            //("postgresql://?hostaddr=127.0.0.1", Ok("hostaddr='127.0.0.1' ")),
            //("postgresql://example.com?hostaddr=63.1.2.4", Ok("host='example.com' hostaddr='63.1.2.4' ")),
            //("postgresql://%68ost/", Ok("host='host' ")),
            //("postgresql://host/db?user=uri-user", Ok("user='uri-user' dbname='db' host='host' ")),
            //("postgresql://host/db?user=uri-user&port=12345", Ok("user='uri-user' dbname='db' host='host' port='12345' ")),
            //("postgresql://host/db?u%73er=someotheruser&port=12345", Ok("user='someotheruser' dbname='db' host='host' port='12345' ")),
            //("postgresql://host/db?u%7aer=someotheruser&port=12345", Err("invalid URI query parameter: \"uzer\" ")),
            //("postgresql://host:12345?user=uri-user", Ok("user='uri-user' host='host' port='12345' ")),
            //("postgresql://host?user=uri-user", Ok("user='uri-user' host='host' ")),
            //("postgresql://host?", Ok("host='host' ")),
            //("postgresql://[::1]:12345/db", Ok("dbname='db' host='::1' port='12345' ")),
            //("postgresql://[::1]/db", Ok("dbname='db' host='::1' ")),
            //("postgresql://[2001:db8::1234]/", Ok("host='2001:db8::1234' ")),
            //("postgresql://[200z:db8::1234]/", Ok("host='200z:db8::1234' ")),
            //("postgresql://[::1]", Ok("host='::1' ")),
            ("postgres://", Ok("")),
            ("postgres:///", Ok("")),
            //("postgres:///db", Ok("dbname='db' ")),
            //("postgres://uri-user@/db", Ok("user='uri-user' dbname='db' ")),
            //("postgres://?host=/path/to/socket/dir", Ok("host='/path/to/socket/dir' ")),
            //("postgresql://host?uzer=", Err("invalid URI query parameter: \"uzer\" ")),
            //("postgre://", Err("missing \"=\" after \"postgre://\" in connection info string ")),
            //("postgres://[::1", Err("end of string reached when looking for matching \"]\" in IPv6 host address in URI: \"postgres://[::1\" ")),
            //("postgres://[]", Err("IPv6 host address may not be empty in URI: \"postgres://[]\" ")),
            //("postgres://[::1]z", Err("unexpected character \"z\" at position 17 in URI (expected \":\" or \"/\"): \"postgres://[::1]z\" ")),
            //("postgresql://host?zzz", Err("missing key/value separator \"=\" in URI query parameter: \"zzz\" ")),
            //("postgresql://host?value1&value2", Err("missing key/value separator \"=\" in URI query parameter: \"value1\" ")),
            //("postgresql://host?key=key=value", Err("extra key/value separator \"=\" in URI query parameter: \"key\" ")),
            //("postgres://host?dbname=%XXfoo", Err("invalid percent-encoded token: \"%XXfoo\" ")),
            //("postgresql://a%00b", Err("forbidden value %00 in percent-encoded value: \"a%00b\" ")),
            //("postgresql://%zz", Err("invalid percent-encoded token: \"%zz\" ")),
            //("postgresql://%1", Err("invalid percent-encoded token: \"%1\" ")),
            //("postgresql://%", Err("invalid percent-encoded token: \"%\" ")),
            //("postgres://@host", Ok("host='host' ")),
            //("postgres://host:/", Ok("host='host' ")),
            //("postgres://:12345/", Ok("port='12345' ")),
            //("postgres://otheruser@?host=/no/such/directory", Ok("user='otheruser' host='/no/such/directory' ")),
            //("postgres://otheruser@/?host=/no/such/directory", Ok("user='otheruser' host='/no/such/directory' ")),
            //("postgres://otheruser@:12345?host=/no/such/socket/path", Ok("user='otheruser' host='/no/such/socket/path' port='12345' ")),
            //("postgres://otheruser@:12345/db?host=/path/to/socket", Ok("user='otheruser' dbname='db' host='/path/to/socket' port='12345' ")),
            //("postgres://:12345/db?host=/path/to/socket", Ok("dbname='db' host='/path/to/socket' port='12345' ")),
            //("postgres://:12345?host=/path/to/socket", Ok("host='/path/to/socket' port='12345' ")),
            //("postgres://%2Fvar%2Flib%2Fpostgresql/dbname", Ok("dbname='dbname' host='/var/lib/postgresql' ")),
        ];

        for (dsn, expected) in tests {
            let config: Result<crate::connection::Config, _> = dsn.parse();
            let actual = config
                .map(|x| x.to_string())
                .map_err(|e| e.to_string());
            let expected = expected
                .map(|x| x.to_string())
                .map_err(|e: crate::Error| e.to_string());
            assert_eq!(actual, expected);
        }
    }
}
