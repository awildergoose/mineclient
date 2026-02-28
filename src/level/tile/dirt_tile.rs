use std::ops::{Deref, DerefMut};

use crate::level::tile::tile::{Tile, TileTrait};

pub struct DirtTile {
    base: Tile,
}

impl DirtTile {
    pub const TILE: Self = Self {
        base: Tile::new(3, 2),
    };
}

impl Deref for DirtTile {
    type Target = Tile;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for DirtTile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TileTrait for DirtTile {
    fn bounds(
        &self,
    ) -> (
        crate::java::JFloat,
        crate::java::JFloat,
        crate::java::JFloat,
        crate::java::JFloat,
        crate::java::JFloat,
        crate::java::JFloat,
    ) {
        self.base.bounds()
    }

    fn get_texture(&self, _face: crate::java::JInt) -> crate::java::JInt {
        self.base.tex
    }
}
