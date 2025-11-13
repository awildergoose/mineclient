use std::ops::{Deref, DerefMut};

use crate::level::tile::tile::{Tile, TileTrait};

pub struct BushTile {
    base: Tile,
}

impl BushTile {
    pub const TILE: BushTile = BushTile {
        base: Tile { tex: 6, id: 6 },
    };
}

impl Deref for BushTile {
    type Target = Tile;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BushTile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TileTrait for BushTile {
    fn get_texture(&self, _face: crate::java::JInt) -> crate::java::JInt {
        self.base.tex
    }
}
