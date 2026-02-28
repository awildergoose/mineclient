use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use javarandom::JavaRandom;

use crate::level::tile::tile::{Tile, TileTrait};

pub struct GrassTile {
    base: Tile,
}

impl GrassTile {
    pub const TILE: Self = Self {
        base: Tile::new_ticking(2, 3),
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
    fn base(&self) -> &Tile {
        &self.base
    }

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
        random: Rc<RefCell<JavaRandom>>,
    ) {
        let mut random = random.borrow_mut();
        if random.next_int_with_bound(4) == 0 {
            if level.is_lit(x, y + 1, z) {
                for _ in 0..4 {
                    let xt = x + random.next_int_with_bound(3) - 1;
                    let yt = y + random.next_int_with_bound(5) - 3;
                    let zt = z + random.next_int_with_bound(3) - 1;

                    if level.get_tile(xt, yt, zt) == Tile::DIRT.id && level.is_lit(xt, yt + 1, zt) {
                        level.set_tile(xt, yt, zt, Tile::GRASS.id);
                    }
                }
            } else {
                level.set_tile(x, y, z, Tile::DIRT.id);
            }
        }
    }
}
