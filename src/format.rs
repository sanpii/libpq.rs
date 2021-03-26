#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum Format {
    Text = 0,
    Binary = 1,
}

impl From<i32> for Format {
    fn from(format: i32) -> Self {
        unsafe { std::mem::transmute(format) }
    }
}

impl From<&Format> for i32 {
    fn from(format: &Format) -> i32 {
        unsafe { std::mem::transmute(*format) }
    }
}
