pub struct HitResult {
    pub type_: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub f: i32,
}

impl HitResult {
    #[must_use]
    pub const fn new(type_: i32, x: i32, y: i32, z: i32, f: i32) -> Self {
        Self { type_, x, y, z, f }
    }
}
