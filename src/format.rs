#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum Format {
    Text = 0,
    Binary = 1,
}

impl From<i32> for Format {
    fn from(format: i32) -> Self {
        match format {
            0 => Self::Text,
            1 => Self::Binary,
            _ => unreachable!(),
        }
    }
}

impl From<&Format> for i32 {
    fn from(format: &Format) -> i32 {
        *format as i32
    }
}
