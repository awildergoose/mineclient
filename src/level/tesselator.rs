use crate::{
    gl,
    java::{JBoolean, JFloat, JInt},
};

pub static MAX_VERTICES: JInt = 100000;

pub struct Tesselator {
    vertex_buffer: Vec<JFloat>,
    tex_coord_buffer: Vec<JFloat>,
    color_buffer: Vec<JFloat>,
    vertices: JInt,
    u: JFloat,
    v: JFloat,
    r: JFloat,
    g: JFloat,
    b: JFloat,
    has_color: JBoolean,
    has_texture: JBoolean,
}

impl Tesselator {
    pub fn new() -> Self {
        Self {
            vertex_buffer: vec![0.0; 300000],
            tex_coord_buffer: vec![0.0; 200000],
            color_buffer: vec![0.0; 300000],
            vertices: 0,
            u: 0.0,
            v: 0.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            has_color: false,
            has_texture: false,
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            gl::VertexPointer(3, gl::FLOAT, 0, self.vertex_buffer.as_ptr() as *const _);

            if self.has_texture {
                gl::TexCoordPointer(2, gl::FLOAT, 0, self.tex_coord_buffer.as_ptr() as *const _);
            }

            if self.has_color {
                gl::ColorPointer(3, gl::FLOAT, 0, self.color_buffer.as_ptr() as *const _);
            }

            gl::EnableClientState(gl::VERTEX_ARRAY);
            if self.has_texture {
                gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);
            }
            if self.has_color {
                gl::EnableClientState(gl::COLOR_ARRAY);
            }

            gl::DrawArrays(gl::QUADS, 0, self.vertices);

            gl::DisableClientState(gl::VERTEX_ARRAY);
            if self.has_texture {
                gl::DisableClientState(gl::TEXTURE_COORD_ARRAY);
            }
            if self.has_color {
                gl::DisableClientState(gl::COLOR_ARRAY);
            }
        }

        self.clear();
    }

    pub fn clear(&mut self) {
        self.vertices = 0;
    }

    pub fn init(&mut self) {
        self.clear();
        self.has_color = false;
        self.has_texture = false;
    }

    pub fn tex(&mut self, u: JFloat, v: JFloat) {
        self.has_texture = true;
        self.u = u;
        self.v = v;
    }

    pub fn color(&mut self, r: JFloat, g: JFloat, b: JFloat) {
        self.has_color = true;
        self.r = r;
        self.g = g;
        self.b = b;
    }

    pub fn vertex(&mut self, x: JFloat, y: JFloat, z: JFloat) {
        let vi = (self.vertices * 3) as usize;
        self.vertex_buffer[vi] = x;
        self.vertex_buffer[vi + 1] = y;
        self.vertex_buffer[vi + 2] = z;

        if self.has_texture {
            let ti = (self.vertices * 2) as usize;
            self.tex_coord_buffer[ti] = self.u;
            self.tex_coord_buffer[ti + 1] = self.v;
        }

        if self.has_color {
            let ci = (self.vertices * 3) as usize;
            self.color_buffer[ci] = self.r;
            self.color_buffer[ci + 1] = self.g;
            self.color_buffer[ci + 2] = self.b;
        }

        self.vertices += 1;

        if self.vertices == MAX_VERTICES {
            self.flush();
        }
    }
}

impl Default for Tesselator {
    fn default() -> Self {
        Self::new()
    }
}
