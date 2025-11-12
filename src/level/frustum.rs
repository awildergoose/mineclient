use std::sync::{Mutex, OnceLock};

use crate::{
    gl,
    java::{JBoolean, JFloat, JInt},
    phys::aabb::AABB,
};

pub static RIGHT: JInt = 0;
pub static LEFT: JInt = 1;
pub static BOTTOM: JInt = 2;
pub static TOP: JInt = 3;
pub static BACK: JInt = 4;
pub static FRONT: JInt = 5;
pub static A: JInt = 0;
pub static B: JInt = 1;
pub static C: JInt = 2;
pub static D: JInt = 3;

pub struct Frustum {
    pub m_frustum: [[f32; 4]; 6],
    _proj: [f32; 16],
    _modl: [f32; 16],
    _clip: [f32; 16],
    proj: [f32; 16],
    modl: [f32; 16],
    clip: [f32; 16],
}

pub fn get_frustum() -> &'static Mutex<Frustum> {
    static FRUSTUM: OnceLock<Mutex<Frustum>> = OnceLock::new();
    let f = FRUSTUM.get_or_init(|| Mutex::new(Frustum::new()));
    f.lock().unwrap().calculate_frustum();
    f
}

impl Frustum {
    pub fn new() -> Self {
        Self {
            m_frustum: [[0.0; 4]; 6],
            _proj: [0.0; 16],
            _modl: [0.0; 16],
            _clip: [0.0; 16],
            proj: [0.0; 16],
            modl: [0.0; 16],
            clip: [0.0; 16],
        }
    }

    fn normalize_plane(frustum: &mut [[f32; 4]; 6], side: JInt) {
        let side = side as usize;
        let magnitude = f32::sqrt(
            frustum[side][0] * frustum[side][0]
                + frustum[side][1] * frustum[side][1]
                + frustum[side][2] * frustum[side][2],
        );
        for j in 0..4 {
            frustum[side][j] /= magnitude;
        }
    }

