use std::{cell::RefCell, rc::Rc};

use javarandom::JavaRandom;

use crate::{
    java::{JBoolean, JFloat, JInt},
    level::{
        level::Level,
        tile::{bush_tile::BushTile, dirt_tile::DirtTile, grass_tile::GrassTile},
    },
    particle::{particle::Particle, particle_engine::ParticleEngine},
    phys::aabb::AABB,
    player::Player,
    renderer::tesselator::Tesselator,
};

#[derive(Debug)]
pub struct Tile {
    pub tex: JInt,
    pub id: JInt,
    pub xx0: JFloat,
    pub yy0: JFloat,
    pub zz0: JFloat,
    pub xx1: JFloat,
    pub yy1: JFloat,
    pub zz1: JFloat,
    pub ticking: JBoolean,
}

#[must_use]
pub fn get_tile(id: i32) -> Option<&'static dyn TileTrait> {
    match id {
        x if x == Tile::ROCK.id => Some(&Tile::ROCK),
        x if x == Tile::GRASS.id => Some(&Tile::GRASS),
        x if x == Tile::DIRT.id => Some(&Tile::DIRT),
        x if x == Tile::STONE_BRICK.id => Some(&Tile::STONE_BRICK),
        x if x == Tile::WOOD.id => Some(&Tile::WOOD),
        x if x == Tile::BUSH.id => Some(&Tile::BUSH),
        _ => None,
    }
}

pub trait TileTrait: Send + Sync {
    fn base(&self) -> &Tile;

    fn get_texture(&self, face: JInt) -> JInt {
        let _ = face;
        self.base().tex
    }

    fn blocks_light(&self) -> JBoolean {
        true
    }

    fn is_solid(&self) -> JBoolean {
        true
    }

    fn tick(&self, level: &mut Level, x: JInt, y: JInt, z: JInt, random: Rc<RefCell<JavaRandom>>) {
        let _ = level;
        let _ = x;
        let _ = y;
        let _ = z;
        let _ = random;
    }

    fn destroy(
        &self,
        level: Rc<RefCell<Level>>,
        x: JInt,
        y: JInt,
        z: JInt,
        particle_engine: &mut ParticleEngine,
    ) {
        let sd = 4;

        for xx in 0..sd {
            for yy in 0..sd {
                for zz in 0..sd {
                    let xp = x as f32 + (xx as f32 + 0.5) / sd as f32;
                    let yp = y as f32 + (yy as f32 + 0.5) / sd as f32;
                    let zp = z as f32 + (zz as f32 + 0.5) / sd as f32;
                    particle_engine.add(Particle::new(
                        level.clone(),
                        xp,
                        yp,
                        zp,
                        xp - x as f32 - 0.5,
                        yp - y as f32 - 0.5,
                        zp - z as f32 - 0.5,
                        self.get_texture(0),
                    ));
                }
            }
        }
    }

    fn get_tile_aabb(&self, x: JInt, y: JInt, z: JInt) -> AABB {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        AABB::new(x, y, z, x + 1.0, y + 1.0, z + 1.0)
    }

    fn get_aabb(&self, x: JInt, y: JInt, z: JInt) -> Option<AABB> {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        Some(AABB::new(x, y, z, x + 1.0, y + 1.0, z + 1.0))
    }

    fn render(&self, t: &mut Tesselator, level: &Level, layer: JInt, x: JInt, y: JInt, z: JInt) {
        let c1 = -1;
        let c2 = -52;
        let c3 = -103;

        if self.should_render_face(level, x, y - 1, z, layer, 0) {
            t.color(c1, c1, c1);
            self.render_face(t, x, y, z, 0);
        }

        if self.should_render_face(level, x, y + 1, z, layer, 1) {
            t.color(c1, c1, c1);
            self.render_face(t, x, y, z, 1);
        }

        if self.should_render_face(level, x, y, z - 1, layer, 2) {
            t.color(c2, c2, c2);
            self.render_face(t, x, y, z, 2);
        }

        if self.should_render_face(level, x, y, z + 1, layer, 3) {
            t.color(c2, c2, c2);
            self.render_face(t, x, y, z, 3);
        }

        if self.should_render_face(level, x - 1, y, z, layer, 4) {
            t.color(c3, c3, c3);
            self.render_face(t, x, y, z, 4);
        }

        if self.should_render_face(level, x + 1, y, z, layer, 5) {
            t.color(c3, c3, c3);
            self.render_face(t, x, y, z, 5);
        }
    }

