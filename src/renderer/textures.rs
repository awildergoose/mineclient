use std::{cell::RefCell, collections::HashMap, ffi::c_void};

use image::{DynamicImage, ImageResult};

use crate::{
    gl,
    gl::types::{GLenum, GLint, GLsizei},
};

pub struct Textures {
    id_map: RefCell<HashMap<String, u32>>,
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

pub fn resolve_texture(name: &str) -> ImageResult<DynamicImage> {
    match name {
        "terrain.png" => image::load_from_memory(include_bytes!("../../assets/terrain.png")),
        "char.png" => image::load_from_memory(include_bytes!("../../assets/char.png")),
        "default.gif" => image::load_from_memory(include_bytes!("../../assets/default.gif")),
        _ => panic!("tried to resolve unknown texture: {name}"),
    }
}

impl Textures {
    #[must_use]
    pub fn new() -> Self {
        Self {
            id_map: RefCell::new(HashMap::new()),
        }
    }

    // Should this really be inline?
    #[inline]
    pub fn load_texture(&self, resource_name: &str, mode: i32) -> u32 {
        if let Some(&id) = self.id_map.borrow().get(resource_name) {
            return id;
        }

        let img = resolve_texture(resource_name)
            .unwrap_or_else(|_| panic!("Failed to load texture: {resource_name}"))
            .into_rgba8();

        let (w, h) = img.dimensions();
        let pixels = img.into_raw();

        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &raw mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            println!("{resource_name} -> {id}");

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
                pixels.as_ptr().cast(),
            );

            gluBuild2DMipmaps(
                gl::TEXTURE_2D,
                gl::RGBA as i32,
                w as i32,
                h as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr().cast(),
            );
        }

        self.id_map
            .borrow_mut()
            .insert(resource_name.to_string(), id);
        id
    }
}

impl Default for Textures {
    fn default() -> Self {
        Self::new()
    }
}