    fn calculate_frustum(&mut self) {
        self._proj = [0.0; 16];
        self._modl = [0.0; 16];
        self._clip = [0.0; 16];
        unsafe {
            gl::GetFloatv(gl::PROJECTION_MATRIX, self._proj.as_mut_ptr());
            gl::GetFloatv(gl::MODELVIEW_MATRIX, self._modl.as_mut_ptr());
        }
        self.proj.copy_from_slice(&self._proj);
        self.modl.copy_from_slice(&self._modl);
        self.clip[0] = self.modl[0] * self.proj[0]
            + self.modl[1] * self.proj[4]
            + self.modl[2] * self.proj[8]
            + self.modl[3] * self.proj[12];
        self.clip[1] = self.modl[0] * self.proj[1]
            + self.modl[1] * self.proj[5]
            + self.modl[2] * self.proj[9]
            + self.modl[3] * self.proj[13];
        self.clip[2] = self.modl[0] * self.proj[2]
            + self.modl[1] * self.proj[6]
            + self.modl[2] * self.proj[10]
            + self.modl[3] * self.proj[14];
        self.clip[3] = self.modl[0] * self.proj[3]
            + self.modl[1] * self.proj[7]
            + self.modl[2] * self.proj[11]
            + self.modl[3] * self.proj[15];
        self.clip[4] = self.modl[4] * self.proj[0]
            + self.modl[5] * self.proj[4]
            + self.modl[6] * self.proj[8]
            + self.modl[7] * self.proj[12];
        self.clip[5] = self.modl[4] * self.proj[1]
            + self.modl[5] * self.proj[5]
            + self.modl[6] * self.proj[9]
            + self.modl[7] * self.proj[13];
        self.clip[6] = self.modl[4] * self.proj[2]
            + self.modl[5] * self.proj[6]
            + self.modl[6] * self.proj[10]
            + self.modl[7] * self.proj[14];
        self.clip[7] = self.modl[4] * self.proj[3]
            + self.modl[5] * self.proj[7]
            + self.modl[6] * self.proj[11]
            + self.modl[7] * self.proj[15];
        self.clip[8] = self.modl[8] * self.proj[0]
            + self.modl[9] * self.proj[4]
            + self.modl[10] * self.proj[8]
            + self.modl[11] * self.proj[12];
        self.clip[9] = self.modl[8] * self.proj[1]
            + self.modl[9] * self.proj[5]
            + self.modl[10] * self.proj[9]
            + self.modl[11] * self.proj[13];
        self.clip[10] = self.modl[8] * self.proj[2]
            + self.modl[9] * self.proj[6]
            + self.modl[10] * self.proj[10]
            + self.modl[11] * self.proj[14];
        self.clip[11] = self.modl[8] * self.proj[3]
            + self.modl[9] * self.proj[7]
            + self.modl[10] * self.proj[11]
            + self.modl[11] * self.proj[15];
        self.clip[12] = self.modl[12] * self.proj[0]
            + self.modl[13] * self.proj[4]
            + self.modl[14] * self.proj[8]
            + self.modl[15] * self.proj[12];
        self.clip[13] = self.modl[12] * self.proj[1]
            + self.modl[13] * self.proj[5]
            + self.modl[14] * self.proj[9]
            + self.modl[15] * self.proj[13];
        self.clip[14] = self.modl[12] * self.proj[2]
            + self.modl[13] * self.proj[6]
            + self.modl[14] * self.proj[10]
            + self.modl[15] * self.proj[14];
        self.clip[15] = self.modl[12] * self.proj[3]
            + self.modl[13] * self.proj[7]
            + self.modl[14] * self.proj[11]
            + self.modl[15] * self.proj[15];
        self.m_frustum[0][0] = self.clip[3] - self.clip[0];
        self.m_frustum[0][1] = self.clip[7] - self.clip[4];
        self.m_frustum[0][2] = self.clip[11] - self.clip[8];
        self.m_frustum[0][3] = self.clip[15] - self.clip[12];
        Self::normalize_plane(&mut self.m_frustum, 0);
        self.m_frustum[1][0] = self.clip[3] + self.clip[0];
        self.m_frustum[1][1] = self.clip[7] + self.clip[4];
        self.m_frustum[1][2] = self.clip[11] + self.clip[8];
        self.m_frustum[1][3] = self.clip[15] + self.clip[12];
        Self::normalize_plane(&mut self.m_frustum, 1);
        self.m_frustum[2][0] = self.clip[3] + self.clip[1];
        self.m_frustum[2][1] = self.clip[7] + self.clip[5];
        self.m_frustum[2][2] = self.clip[11] + self.clip[9];
        self.m_frustum[2][3] = self.clip[15] + self.clip[13];
        Self::normalize_plane(&mut self.m_frustum, 2);
        self.m_frustum[3][0] = self.clip[3] - self.clip[1];
        self.m_frustum[3][1] = self.clip[7] - self.clip[5];
        self.m_frustum[3][2] = self.clip[11] - self.clip[9];
        self.m_frustum[3][3] = self.clip[15] - self.clip[13];
        Self::normalize_plane(&mut self.m_frustum, 3);
        self.m_frustum[4][0] = self.clip[3] - self.clip[2];
        self.m_frustum[4][1] = self.clip[7] - self.clip[6];
        self.m_frustum[4][2] = self.clip[11] - self.clip[10];
        self.m_frustum[4][3] = self.clip[15] - self.clip[14];
        Self::normalize_plane(&mut self.m_frustum, 4);
        self.m_frustum[5][0] = self.clip[3] + self.clip[2];
        self.m_frustum[5][1] = self.clip[7] + self.clip[6];
        self.m_frustum[5][2] = self.clip[11] + self.clip[10];
        self.m_frustum[5][3] = self.clip[15] + self.clip[14];
        Self::normalize_plane(&mut self.m_frustum, 5);
    }

    pub fn point_in_frustum(&self, x: JFloat, y: JFloat, z: JFloat) -> JBoolean {
        for i in 0..6 {
            if self.m_frustum[i][0] * x
                + self.m_frustum[i][1] * y
                + self.m_frustum[i][2] * z
                + self.m_frustum[i][3]
                <= 0.0
            {
                return false;
            }
        }

        true
    }

    pub fn sphere_in_frustum(&self, x: JFloat, y: JFloat, z: JFloat, radius: JFloat) -> JBoolean {
        for i in 0..6 {
            if self.m_frustum[i][0] * x
                + self.m_frustum[i][1] * y
                + self.m_frustum[i][2] * z
                + self.m_frustum[i][3]
                <= -radius
            {
                return false;
            }
        }

        true
    }