    fn should_render_face(
        &self,
        level: &Level,
        x: JInt,
        y: JInt,
        z: JInt,
        layer: JInt,
        face: JInt,
    ) -> JBoolean {
        let _ = face;
        let mut layer_ok = true;

        if layer == 2 {
            return false;
        }

        if layer >= 0 {
            layer_ok = level.is_lit(x, y, z) ^ (layer == 1);
        }

        !level.is_solid_tile(x, y, z) && layer_ok
    }

    fn render_face(&self, t: &mut Tesselator, x: JInt, y: JInt, z: JInt, face: JInt) {
        let tex = self.get_texture(face);
        let base = self.base();
        let xt = (tex % 16) as f32 * 16.0;
        let u0 = xt / 256.0;
        let yt = (tex / 16) as f32 * 16.0;
        let u1 = (xt + 15.99) / 256.0;
        let v0 = yt / 256.0;
        let v1 = (yt + 15.99) / 256.0;
        let x0 = x as f32 + base.xx0;
        let x1 = x as f32 + base.xx1;
        let y0 = y as f32 + base.yy0;
        let y1 = y as f32 + base.yy1;
        let z0 = z as f32 + base.zz0;
        let z1 = z as f32 + base.zz1;

        if face == 0 {
            t.vertex_uv(x0, y0, z1, u0, v1);
            t.vertex_uv(x0, y0, z0, u0, v0);
            t.vertex_uv(x1, y0, z0, u1, v0);
            t.vertex_uv(x1, y0, z1, u1, v1);
        } else if face == 1 {
            t.vertex_uv(x1, y1, z1, u1, v1);
            t.vertex_uv(x1, y1, z0, u1, v0);
            t.vertex_uv(x0, y1, z0, u0, v0);
            t.vertex_uv(x0, y1, z1, u0, v1);
        } else if face == 2 {
            t.vertex_uv(x0, y1, z0, u1, v0);
            t.vertex_uv(x1, y1, z0, u0, v0);
            t.vertex_uv(x1, y0, z0, u0, v1);
            t.vertex_uv(x0, y0, z0, u1, v1);
        } else if face == 3 {
            t.vertex_uv(x0, y1, z1, u0, v0);
            t.vertex_uv(x0, y0, z1, u0, v1);
            t.vertex_uv(x1, y0, z1, u1, v1);
            t.vertex_uv(x1, y1, z1, u1, v0);
        } else if face == 4 {
            t.vertex_uv(x0, y1, z1, u1, v0);
            t.vertex_uv(x0, y1, z0, u0, v0);
            t.vertex_uv(x0, y0, z0, u0, v1);
            t.vertex_uv(x0, y0, z1, u1, v1);
        } else if face == 5 {
            t.vertex_uv(x1, y0, z1, u0, v1);
            t.vertex_uv(x1, y0, z0, u1, v1);
            t.vertex_uv(x1, y1, z0, u1, v0);
            t.vertex_uv(x1, y1, z1, u0, v0);
        }
    }

    fn render_backface(&self, t: &mut Tesselator, x: JInt, y: JInt, z: JInt, face: JInt) {
        let tex = self.get_texture(face);
        let base = self.base();
        let u0 = (tex % 16) as f32 / 16.0;
        let u1 = u0 + 0.062_437_5;
        let v0 = (tex / 16) as f32 / 16.0;
        let v1 = v0 + 0.062_437_5;
        let x0 = x as f32 + base.xx0;
        let x1 = x as f32 + base.xx1;
        let y0 = y as f32 + base.yy0;
        let y1 = y as f32 + base.yy1;
        let z0 = z as f32 + base.zz0;
        let z1 = z as f32 + base.zz1;

        if face == 0 {
            t.vertex_uv(x1, y0, z1, u1, v1);
            t.vertex_uv(x1, y0, z0, u1, v0);
            t.vertex_uv(x0, y0, z0, u0, v0);
            t.vertex_uv(x0, y0, z1, u0, v1);
        }

        if face == 1 {
            t.vertex_uv(x0, y1, z1, u0, v1);
            t.vertex_uv(x0, y1, z0, u0, v0);
            t.vertex_uv(x1, y1, z0, u1, v0);
            t.vertex_uv(x1, y1, z1, u1, v1);
        }

        if face == 2 {
            t.vertex_uv(x0, y0, z0, u1, v1);
            t.vertex_uv(x1, y0, z0, u0, v1);
            t.vertex_uv(x1, y1, z0, u0, v0);
            t.vertex_uv(x0, y1, z0, u1, v0);
        }

        if face == 3 {
            t.vertex_uv(x1, y1, z1, u1, v0);
            t.vertex_uv(x1, y0, z1, u1, v1);
            t.vertex_uv(x0, y0, z1, u0, v1);
            t.vertex_uv(x0, y1, z1, u0, v0);
        }

        if face == 4 {
            t.vertex_uv(x0, y0, z1, u1, v1);
            t.vertex_uv(x0, y0, z0, u0, v1);
            t.vertex_uv(x0, y1, z0, u0, v0);
            t.vertex_uv(x0, y1, z1, u1, v0);
        }

        if face == 5 {
            t.vertex_uv(x1, y1, z1, u0, v0);
            t.vertex_uv(x1, y1, z0, u1, v0);
            t.vertex_uv(x1, y0, z0, u1, v1);
            t.vertex_uv(x1, y0, z1, u0, v1);
        }
    }

