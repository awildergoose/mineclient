use crate::java::{JBoolean, JFloat};

#[derive(Clone)]
pub struct AABB {
    epsilon: JFloat,
    pub x0: JFloat,
    pub y0: JFloat,
    pub z0: JFloat,
    pub x1: JFloat,
    pub y1: JFloat,
    pub z1: JFloat,
}

impl AABB {
    pub fn new(x0: JFloat, y0: JFloat, z0: JFloat, x1: JFloat, y1: JFloat, z1: JFloat) -> Self {
        Self {
            epsilon: 0.0,
            x0,
            y0,
            z0,
            x1,
            y1,
            z1,
        }
    }

    pub fn expand(&self, xa: JFloat, ya: JFloat, za: JFloat) -> Self {
        let mut _x0 = self.x0;
        let mut _y0 = self.y0;
        let mut _z0 = self.z0;
        let mut _x1 = self.x1;
        let mut _y1 = self.y1;
        let mut _z1 = self.z1;
        if xa < 0.0 {
            _x0 += xa;
        }

        if xa > 0.0 {
            _x1 += xa;
        }

        if ya < 0.0 {
            _y0 += ya;
        }

        if ya > 0.0 {
            _y1 += ya;
        }

        if za < 0.0 {
            _z0 += za;
        }

        if za > 0.0 {
            _z1 += za;
        }

        Self::new(_x0, _y0, _z0, _x1, _y1, _z1)
    }

    pub fn grow(&self, xa: JFloat, ya: JFloat, za: JFloat) -> Self {
        let _x0 = self.x0 - xa;
        let _y0 = self.y0 - ya;
        let _z0 = self.z0 - za;
        let _x1 = self.x1 + xa;
        let _y1 = self.y1 + ya;
        let _z1 = self.z1 + za;
        Self::new(_x0, _y0, _z0, _x1, _y1, _z1)
    }

    pub fn clip_x_collide(&self, c: &AABB, mut xa: JFloat) -> JFloat {
        if c.y1 <= self.y0 || c.y0 >= self.y1 {
            xa
        } else if c.z1 > self.z0 && c.z0 < self.z1 {
            if xa > 0.0 && c.x1 <= self.x0 {
                let max = self.x0 - c.x1 - self.epsilon;
                if max < xa {
                    xa = max;
                }
            }

            if xa < 0.0 && c.x0 >= self.x1 {
                let max = self.x1 - c.x0 + self.epsilon;
                if max > xa {
                    xa = max;
                }
            }

            xa
        } else {
            xa
        }
    }

    pub fn clip_y_collide(&self, c: &AABB, mut ya: JFloat) -> JFloat {
        if c.x1 <= self.x0 || c.x0 >= self.x1 {
            ya
        } else if !(c.z1 <= self.z0) && !(c.z0 >= self.z1) {
            if ya > 0.0 && c.y1 <= self.y0 {
                let max = self.y0 - c.y1 - self.epsilon;
                if max < ya {
                    ya = max;
                }
            }

            if ya < 0.0 && c.y0 >= self.y1 {
                let max = self.y1 - c.y0 + self.epsilon;
                if max > ya {
                    ya = max;
                }
            }

            ya
        } else {
            ya
        }
    }

    pub fn clip_z_collide(&self, c: &AABB, mut za: JFloat) -> JFloat {
        if c.x1 <= self.x0 || c.x0 >= self.x1 {
            za
        } else if !(c.y1 <= self.y0) && !(c.y0 >= self.y1) {
            if za > 0.0 && c.z1 <= self.z0 {
                let max = self.z0 - c.z1 - self.epsilon;
                if max < za {
                    za = max;
                }
            }

            if za < 0.0 && c.z0 >= self.z1 {
                let max = self.z1 - c.z0 + self.epsilon;
                if max > za {
                    za = max;
                }
            }

            za
        } else {
            za
        }
    }

    pub fn intersects(&self, c: AABB) -> JBoolean {
        if c.x1 <= self.x0 || c.x0 >= self.x1 {
            return false;
        }
        if c.y1 <= self.y0 || c.y0 >= self.y1 {
            return false;
        }
        c.z1 > self.z0 && c.z0 < self.z1
    }

    pub fn move_(&mut self, xa: JFloat, ya: JFloat, za: JFloat) {
        self.x0 += xa;
        self.y0 += ya;
        self.z0 += za;
        self.x1 += xa;
        self.y1 += ya;
        self.z1 += za;
    }
}
