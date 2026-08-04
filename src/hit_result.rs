use crate::{
    java::{JBoolean, JFloat, JInt},
    player::Player,
};

pub struct HitResult {
    pub type_: JInt,
    pub x: JInt,
    pub y: JInt,
    pub z: JInt,
    pub f: JInt,
}

impl HitResult {
    #[must_use]
    pub const fn new(type_: JInt, x: JInt, y: JInt, z: JInt, f: JInt) -> Self {
        Self { type_, x, y, z, f }
    }

    #[must_use]
    pub fn is_closer_than(&self, player: &Player, o: &Self, edit_mode: JInt) -> JBoolean {
        let mut dist = self.distance_to(player, 0);
        let mut dist2 = o.distance_to(player, 0);

        if dist < dist2 {
            return true;
        }

        dist = self.distance_to(player, edit_mode);
        dist2 = o.distance_to(player, edit_mode);
        dist < dist2
    }

    fn distance_to(&self, player: &Player, edit_mode: JInt) -> JFloat {
        let mut xx = self.x;
        let mut yy = self.y;
        let mut zz = self.z;

        if edit_mode == 1 {
            if self.f == 0 {
                yy -= 1;
            }

            if self.f == 1 {
                yy += 1;
            }

            if self.f == 2 {
                zz -= 1;
            }

            if self.f == 3 {
                zz += 1;
            }

            if self.f == 4 {
                xx -= 1;
            }

            if self.f == 5 {
                xx += 1;
            }
        }

        let xd = xx as f32 - player.x;
        let yd = yy as f32 - player.y;
        let zd = zz as f32 - player.z;

        zd.mul_add(zd, xd.mul_add(xd, yd * yd))
    }
}
