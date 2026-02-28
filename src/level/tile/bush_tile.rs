use std::{
    cell::RefCell,
    f32::consts::PI,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use javarandom::JavaRandom;

use crate::level::tile::tile::{Tile, TileTrait};

pub struct BushTile {
    base: Tile,
}

impl BushTile {
    pub const TILE: Self = Self {
        base: Tile::new_ticking(6, 15),
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
    fn base(&self) -> &Tile {
        &self.base
    }

    fn tick(
        &self,
        level: &mut crate::level::level::Level,
        x: crate::java::JInt,
        y: crate::java::JInt,
        z: crate::java::JInt,
        _random: Rc<RefCell<JavaRandom>>,
    ) {
        let below = level.get_tile(x, y - 1, z);
        if !level.is_lit(x, y, z) || below != Tile::DIRT.id && below != Tile::GRASS.id {
            level.set_tile(x, y, z, 0);
        }
    }

    fn render(
        &self,
        t: &mut crate::renderer::tesselator::Tesselator,
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
            let u1 = u0 + 0.062_437_5;
            let v0 = (tex / 16) as f32 / 16.0;
            let v1 = v0 + 0.062_437_5;
            let rots = 2;
            t.colori3(255, 255, 255);

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
                t.vertex_uv(x1, y1, z1, u1, v0);
                t.vertex_uv(x0, y1, z0, u0, v0);
                t.vertex_uv(x0, y0, z0, u0, v1);
                t.vertex_uv(x1, y0, z1, u1, v1);
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
