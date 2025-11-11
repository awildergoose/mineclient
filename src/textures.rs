use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Mutex, OnceLock};

use image::{DynamicImage, ImageResult};

use crate::gl;
use crate::gl::types::{GLenum, GLint, GLsizei};

pub struct Textures {
    id_map: RefCell<HashMap<String, u32>>,
    last_id: RefCell<i32>,
}

pub fn get_textures() -> &'static Mutex<Textures> {
    static TEXTURES: OnceLock<Mutex<Textures>> = OnceLock::new();
    TEXTURES.get_or_init(|| Mutex::new(Textures::new()))
}

unsafe extern "C" {
    pub fn gluBuild2DMipmaps(
        target: GLenum,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        data: *const c_void,
    ) -> GLint;
}

fn resolve_texture(name: &str) -> ImageResult<DynamicImage> {
    match name {
        "terrain.png" => image::load_from_memory(include_bytes!("../assets/terrain.png")),
        _ => panic!("tried to resolve unknown texture: {}", name),
    }
}

impl Textures {
    pub fn new() -> Self {
        Self {
            id_map: RefCell::new(HashMap::new()),
            last_id: RefCell::new(-9999999),
        }
    }

    #[inline(always)]
    pub fn load_texture(&self, resource_name: &str, mode: i32) -> u32 {
        if let Some(&id) = self.id_map.borrow().get(resource_name) {
            return id;
        }

        let img = resolve_texture(resource_name)
            .unwrap_or_else(|_| panic!("Failed to load texture: {}", resource_name))
            .into_rgba8();

        let (w, h) = img.dimensions();
        let pixels = img.into_raw();

        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            self.bind(id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mode);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mode);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                w as i32,
                h as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const _,
            );

            gluBuild2DMipmaps(
                gl::TEXTURE_2D,
                gl::RGBA as i32,
                w as i32,
                h as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const _,
            );
        }

        self.id_map
            .borrow_mut()
            .insert(resource_name.to_string(), id);
        id
    }

    pub fn bind(&self, id: u32) {
        unsafe {
            let mut last = self.last_id.borrow_mut();
            if *last != id as i32 {
                gl::BindTexture(gl::TEXTURE_2D, id);
                *last = id as i32;
            }
        }
    }
}

impl Default for Textures {
    fn default() -> Self {
        Self::new()
    }
}