    fn render_face_no_texture(
        &self,
        player: &Player,
        t: &mut Tesselator,
        x: JInt,
        y: JInt,
        z: JInt,
        face: JInt,
    ) {
        let x0 = x as f32 + 0.0;
        let x1 = x as f32 + 1.0;
        let y0 = y as f32 + 0.0;
        let y1 = y as f32 + 1.0;
        let z0 = z as f32 + 0.0;
        let z1 = z as f32 + 1.0;

        if face == 0 && (y as f32) > player.y {
            t.vertex(x0, y0, z1);
            t.vertex(x0, y0, z0);
            t.vertex(x1, y0, z0);
            t.vertex(x1, y0, z1);
        }

        if face == 1 && (y as f32) < player.y {
            t.vertex(x1, y1, z1);
            t.vertex(x1, y1, z0);
            t.vertex(x0, y1, z0);
            t.vertex(x0, y1, z1);
        }

        if face == 2 && (z as f32) > player.z {
            t.vertex(x0, y1, z0);
            t.vertex(x1, y1, z0);
            t.vertex(x1, y0, z0);
            t.vertex(x0, y0, z0);
        }

        if face == 3 && (z as f32) < player.z {
            t.vertex(x0, y1, z1);
            t.vertex(x0, y0, z1);
            t.vertex(x1, y0, z1);
            t.vertex(x1, y1, z1);
        }

        if face == 4 && (x as f32) > player.x {
            t.vertex(x0, y1, z1);
            t.vertex(x0, y1, z0);
            t.vertex(x0, y0, z0);
            t.vertex(x0, y0, z1);
        }

        if face == 5 && (x as f32) < player.x {
            t.vertex(x1, y0, z1);
            t.vertex(x1, y0, z0);
            t.vertex(x1, y1, z0);
            t.vertex(x1, y1, z1);
        }
    }

    fn may_pick(&self) -> JBoolean {
        true
    }

    fn get_liquid_type(&self) -> JInt {
        Tile::LIQUID_NOT
    }

    fn neighbor_changed(&self, level: &Level, x: JInt, y: JInt, z: JInt, type_: JInt) {
        let _ = level;
        let _ = x;
        let _ = y;
        let _ = z;
        let _ = type_;
    }
}

impl Tile {
    pub const LIQUID_NOT: JInt = 0;
    pub const LIQUID_WATER: JInt = 1;
    pub const LIQUID_LAVA: JInt = 2;

    pub const ROCK: Self = Self::new(1, 1);
    pub const GRASS: GrassTile = GrassTile::TILE;
    pub const DIRT: DirtTile = DirtTile::TILE;
    pub const STONE_BRICK: Self = Self::new(4, 16);
    pub const WOOD: Self = Self::new(5, 4);
    pub const BUSH: BushTile = BushTile::TILE;
    pub const UNBREAKABLE: Self = Self::new(7, 17);

    #[must_use]
    pub const fn default() -> Self {
        Self {
            tex: 0,
            id: 0,
            xx0: 0.0,
            yy0: 0.0,
            zz0: 0.0,
            xx1: 1.0,
            yy1: 1.0,
            zz1: 1.0,
            ticking: false,
        }
    }

    #[must_use]
    pub const fn new(id: JInt, tex: JInt) -> Self {
        Self {
            tex,
            id,
            ..Self::default()
        }
    }

    #[must_use]
    pub const fn new_ticking(id: JInt, tex: JInt) -> Self {
        Self {
            id,
            tex,
            ticking: true,
            ..Self::default()
        }
    }

    pub const fn set_ticking(&mut self, tick: JBoolean) {
        self.ticking = tick;
    }

    pub const fn set_bounds(
        &mut self,
        x0: JFloat,
        y0: JFloat,
        z0: JFloat,
        x1: JFloat,
        y1: JFloat,
        z1: JFloat,
    ) {
        self.xx0 = x0;
        self.yy0 = y0;
        self.zz0 = z0;
        self.xx1 = x1;
        self.yy1 = y1;
        self.zz1 = z1;
    }
}

impl TileTrait for Tile {
    fn base(&self) -> &Tile {
        self
    }
}
