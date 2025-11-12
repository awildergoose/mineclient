use std::sync::{Mutex, OnceLock};

use crate::{
    java::{JBoolean, JInt},
    level::{level::Level, tesselator::Tesselator},
    phys::aabb::AABB,
};

pub struct Tile {
    tex: JInt,
    id: JInt,
}

static TILES: OnceLock<Mutex<Vec<Tile>>> = OnceLock::new();

pub fn get_tiles() -> &'static Mutex<Vec<Tile>> {
    TILES.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn init_tiles() {
    let mut tiles = get_tiles().lock().unwrap();
    tiles.push(Tile::ROCK);
    tiles.push(Tile::GRASS);
}

impl Tile {
    pub const ROCK: Tile = Tile { id: 1, tex: 1 };
    pub const GRASS: Tile = Tile { id: 2, tex: 2 };
    pub const DIRT: Tile = Tile { id: 3, tex: 2 };
    pub const STONE_BRICK: Tile = Tile { id: 4, tex: 16 };
    pub const WOOD: Tile = Tile { id: 5, tex: 4 };
    // TODO use Bush type
    pub const BUSH: Tile = Tile { id: 6, tex: 6 };

    pub fn new(id: JInt) -> Self {
        Self { tex: 0, id }
    }

    pub fn new_with_id(id: JInt, tex: JInt) -> Self {
        Self { tex, id }
    }

    pub fn render(
        &self,
        t: &mut Tesselator,
        level: &Level,
        layer: JInt,
        x: JInt,
        y: JInt,
        z: JInt,
    ) {
        let c1 = 1.0;
        let c2 = 0.8;
        let c3 = 0.6;

        if self.should_render_face(level, x, y - 1, z, layer) {
            t.color(c1, c1, c1);
            self.render_face(t, x, y, z, 0);
        }

        if self.should_render_face(level, x, y + 1, z, layer) {
            t.color(c1, c1, c1);
            self.render_face(t, x, y, z, 1);
        }

        if self.should_render_face(level, x, y, z - 1, layer) {
            t.color(c2, c2, c2);
            self.render_face(t, x, y, z, 2);
        }

        if self.should_render_face(level, x, y, z + 1, layer) {
            t.color(c2, c2, c2);
            self.render_face(t, x, y, z, 3);
        }

        if self.should_render_face(level, x - 1, y, z, layer) {
            t.color(c3, c3, c3);
            self.render_face(t, x, y, z, 4);
        }

        if self.should_render_face(level, x + 1, y, z, layer) {
            t.color(c3, c3, c3);
            self.render_face(t, x, y, z, 5);
        }
    }

    pub fn should_render_face(
        &self,
        level: &Level,
        x: JInt,
        y: JInt,
        z: JInt,
        layer: JInt,
    ) -> JBoolean {
        (!level.is_solid_tile(x, y, z) && level.is_lit(x, y, z)) ^ (layer == 1)
    }

    pub fn get_texture(&self, _face: JInt) -> JInt {
        self.tex
    }

    pub fn render_face(&self, t: &mut Tesselator, x: JInt, y: JInt, z: JInt, face: JInt) {
        let tex = self.get_texture(face);
        let u0 = ((tex % 16) / 16) as f32; // TODO verify this cast
        let u1 = u0 + 0.0624375;
        let v0 = tex as f32 / 16.0 / 16.0;
        let v1 = v0 + 0.0624375;
        let x0 = x as f32;
        let x1 = x as f32 + 1.0;
        let y0 = y as f32;
        let y1 = y as f32 + 1.0;
        let z0 = z as f32;
        let z1 = z as f32 + 1.0;

        if face == 0 {
            t.vertex_uv(x0, y0, z1, u0, v1);
            t.vertex_uv(x0, y0, z0, u0, v0);
            t.vertex_uv(x1, y0, z0, u1, v0);
            t.vertex_uv(x1, y0, z1, u1, v1);
        }

        if face == 1 {
            t.vertex_uv(x1, y1, z1, u1, v1);
            t.vertex_uv(x1, y1, z0, u1, v0);
            t.vertex_uv(x0, y1, z0, u0, v0);
            t.vertex_uv(x0, y1, z1, u0, v1);
        }

        if face == 2 {
            t.vertex_uv(x0, y1, z0, u1, v0);
            t.vertex_uv(x1, y1, z0, u0, v0);
            t.vertex_uv(x1, y0, z0, u0, v1);
            t.vertex_uv(x0, y0, z0, u1, v1);
        }

        if face == 3 {
            t.vertex_uv(x0, y1, z1, u0, v0);
            t.vertex_uv(x0, y0, z1, u0, v1);
            t.vertex_uv(x1, y0, z1, u1, v1);
            t.vertex_uv(x1, y1, z1, u1, v0);
        }

        if face == 4 {
            t.vertex_uv(x0, y1, z1, u1, v0);
            t.vertex_uv(x0, y1, z0, u0, v0);
            t.vertex_uv(x0, y0, z0, u0, v1);
            t.vertex_uv(x0, y0, z1, u1, v1);
        }

        if face == 5 {
            t.vertex_uv(x1, y0, z1, u0, v1);
            t.vertex_uv(x1, y0, z0, u1, v1);
            t.vertex_uv(x1, y1, z0, u1, v0);
            t.vertex_uv(x1, y1, z1, u0, v0);
        }
    }

    pub fn render_face_no_texture(
        &self,
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

        if face == 0 {
            t.vertex(x0, y0, z1);
            t.vertex(x0, y0, z0);
            t.vertex(x1, y0, z0);
            t.vertex(x1, y0, z1);
        }

        if face == 1 {
            t.vertex(x1, y1, z1);
            t.vertex(x1, y1, z0);
            t.vertex(x0, y1, z0);
            t.vertex(x0, y1, z1);
        }

        if face == 2 {
            t.vertex(x0, y1, z0);
            t.vertex(x1, y1, z0);
            t.vertex(x1, y0, z0);
            t.vertex(x0, y0, z0);
        }

        if face == 3 {
            t.vertex(x0, y1, z1);
            t.vertex(x0, y0, z1);
            t.vertex(x1, y0, z1);
            t.vertex(x1, y1, z1);
        }

        if face == 4 {
            t.vertex(x0, y1, z1);
            t.vertex(x0, y1, z0);
            t.vertex(x0, y0, z0);
            t.vertex(x0, y0, z1);
        }

        if face == 5 {
            t.vertex(x1, y0, z1);
            t.vertex(x1, y0, z0);
            t.vertex(x1, y1, z0);
            t.vertex(x1, y1, z1);
        }
    }

    pub fn get_tile_aabb(&self, x: JInt, y: JInt, z: JInt) -> AABB {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        AABB::new(x, y, z, x + 1.0, y + 1.0, z + 1.0)
    }

    pub fn get_aabb(&self, x: JInt, y: JInt, z: JInt) -> AABB {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        AABB::new(x, y, z, x + 1.0, y + 1.0, z + 1.0)
    }

    pub fn blocks_light(&self) -> JBoolean {
        true
    }

    pub fn is_solid(&self) -> JBoolean {
        true
    }
}
