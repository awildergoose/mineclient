use crate::{
    character::vec3::Vec3,
    java::{JFloat, JInt},
};

#[derive(Clone)]
pub struct Vertex {
    pub pos: Vec3,
    pub u: JFloat,
    pub v: JFloat,
}

impl Vertex {
    #[must_use]
    pub const fn new(x: JFloat, y: JFloat, z: JFloat, u: JFloat, v: JFloat) -> Self {
        Self {
            pos: Vec3::new(x, y, z),
            u,
            v,
        }
    }

    #[must_use]
    pub fn new_from_vertex(vertex: &Self, u: JFloat, v: JFloat) -> Self {
        Self {
            pos: vertex.pos.clone(),
            u,
            v,
        }
    }

    #[must_use]
    pub const fn new_from_pos(pos: Vec3, u: JFloat, v: JFloat) -> Self {
        Self { pos, u, v }
    }

    #[must_use]
    pub fn remap(&self, u: JFloat, v: JFloat) -> Self {
        Self::new_from_vertex(self, u, v)
    }

    // helper
    #[inline]
    #[must_use]
    pub fn remapi(&self, u: JInt, v: JInt) -> Self {
        Self::new_from_vertex(self, u as JFloat, v as JFloat)
    }
}
