use std::ffi::c_void;

use crate::{
    gl,
    java::{JBoolean, JFloat, JInt},
};

pub static MAX_MEMORY_USE: JInt = 4194304;
pub static MAX_FLOATS: usize = 524288;

pub struct Tesselator {
    buffer: Vec<JFloat>,
    vertices: JInt,
    u: JFloat,
    v: JFloat,
    r: JFloat,
    g: JFloat,
    b: JFloat,
    has_color: JBoolean,
    has_texture: JBoolean,
    len: usize,
    p: usize,
}

impl Tesselator {
    pub fn new() -> Self {
        Self {
            buffer: vec![0.0; MAX_FLOATS],
            vertices: 0,
            u: 0.0,
            v: 0.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            has_color: false,
            has_texture: false,
            len: 3,
            p: 0,
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            let buffer = self.buffer.as_mut_ptr() as *const c_void;
            if self.has_texture && self.has_color {
                gl::InterleavedArrays(10794, 0, buffer);
            } else if self.has_texture {
                gl::InterleavedArrays(10791, 0, buffer);
            } else if self.has_color {
                gl::InterleavedArrays(10788, 0, buffer);
            } else {
                gl::InterleavedArrays(10785, 0, buffer);
            }

            gl::EnableClientState(32884);
            if self.has_texture {
                gl::EnableClientState(32888);
            }
            if self.has_color {
                gl::EnableClientState(32886);
            }

            gl::DrawArrays(7, 0, self.vertices);
            gl::DisableClientState(32884);
            if self.has_texture {
                gl::DisableClientState(32888);
            }
            if self.has_color {
                gl::DisableClientState(32886);
            }
        }

        self.clear();
    }

    pub fn clear(&mut self) {
        self.vertices = 0;
        // Should we clear? Probably not.
        self.p = 0;
    }

    pub fn init(&mut self) {
        self.clear();
        self.has_color = false;
        self.has_texture = false;
    }

    pub fn tex(&mut self, u: JFloat, v: JFloat) {
        if !self.has_texture {
            self.len += 2;
        }

        self.has_texture = true;
        self.u = u;
        self.v = v;
    }

    pub fn color(&mut self, r: JFloat, g: JFloat, b: JFloat) {
        if !self.has_color {
            self.len += 3;
        }

        self.has_color = true;
        self.r = r;
        self.g = g;
        self.b = b;
    }

    pub fn vertex_uv(&mut self, x: JFloat, y: JFloat, z: JFloat, u: JFloat, v: JFloat) {
        self.tex(u, v);
        self.vertex(x, y, z);
    }

    pub fn vertex(&mut self, x: JFloat, y: JFloat, z: JFloat) {
        if self.has_texture {
            let p = self.advance();
            self.buffer[p] = self.u;
            let p = self.advance();
            self.buffer[p] = self.v;
        }

        if self.has_color {
            let p = self.advance();
            self.buffer[p] = self.r;
            let p = self.advance();
            self.buffer[p] = self.g;
            let p = self.advance();
            self.buffer[p] = self.b;
        }

        let p = self.advance();
        self.buffer[p] = x;
        let p = self.advance();
        self.buffer[p] = y;
        let p = self.advance();
        self.buffer[p] = z;
        self.vertices += 1;

        if self.vertices % 4 == 0 && self.p >= MAX_FLOATS - self.len * 4 {
            self.flush();
        }
    }

    #[inline]
    fn advance(&mut self) -> usize {
        self.p += 1;
        self.p
    }
}

impl Default for Tesselator {
    fn default() -> Self {
        Self::new()
    }
}
