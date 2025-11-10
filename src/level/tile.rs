use crate::{
    java::JInt,
    level::{level::Level, tesselator::Tesselator},
};

pub struct Tile {
    tex: JInt,
}

impl Tile {
    pub const ROCK: Tile = Tile { tex: 0 };
    pub const GRASS: Tile = Tile { tex: 1 };

    pub fn new(tex: JInt) -> Self {
        Self { tex }
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
        let u0 = self.tex as f32 / 16.0;
        let u1 = u0 + 0.0624375;
        let v0 = 0.0_f32;
        let v1 = v0 + 0.0624375;
        let c1 = 1.0;
        let c2 = 0.8;
        let c3 = 0.6;
        let x0 = x as f32;
        let x1 = x as f32 + 1.0;
        let y0 = y as f32;
        let y1 = y as f32 + 1.0;
        let z0 = z as f32;
        let z1 = z as f32 + 1.0;
        if !level.is_solid_tile(x, y - 1, z) {
            let br = level.get_brightness(x, y - 1, z) * c1;
            if br == c1 && c1 as i32 ^ layer == 1 {
                t.color(br, br, br);
                t.tex(u0, v1);
                t.vertex(x0, y0, z1);
                t.tex(u0, v0);
                t.vertex(x0, y0, z0);
                t.tex(u1, v0);
                t.vertex(x1, y0, z0);
                t.tex(u1, v1);
                t.vertex(x1, y0, z1);
            }
        }

        if !level.is_solid_tile(x, y + 1, z) {
            let br = level.get_brightness(x, y, z) * c1;
            if br == c1 && c1 as i32 ^ layer == 1 {
                t.color(br, br, br);
                t.tex(u1, v1);
                t.vertex(x1, y1, z1);
                t.tex(u1, v0);
                t.vertex(x1, y1, z0);
                t.tex(u0, v0);
                t.vertex(x0, y1, z0);
                t.tex(u0, v1);
                t.vertex(x0, y1, z1);
            }
        }

        if !level.is_solid_tile(x, y, z - 1) {
            let br = level.get_brightness(x, y, z - 1) * c2;
            if br == c2 && c2 as i32 ^ layer == 1 {
                t.color(br, br, br);
                t.tex(u1, v0);
                t.vertex(x0, y1, z0);
                t.tex(u0, v0);
                t.vertex(x1, y1, z0);
                t.tex(u0, v1);
                t.vertex(x1, y0, z0);
                t.tex(u1, v1);
                t.vertex(x0, y0, z0);
            }
        }

        if !level.is_solid_tile(x, y, z + 1) {
            let br = level.get_brightness(x, y, z + 1) * c2;
            if br == c2 && c2 as i32 ^ layer == 1 {
                t.color(br, br, br);
                t.tex(u0, v0);
                t.vertex(x0, y1, z1);
                t.tex(u0, v1);
                t.vertex(x0, y0, z1);
                t.tex(u1, v1);
                t.vertex(x1, y0, z1);
                t.tex(u1, v0);
                t.vertex(x1, y1, z1);
            }
        }

        if !level.is_solid_tile(x - 1, y, z) {
            let br = level.get_brightness(x - 1, y, z) * c3;
            if br == c3 && c3 as i32 ^ layer == 1 {
                t.color(br, br, br);
                t.tex(u1, v0);
                t.vertex(x0, y1, z1);
                t.tex(u0, v0);
                t.vertex(x0, y1, z0);
                t.tex(u0, v1);
                t.vertex(x0, y0, z0);
                t.tex(u1, v1);
                t.vertex(x0, y0, z1);
            }
        }

        if !level.is_solid_tile(x + 1, y, z) {
            let br = level.get_brightness(x + 1, y, z) * c3;
            if br == c3 && c3 as i32 ^ layer == 1 {
                t.color(br, br, br);
                t.tex(u0, v1);
                t.vertex(x1, y0, z1);
                t.tex(u1, v1);
                t.vertex(x1, y0, z0);
                t.tex(u1, v0);
                t.vertex(x1, y1, z0);
                t.tex(u0, v0);
                t.vertex(x1, y1, z1);
            }
        }
    }

    pub fn render_face(&self, mut t: Tesselator, x: JInt, y: JInt, z: JInt, face: JInt) {
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
}
