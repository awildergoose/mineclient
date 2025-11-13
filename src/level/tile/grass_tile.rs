use std::ops::{Deref, DerefMut};

use crate::level::tile::tile::{Tile, TileTrait};

pub struct GrassTile {
    base: Tile,
}

impl GrassTile {
    pub const TILE: GrassTile = GrassTile {
        base: Tile { tex: 3, id: 2 },
    };
}

impl Deref for GrassTile {
    type Target = Tile;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for GrassTile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TileTrait for GrassTile {
    fn get_texture(&self, face: crate::java::JInt) -> crate::java::JInt {
        if face == 1 {
            0
        } else if face == 0 {
            2
        } else {
            3
        }
    }

    fn tick(
        &self,
        level: &mut crate::level::level::Level,
        x: crate::java::JInt,
        y: crate::java::JInt,
        z: crate::java::JInt,
    ) {
        if !level.is_lit(x, y, z) {
            level.set_tile(x, y, z, Tile::DIRT.id);
        } else {
            for _ in 0..4 {
                let (xt, yt, zt) = {
                    let mut random = level.random.borrow_mut();
                    let xt = x + random.next_int_with_bound(3) - 1;
                    let yt = y + random.next_int_with_bound(5) - 3;
                    let zt = z + random.next_int_with_bound(3) - 1;
                    (xt, yt, zt)
                };

                if level.get_tile(xt, yt, zt) == Tile::DIRT.id && level.is_lit(xt, yt, zt) {
                    level.set_tile(xt, yt, zt, Tile::GRASS.id);
                }
            }
        }
    }
}
