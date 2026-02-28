use std::{cell::RefCell, rc::Rc};

use crate::{
    gl,
    java::{JBoolean, JInt},
    renderer::{
        tesselator::Tesselator,
        textures::{Textures, resolve_texture},
    },
};

pub struct Font {
    pub char_widths: [i32; 256],
    pub font_texture: u32,
}

impl Font {
    pub fn new(name: &str, textures: Rc<RefCell<Textures>>) -> Self {
        let img = resolve_texture(name)
            .unwrap_or_else(|_| panic!("Failed to load font image: {name}"))
            .into_rgba8();

        let (w, _h) = img.dimensions();
        let raw_pixels = img.into_raw();

        let mut char_widths = [0i32; 256];

        #[allow(clippy::needless_range_loop)] // LLVM probably optimizes it anyway
        for i in 0..128 {
            let xt = i % 16;
            let yt = i / 16;
            let mut x = 0;

            'col: for col in 0..8 {
                let x_pixel = xt * 8 + col;
                let mut empty_column = true;

                for row in 0..8 {
                    let y_pixel = (yt * 8 + row) * w as usize;
                    let idx = x_pixel + y_pixel;
                    let pixel = raw_pixels[idx * 4 + 2];

                    if pixel > 128 {
                        empty_column = false;
                        break;
                    }
                }

                if empty_column {
                    break 'col;
                }
                x += 1;
            }

            if i == 32 {
                x = 4;
            }

            char_widths[i] = x;
        }

        let font_texture = textures.borrow_mut().load_texture(name, 9728);

        Self {
            char_widths,
            font_texture,
        }
    }

    pub fn draw_shadow(&self, t: &mut Tesselator, s: String, x: JInt, y: JInt, color: JInt) {
        self.draw(t, &s, x + 1, y + 1, color, true);
        self.draw_no_shadow(t, s.clone(), x, y, color);
    }

    pub fn draw_no_shadow(&self, t: &mut Tesselator, s: String, x: JInt, y: JInt, color: JInt) {
        self.draw(t, &s, x, y, color, false);
    }

    pub fn draw(
        &self,
        t: &mut Tesselator,
        s: &str,
        x: JInt,
        y: JInt,
        mut color: JInt,
        darken: JBoolean,
    ) {
        let chars: Vec<char> = s.chars().collect();
        if darken {
            color = (color & 16_579_836) >> 2;
        }

        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, self.font_texture);
        }

        t.begin();
        t.colori(color);
        let mut xo = 0i32;
        let mut i = 0usize;

        while i < chars.len() {
            let c = chars[i];

            if c == '&' {
                if i + 1 < chars.len()
                    && let Some(cc_pos) = "0123456789abcdef".find(chars[i + 1])
                {
                    let cc = cc_pos as i32;
                    let br = (cc & 8) * 8;
                    let b = (cc & 1) * 191 + br;
                    let g = ((cc & 2) >> 1) * 191 + br;
                    let r = ((cc & 4) >> 2) * 191 + br;
                    color = (r << 16) | (g << 8) | b;
                    i += 2;
                    if darken {
                        color = (color & 16_579_836) >> 2;
                    }
                    t.colori(color);
                    continue;
                }

                i += 1;
                continue;
            }

            let ci = c as i32;
            let ix = ((ci % 16) * 8) as f32;
            let iy = ((ci / 16) * 8) as f32;
            t.vertex_uv(
                (x + xo) as f32,
                (y + 8) as f32,
                0.0,
                ix / 128.0,
                (iy + 8.0) / 128.0,
            );
            t.vertex_uv(
                (x + xo + 8) as f32,
                (y + 8) as f32,
                0.0,
                (ix + 8.0) / 128.0,
                (iy + 8.0) / 128.0,
            );
            t.vertex_uv(
                (x + xo + 8) as f32,
                y as f32,
                0.0,
                (ix + 8.0) / 128.0,
                iy / 128.0,
            );
            t.vertex_uv((x + xo) as f32, y as f32, 0.0, ix / 128.0, iy / 128.0);

            xo += self.char_widths[c as usize] + 1;
            i += 1;
        }

        t.end();
        unsafe {
            gl::Disable(gl::TEXTURE_2D);
        }
    }

    #[must_use]
    pub fn width(&self, s: &str) -> JInt {
        let chars: Vec<char> = s.chars().collect();
        let mut len = 0;
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '&' {
                i += 2;
                continue;
            }

            len += self.char_widths[chars[i] as usize];
            i += 1;
        }

        len
    }
}
