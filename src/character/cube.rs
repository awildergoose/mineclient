use std::f32::consts::PI;

use crate::{
    character::{polygon::Polygon, vertex::Vertex},
    gl,
    java::{JFloat, JInt},
};

pub struct Cube {
    vertices: Vec<Vertex>,
    polygons: Vec<Polygon>,
    x_tex_offs: JInt,
    y_tex_offs: JInt,
    pub x: JFloat,
    pub y: JFloat,
    pub z: JFloat,
    pub x_rot: JFloat,
    pub y_rot: JFloat,
    pub z_rot: JFloat,
}

impl Cube {
    pub fn new(x_tex_offs: JInt, y_tex_offs: JInt) -> Self {
        Self {
            x_tex_offs,
            y_tex_offs,
            vertices: Vec::new(),
            polygons: Vec::new(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
        }
    }

    pub fn set_tex_offs(&mut self, x_tex_offs: JInt, y_tex_offs: JInt) {
        self.x_tex_offs = x_tex_offs;
        self.y_tex_offs = y_tex_offs;
    }

    pub fn add_box(&mut self, x0: JFloat, y0: JFloat, z0: JFloat, w: JInt, h: JInt, d: JInt) {
        self.vertices.clear();
        self.polygons.clear();

        let x1 = x0 + w as f32;
        let y1 = y0 + h as f32;
        let z1 = z0 + d as f32;

        let u0 = Vertex::new(x0, y0, z0, 0.0, 0.0);
        let u1 = Vertex::new(x1, y0, z0, 0.0, 8.0);
        let u2 = Vertex::new(x1, y1, z0, 8.0, 8.0);
        let u3 = Vertex::new(x0, y1, z0, 8.0, 0.0);
        let l0 = Vertex::new(x0, y0, z1, 0.0, 0.0);
        let l1 = Vertex::new(x1, y0, z1, 0.0, 8.0);
        let l2 = Vertex::new(x1, y1, z1, 8.0, 8.0);
        let l3 = Vertex::new(x0, y1, z1, 8.0, 0.0);

        self.vertices.push(u0.clone());
        self.vertices.push(u1.clone());
        self.vertices.push(u2.clone());
        self.vertices.push(u3.clone());
        self.vertices.push(l0.clone());
        self.vertices.push(l1.clone());
        self.vertices.push(l2.clone());
        self.vertices.push(l3.clone());

        self.polygons.push(Polygon::new_with_uv(
            vec![l1.clone(), u1.clone(), u2.clone(), l2.clone()],
            self.x_tex_offs + d + w,
            self.y_tex_offs + d,
            self.x_tex_offs + d + w + d,
            self.y_tex_offs + d + h,
        ));
        self.polygons.push(Polygon::new_with_uv(
            vec![u0.clone(), l0.clone(), l3.clone(), u3.clone()],
            self.x_tex_offs,
            self.y_tex_offs + d,
            self.x_tex_offs + d,
            self.y_tex_offs + d + h,
        ));
        self.polygons.push(Polygon::new_with_uv(
            vec![l1.clone(), l0.clone(), u0.clone(), u1.clone()],
            self.x_tex_offs + d,
            self.y_tex_offs,
            self.x_tex_offs + d + w,
            self.y_tex_offs + d,
        ));
        self.polygons.push(Polygon::new_with_uv(
            vec![u2.clone(), u3.clone(), l3.clone(), l2.clone()],
            self.x_tex_offs + d + w,
            self.y_tex_offs,
            self.x_tex_offs + d + w + w,
            self.y_tex_offs + d,
        ));
        self.polygons.push(Polygon::new_with_uv(
            vec![u1.clone(), u0.clone(), u3.clone(), u2.clone()],
            self.x_tex_offs + d,
            self.y_tex_offs + d,
            self.x_tex_offs + d + w,
            self.y_tex_offs + d + h,
        ));
        self.polygons.push(Polygon::new_with_uv(
            vec![l0.clone(), l1.clone(), l2.clone(), l3.clone()],
            self.x_tex_offs + d + w + d,
            self.y_tex_offs + d,
            self.x_tex_offs + d + w + d + w,
            self.y_tex_offs + d + h,
        ));
    }

    pub fn set_pos(&mut self, x: JFloat, y: JFloat, z: JFloat) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn render(&self) {
        let c = 180.0 / PI;
        unsafe {
            gl::PushMatrix();
            gl::Translatef(self.x, self.y, self.z);
            gl::Rotatef(self.z_rot * c, 0.0, 0.0, 1.0);
            gl::Rotatef(self.y_rot * c, 0.0, 1.0, 0.0);
            gl::Rotatef(self.x_rot * c, 1.0, 0.0, 0.0);
            gl::Begin(gl::QUADS);

            for p in &self.polygons {
                p.render();
            }

            gl::End();
            gl::PopMatrix();
        }
    }
}
