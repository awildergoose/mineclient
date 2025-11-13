use std::ops::{Deref, DerefMut};

use crate::{
    java::JInt,
    level::tile::tile::{Tile, TileTrait},
};

pub struct DirtTile {
    base: Tile,
}

impl DirtTile {
    pub const TILE: DirtTile = DirtTile {
        base: Tile { tex: 2, id: 3 },
    };

    pub fn new(id: JInt, tex: JInt) -> Self {
        let base = Tile::new_with_id(id, tex);

        Self { base }
    }
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
    fn get_texture(&self, _face: crate::java::JInt) -> crate::java::JInt {
        self.base.tex
    }
}
