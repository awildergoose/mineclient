use crate::java::JFloat;

#[derive(Clone)]
pub struct Vec3 {
    pub x: JFloat,
    pub y: JFloat,
    pub z: JFloat,
}

impl Vec3 {
    #[must_use]
    pub const fn new(x: JFloat, y: JFloat, z: JFloat) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub const fn interpolate_to(&self, t: Self, p: JFloat) -> Self {
        let xt = self.x + (t.x - self.x) * p;
        let yt = self.y + (t.y - self.y) * p;
        let zt = self.z + (t.z - self.z) * p;
        Self::new(xt, yt, zt)
    }

    pub const fn set(&mut self, x: JFloat, y: JFloat, z: JFloat) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}
