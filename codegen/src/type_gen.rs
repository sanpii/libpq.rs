use marksman_escape::Escape;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::iter;
use std::str;

const PG_TYPE_DAT: &str = include_str!("pg_type.dat");
const PG_RANGE_DAT: &str = include_str!("pg_range.dat");

struct Type {
    oid: u32,
    name: String,
    ident: String,
    kind: String,
    element: u32,
    doc: String,
}

pub fn build(filename: &str) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(filename)?);
    let types = parse_types();

    make_header(&mut file)?;
    make_consts(&mut file, &types)?;
    make_impl(&mut file, &types)
}

struct DatParser<'a> {
    it: iter::Peekable<str::CharIndices<'a>>,
    s: &'a str,
}

impl<'a> DatParser<'a> {
    fn new(s: &'a str) -> DatParser<'a> {
        DatParser {
            it: s.char_indices().peekable(),
            s,
        }
    }

    fn parse_array(&mut self) -> Vec<HashMap<String, String>> {
        self.eat('[');
        let mut vec = vec![];
        while !self.try_eat(']') {
            let object = self.parse_object();
            vec.push(object);
        }
        self.eof();

        vec
    }

    fn parse_object(&mut self) -> HashMap<String, String> {
        let mut object = HashMap::new();

        self.eat('{');
        loop {
            let key = self.parse_ident();
            self.eat('=');
            self.eat('>');
            let value = self.parse_string();
            object.insert(key, value);
            if !self.try_eat(',') {
                break;
            }
        }
        self.eat('}');
        self.eat(',');

        object
    }

    fn parse_ident(&mut self) -> String {
        self.skip_ws();

        let start = match self.it.peek() {
            Some((i, _)) => *i,
            None => return "".to_string(),
        };

        loop {
            match self.it.peek() {
                Some((_, 'a'..='z')) | Some((_, '_')) => {
                    self.it.next();
                }
                Some((i, _)) => return self.s[start..*i].to_string(),
                None => return self.s[start..].to_string(),
            }
        }
    }

    fn parse_string(&mut self) -> String {
        self.skip_ws();

        let mut s = String::new();

        self.eat('\'');
        loop {
            match self.it.next() {
                Some((_, '\'')) => return s,
                Some((_, '\\')) => {
                    let (_, ch) = self.it.next().expect("unexpected eof");
                    s.push(ch);
                }
                Some((_, ch)) => s.push(ch),
                None => panic!("unexpected eof"),
            }
        }
    }

    fn eat(&mut self, target: char) {
        self.skip_ws();

        match self.it.next() {
            Some((_, ch)) if ch == target => {}
            Some((_, ch)) => panic!("expected {} but got {}", target, ch),
            None => panic!("expected {} but got eof", target),
        }
    }

    fn try_eat(&mut self, target: char) -> bool {
        if self.peek(target) {
            self.eat(target);
            true
        } else {
            false
        }
    }

    fn peek(&mut self, target: char) -> bool {
        self.skip_ws();

        match self.it.peek() {
            Some((_, ch)) if *ch == target => true,
            _ => false,
        }
    }

    fn eof(&mut self) {
        self.skip_ws();
        if let Some((_, ch)) = self.it.next() {
            panic!("expected eof but got {}", ch);
        }
    }

    fn skip_ws(&mut self) {
        loop {
            match self.it.peek() {
                Some(&(_, '#')) => self.skip_to('\n'),
                Some(&(_, '\n')) | Some(&(_, ' ')) | Some(&(_, '\t')) => {
                    self.it.next();
                }
                _ => break,
            }
        }
    }

    fn skip_to(&mut self, target: char) {
        for (_, ch) in &mut self.it {
            if ch == target {
                break;
            }
        }
    }
}

fn parse_types() -> BTreeMap<u32, Type> {
    let raw_types = DatParser::new(PG_TYPE_DAT).parse_array();
    let raw_ranges = DatParser::new(PG_RANGE_DAT).parse_array();

    let oids_by_name = raw_types
        .iter()
        .map(|m| (m["typname"].clone(), m["oid"].parse::<u32>().unwrap()))
        .collect::<HashMap<_, _>>();

    let range_elements = raw_ranges
        .iter()
        .map(|m| {
            (
                oids_by_name[&*m["rngtypid"]],
                oids_by_name[&*m["rngsubtype"]],
            )
        })
        .collect::<HashMap<_, _>>();

    let range_vector_re = Regex::new("(range|vector)$").unwrap();
    let array_re = Regex::new("^_(.*)").unwrap();

    let mut types = BTreeMap::new();

    for raw_type in raw_types {
        let oid = raw_type["oid"].parse::<u32>().unwrap();

        let name = raw_type["typname"].clone();

        let ident = range_vector_re.replace(&name, "_$1");
        let ident = array_re.replace(&ident, "${1}_array");
        let ident = ident.to_ascii_uppercase();

        let kind = raw_type["typcategory"].clone();

        // we need to be able to pull composite fields and enum variants at runtime
        if kind == "C" || kind == "E" {
            continue;
        }

        let element = match &*kind {
            "R" => range_elements[&oid],
            "A" => oids_by_name[&raw_type["typelem"]],
            _ => 0,
        };

        let doc_name = array_re.replace(&name, "$1[]").to_ascii_uppercase();
        let mut doc = doc_name.clone();
        if let Some(descr) = raw_type.get("descr") {
            write!(doc, " - {}", descr).unwrap();
        }
        let doc = Escape::new(doc.as_bytes().iter().cloned()).collect();
        let doc = String::from_utf8(doc).unwrap();

        if let Some(array_type_oid) = raw_type.get("array_type_oid") {
            let array_type_oid = array_type_oid.parse::<u32>().unwrap();

            let name = format!("_{}", name);
            let doc = format!("{}&#91;&#93;", doc_name);
            let ident = format!("{}_ARRAY", ident);

            let ty = Type {
                oid: array_type_oid,
                name,
                ident,
                kind: "A".to_string(),
                element: oid,
                doc,
            };
            types.insert(array_type_oid, ty);
        }

        let ty = Type {
            oid,
            name,
            ident,
            kind,
            element,
            doc,
        };
        types.insert(oid, ty);
    }

    types
}

fn make_header(w: &mut BufWriter<File>) -> std::io::Result<()> {
    writeln!(w, "// Autogenerated file - DO NOT EDIT")
}

fn make_impl(w: &mut BufWriter<File>, types: &BTreeMap<u32, Type>) -> std::io::Result<()> {
    impl_try_from_u32(w, types)?;
    impl_try_from_str(w, types)
}

fn impl_try_from_u32(w: &mut BufWriter<File>, types: &BTreeMap<u32, Type>) -> std::io::Result<()> {
    writeln!(
        w,
        "
impl std::convert::TryFrom<u32> for Type {{
    type Error = String;

    fn try_from(oid: u32) -> std::result::Result<Self, Self::Error> {{
        match oid {{"
    )?;

    for ty in types.values() {
        writeln!(w, "            {} => Ok({}),", ty.oid, ty.ident)?;
    }

    write!(
        w,
        r#"
            _ => Err("unknow type".to_string()),
        }}
    }}
}}"#
    )
}

fn impl_try_from_str(w: &mut BufWriter<File>, types: &BTreeMap<u32, Type>) -> std::io::Result<()> {
    writeln!(
        w,
        "
impl std::str::FromStr for Type {{
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {{
        match s {{"
    )?;

    for ty in types.values() {
        writeln!(w, "            \"{}\" => Ok({}),", ty.name, ty.ident)?;
    }

    write!(
        w,
        r#"
            _ => Err("unknow type".to_string()),
        }}
    }}
}}"#
    )
}

fn make_consts(w: &mut BufWriter<File>, types: &BTreeMap<u32, Type>) -> std::io::Result<()> {
    for ty in types.values() {
        writeln!(
            w,
            r#"
/// {descr}
pub const {ident}: Type = Type {{
    oid: {oid},
    descr: "{descr}",
    name: "{name}",
    kind: {kind},
}};"#,
            ident = ty.ident,
            oid = ty.oid,
            name = ty.name,
            kind = match ty.kind.as_str() {
                "A" => format!("Kind::Array({})", ty.element),
                "B" => "Kind::Boolean".to_string(),
                "C" => "Kind::Composite".to_string(),
                "D" => "Kind::DateTime".to_string(),
                "E" => "Kind::Enum".to_string(),
                "G" => "Kind::Geometric".to_string(),
                "I" => "Kind::Network".to_string(),
                "N" => "Kind::Numeric".to_string(),
                "P" => "Kind::Pseudo".to_string(),
                "S" => "Kind::String".to_string(),
                "R" => format!("Kind::Range({})", ty.element),
                "T" => "Kind::Timestamp".to_string(),
                "U" => "Kind::UserDefined".to_string(),
                "V" => "Kind::BitString".to_string(),
                "X" => "Kind::Unknow".to_string(),
                _ => panic!("Unknow type categorie '{}'", ty.kind),
            },
            descr = ty.doc,
        )?;
    }

    Ok(())
}
