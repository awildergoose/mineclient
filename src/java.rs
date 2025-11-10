use std::time::Instant;

pub type JByte = i8;
pub type JShort = i16;
pub type JInt = i32;
pub type JLong = i64;
pub type JFloat = f32;
pub type JDouble = f64;
pub type JChar = char;
pub type JBoolean = bool;

pub fn get_nano_time() -> JLong {
    Instant::now().elapsed().as_nanos() as JLong
}
