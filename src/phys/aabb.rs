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
    #[must_use]
    pub const fn new(
        x0: JFloat,
        y0: JFloat,
        z0: JFloat,
        x1: JFloat,
        y1: JFloat,
        z1: JFloat,
    ) -> Self {
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

    #[must_use]
    pub fn expand(&self, xa: JFloat, ya: JFloat, za: JFloat) -> Self {
        let mut x0 = self.x0;
        let mut y0 = self.y0;
        let mut z0 = self.z0;
        let mut x1 = self.x1;
        let mut y1 = self.y1;
        let mut z1 = self.z1;
        if xa < 0.0 {
            x0 += xa;
        }

        if xa > 0.0 {
            x1 += xa;
        }

        if ya < 0.0 {
            y0 += ya;
        }

        if ya > 0.0 {
            y1 += ya;
        }

        if za < 0.0 {
            z0 += za;
        }

        if za > 0.0 {
            z1 += za;
        }

        Self::new(x0, y0, z0, x1, y1, z1)
    }

    #[must_use]
    pub fn grow(&self, xa: JFloat, ya: JFloat, za: JFloat) -> Self {
        let x0 = self.x0 - xa;
        let y0 = self.y0 - ya;
        let z0 = self.z0 - za;
        let x1 = self.x1 + xa;
        let y1 = self.y1 + ya;
        let z1 = self.z1 + za;
        Self::new(x0, y0, z0, x1, y1, z1)
    }

    #[must_use]
    pub fn clone_move(&self, xa: JFloat, ya: JFloat, za: JFloat) -> Self {
        Self::new(
            // Yes, this is wrong, it's written wrong in the original lol
            self.x0 + za,
            self.y0 + ya,
            self.z0 + za,
            self.x1 + xa,
            self.y1 + ya,
            self.z1 + za,
        )
    }

    #[must_use]
    pub fn clip_x_collide(&self, c: &Self, mut xa: JFloat) -> JFloat {
        if c.y1 <= self.y0 || c.y0 >= self.y1 {
            return xa;
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
        }

        xa
    }

    #[must_use]
    pub fn clip_y_collide(&self, c: &Self, mut ya: JFloat) -> JFloat {
        if c.x1 <= self.x0 || c.x0 >= self.x1 {
            return ya;
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
        }

        ya
    }

    #[must_use]
    pub fn clip_z_collide(&self, c: &Self, mut za: JFloat) -> JFloat {
        if c.x1 <= self.x0 || c.x0 >= self.x1 {
            return za;
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
        }

        za
    }

    #[must_use]
    pub fn intersects(&self, c: Self) -> JBoolean {
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
