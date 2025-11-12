use crate::{character::vertex::Vertex, gl, java::JInt};

pub struct Polygon {
    vertices: Vec<Vertex>,
    // omitted vertexCount
}

impl Polygon {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }

    pub fn new_with_uv(mut vertices: Vec<Vertex>, u0: JInt, v0: JInt, u1: JInt, v1: JInt) -> Self {
        vertices[0] = vertices[0].remapi(u1, v0);
        vertices[1] = vertices[1].remapi(u0, v0);
        vertices[2] = vertices[2].remapi(u0, v1);
        vertices[3] = vertices[3].remapi(u1, v1);
        Self { vertices }
    }

    pub fn render(&self) {
        unsafe {
            gl::Color3f(1.0, 1.0, 1.0);

            for i in (0..=3).rev() {
                let v = &self.vertices[i];
                gl::TexCoord2f(v.u / 64.0, v.v / 32.0);
                gl::Vertex3f(v.pos.x, v.pos.y, v.pos.z);
            }
        }
    }
}
