use std::{
    f32::consts::PI,
    ops::{Deref, DerefMut},
};

use crate::level::tile::tile::{Tile, TileTrait};

pub struct BushTile {
    base: Tile,
}

impl BushTile {
    pub const TILE: BushTile = BushTile {
        base: Tile { tex: 15, id: 6 },
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

    fn tick(
        &self,
        level: &mut crate::level::level::Level,
        x: crate::java::JInt,
        y: crate::java::JInt,
        z: crate::java::JInt,
    ) {
        let below = level.get_tile(x, y - 1, z);
        if !level.is_lit(x, y, z) || below != Tile::DIRT.id && below != Tile::GRASS.id {
            level.set_tile(x, y, z, 0);
        }
    }

    fn render(
        &self,
        t: &mut crate::level::tesselator::Tesselator,
        level: &crate::level::level::Level,
        layer: crate::java::JInt,
        x: crate::java::JInt,
        y: crate::java::JInt,
        z: crate::java::JInt,
    ) {
        // TODO is this right? should we put ! before or after?
        if (!level.is_lit(x, y, z)) ^ (layer != 1) {
            let tex = self.get_texture(15);
            let u0 = (tex % 16) as f32 / 16.0;
            let u1 = u0 + 0.0624375;
            let v0 = (tex / 16) as f32 / 16.0;
            let v1 = v0 + 0.0624375;
            let rots = 2;
            t.color(1.0, 1.0, 1.0);

            for r in 0..rots {
                let xa = f32::sin(r as f32 * PI / rots as f32 + (PI / 4.0)) * 0.5;
                let za = f32::cos(r as f32 * PI / rots as f32 + (PI / 4.0)) * 0.5;
                let x0 = x as f32 + 0.5 - xa;
                let x1 = x as f32 + 0.5 + xa;
                let y0 = y as f32 + 0.0;
                let y1 = y as f32 + 1.0;
                let z0 = z as f32 + 0.5 - za;
                let z1 = z as f32 + 0.5 + za;
                t.vertex_uv(x0, y1, z0, u1, v0);
                t.vertex_uv(x1, y1, z1, u0, v0);
                t.vertex_uv(x1, y0, z1, u0, v1);
                t.vertex_uv(x0, y0, z0, u1, v1);
                t.vertex_uv(x1, y1, z1, u0, v0);
                t.vertex_uv(x0, y1, z0, u1, v0);
                t.vertex_uv(x0, y0, z0, u1, v1);
                t.vertex_uv(x1, y0, z1, u0, v1);
            }
        }
    }

    fn get_aabb(
        &self,
        _x: crate::java::JInt,
        _y: crate::java::JInt,
        _z: crate::java::JInt,
    ) -> Option<crate::phys::aabb::AABB> {
        None
    }

    fn blocks_light(&self) -> crate::java::JBoolean {
        false
    }

    fn is_solid(&self) -> crate::java::JBoolean {
        false
    }
}
