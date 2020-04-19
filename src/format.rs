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

impl Into<i32> for &Format {
    fn into(self) -> i32 {
        unsafe { std::mem::transmute(*self) }
    }
}
