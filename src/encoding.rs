// @see https://github.com/postgres/postgres/blob/REL_12_2/src/include/mb/pg_wchar.h#L238
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Encoding {
    SQL_ASCII = 0,
    EUC_JP,
    EUC_CN,
    EUC_KR,
    EUC_TW,
    EUC_JIS_2004,
    UTF8,
    MULE_INTERNAL,
    LATIN1,
    LATIN2,
    LATIN3,
    LATIN4,
    LATIN5,
    LATIN6,
    LATIN7,
    LATIN8,
    LATIN9,
    LATIN10,
    WIN1256,
    WIN1258,
    WIN866,
    WIN874,
    KOI8R,
    WIN1251,
    WIN1252,
    ISO_8859_5,
    ISO_8859_6,
    ISO_8859_7,
    ISO_8859_8,
    WIN1250,
    WIN1253,
    WIN1254,
    WIN1255,
    WIN1257,
    KOI8U,
    SJIS,
    BIG5,
    GBK,
    UHC,
    GB18030,
    JOHAB,
    SHIFT_JIS_2004,
}

impl From<i32> for Encoding {
    fn from(encoding: i32) -> Self {
        unsafe { std::mem::transmute(encoding) }
    }
}

impl std::fmt::Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
