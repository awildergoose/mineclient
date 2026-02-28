use crate::java::JInt;

pub struct Coord {
    pub x: JInt,
    pub y: JInt,
    pub z: JInt,
}

impl Coord {
    #[must_use]
    pub const fn new(x: JInt, y: JInt, z: JInt) -> Self {
        Self { x, y, z }
    }
}
