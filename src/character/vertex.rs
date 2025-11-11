use crate::{character::vec3::Vec3, java::JFloat};

pub struct Vertex {
    pub pos: Vec3,
    pub u: JFloat,
    pub v: JFloat,
}

impl Vertex {
    pub fn new(x: JFloat, y: JFloat, z: JFloat, u: JFloat, v: JFloat) -> Self {
        Self {
            pos: Vec3::new(x, y, z),
            u,
            v,
        }
    }

    pub fn new_from_vertex(vertex: &Self, u: JFloat, v: JFloat) -> Self {
        Self {
            pos: vertex.pos.clone(),
            u,
            v,
        }
    }

    pub fn new_from_pos(pos: Vec3, u: JFloat, v: JFloat) -> Self {
        Self { pos, u, v }
    }

    pub fn remap(&self, u: JFloat, v: JFloat) -> Self {
        Vertex::new_from_vertex(self, u, v)
    }
}