    pub fn cube_fully_in_frustum(
        &self,
        x1: JFloat,
        y1: JFloat,
        z1: JFloat,
        x2: JFloat,
        y2: JFloat,
        z2: JFloat,
    ) -> JBoolean {
        for i in 0..6 {
            if !(self.m_frustum[i][0] * x1
                + self.m_frustum[i][1] * y1
                + self.m_frustum[i][2] * z1
                + self.m_frustum[i][3]
                > 0.0)
            {
                return false;
            }

            if !(self.m_frustum[i][0] * x2
                + self.m_frustum[i][1] * y1
                + self.m_frustum[i][2] * z1
                + self.m_frustum[i][3]
                > 0.0)
            {
                return false;
            }

            if !(self.m_frustum[i][0] * x1
                + self.m_frustum[i][1] * y2
                + self.m_frustum[i][2] * z1
                + self.m_frustum[i][3]
                > 0.0)
            {
                return false;
            }

            if !(self.m_frustum[i][0] * x2
                + self.m_frustum[i][1] * y2
                + self.m_frustum[i][2] * z1
                + self.m_frustum[i][3]
                > 0.0)
            {
                return false;
            }

            if !(self.m_frustum[i][0] * x1
                + self.m_frustum[i][1] * y1
                + self.m_frustum[i][2] * z2
                + self.m_frustum[i][3]
                > 0.0)
            {
                return false;
            }

            if !(self.m_frustum[i][0] * x2
                + self.m_frustum[i][1] * y1
                + self.m_frustum[i][2] * z2
                + self.m_frustum[i][3]
                > 0.0)
            {
                return false;
            }

            if !(self.m_frustum[i][0] * x1
                + self.m_frustum[i][1] * y2
                + self.m_frustum[i][2] * z2
                + self.m_frustum[i][3]
                > 0.0)
            {
                return false;
            }

            if !(self.m_frustum[i][0] * x2
                + self.m_frustum[i][1] * y2
                + self.m_frustum[i][2] * z2
                + self.m_frustum[i][3]
                > 0.0)
            {
                return false;
            }
        }

        true
    }

    pub fn cube_in_frustum(
        &self,
        x1: JFloat,
        y1: JFloat,
        z1: JFloat,
        x2: JFloat,
        y2: JFloat,
        z2: JFloat,
    ) -> JBoolean {
        for i in 0..6 {
            if !(self.m_frustum[i][0] * x1
                + self.m_frustum[i][1] * y1
                + self.m_frustum[i][2] * z1
                + self.m_frustum[i][3]
                > 0.0)
                && !(self.m_frustum[i][0] * x2
                    + self.m_frustum[i][1] * y1
                    + self.m_frustum[i][2] * z1
                    + self.m_frustum[i][3]
                    > 0.0)
                && !(self.m_frustum[i][0] * x1
                    + self.m_frustum[i][1] * y2
                    + self.m_frustum[i][2] * z1
                    + self.m_frustum[i][3]
                    > 0.0)
                && !(self.m_frustum[i][0] * x2
                    + self.m_frustum[i][1] * y2
                    + self.m_frustum[i][2] * z1
                    + self.m_frustum[i][3]
                    > 0.0)
                && !(self.m_frustum[i][0] * x1
                    + self.m_frustum[i][1] * y1
                    + self.m_frustum[i][2] * z2
                    + self.m_frustum[i][3]
                    > 0.0)
                && !(self.m_frustum[i][0] * x2
                    + self.m_frustum[i][1] * y1
                    + self.m_frustum[i][2] * z2
                    + self.m_frustum[i][3]
                    > 0.0)
                && !(self.m_frustum[i][0] * x1
                    + self.m_frustum[i][1] * y2
                    + self.m_frustum[i][2] * z2
                    + self.m_frustum[i][3]
                    > 0.0)
                && !(self.m_frustum[i][0] * x2
                    + self.m_frustum[i][1] * y2
                    + self.m_frustum[i][2] * z2
                    + self.m_frustum[i][3]
                    > 0.0)
            {
                return false;
            }
        }

        true
    }

    pub fn aabb_in_frustum(&self, aabb: &AABB) -> JBoolean {
        self.cube_in_frustum(aabb.x0, aabb.y0, aabb.z0, aabb.x1, aabb.y1, aabb.z1)
    }
}

impl Default for Frustum {
    fn default() -> Self {
        Self::new()
    }
}
