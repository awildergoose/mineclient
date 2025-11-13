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
    no_color: JBoolean,
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
            no_color: false,
            len: 3,
            p: 0,
        }
    }

    pub fn flush(&mut self) {
        if self.vertices > 0 {
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
        self.no_color = false;
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
        if !self.no_color {
            if !self.has_color {
                self.len += 3;
            }

            self.has_color = true;
            self.r = r;
            self.g = g;
            self.b = b;
        }
    }

    pub fn colori(&mut self, c: JInt) {
        let r = (c >> 16 & 0xFF) as f32 / 255.0;
        let g = (c >> 8 & 0xFF) as f32 / 255.0;
        let b = (c & 0xFF) as f32 / 255.0;
        self.color(r, g, b);
    }

    pub fn vertex_uv(&mut self, x: JFloat, y: JFloat, z: JFloat, u: JFloat, v: JFloat) {
        self.tex(u, v);
        self.vertex(x, y, z);
    }

    pub fn no_color(&mut self) {
        self.no_color = true;
    }

    pub fn vertex(&mut self, x: JFloat, y: JFloat, z: JFloat) {
        let mut p = self.p;
        unsafe {
            let ptr = self.buffer.as_mut_ptr();

            if self.has_texture {
                *ptr.add(p) = self.u;
                p += 1;
                *ptr.add(p) = self.v;
                p += 1;
            }

            if self.has_color {
                *ptr.add(p) = self.r;
                p += 1;
                *ptr.add(p) = self.g;
                p += 1;
                *ptr.add(p) = self.b;
                p += 1;
            }

            *ptr.add(p) = x;
            p += 1;
            *ptr.add(p) = y;
            p += 1;
            *ptr.add(p) = z;
            p += 1;
        }

        self.p = p;
        self.vertices += 1;

        if self.vertices % 4 == 0 {
            let threshold = MAX_FLOATS.saturating_sub(self.len * 4);
            if self.p >= threshold {
                self.flush();
            }
        }
    }
}

impl Default for Tesselator {
    fn default() -> Self {
        Self::new()
    }
}
