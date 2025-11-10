use std::sync::Mutex;

use crate::{
    gl,
    java::{JBoolean, JFloat, JInt},
    level::{level::Level, tesselator::Tesselator, tile::Tile},
    phys::aabb::AABB,
};

pub static CHUNK_TEXTURE: Mutex<u32> = Mutex::new(0);

pub struct Chunk {
    pub aabb: AABB,
    pub level: Level,
    pub x0: JInt,
    pub y0: JInt,
    pub z0: JInt,
    pub x1: JInt,
    pub y1: JInt,
    pub z1: JInt,
    dirty: JBoolean,
    lists: u32,
    t: Tesselator,
    rebuilt_this_frame: JInt,
    updates: JInt,
}

impl Chunk {
    pub fn new(level: Level, x0: JInt, y0: JInt, z0: JInt, x1: JInt, y1: JInt, z1: JInt) -> Self {
        Self {
            aabb: AABB::new(
                x0 as JFloat,
                y0 as JFloat,
                z0 as JFloat,
                x1 as JFloat,
                y1 as JFloat,
                z1 as JFloat,
            ),
            level,
            x0,
            y0,
            z0,
            x1,
            y1,
            z1,
            dirty: true,
            lists: unsafe { gl::GenLists(2) },
            t: Tesselator::new(),
            rebuilt_this_frame: 0,
            updates: 0,
        }
    }

    fn rebuild(&mut self, layer: u32) {
        if self.rebuilt_this_frame != 2 {
            self.dirty = false;
            self.updates += 1;
            self.rebuilt_this_frame += 1;
            unsafe {
                gl::NewList(self.lists + layer, 4864);
                gl::Enable(3553);
                gl::BindTexture(3553, *CHUNK_TEXTURE.lock().unwrap());
            }
            self.t.init();

            for x in self.x0..self.x1 {
                for y in self.y0..self.y1 {
                    for z in self.z0..self.z1 {
                        if self.level.is_tile(x, y, z) {
                            let tex = if y == self.level.depth * 2 / 3 { 0 } else { 1 };

                            if tex == 0 {
                                Tile::ROCK.render(&mut self.t, &self.level, layer as i32, x, y, z);
                            } else {
                                Tile::GRASS.render(&mut self.t, &self.level, layer as i32, x, y, z);
                            }
                        }
                    }
                }
            }

            self.t.flush();

            unsafe {
                gl::Disable(3553);
                gl::EndList();
            }
        }
    }

    pub fn render(&mut self, layer: u32) {
        if self.dirty {
            self.rebuild(0);
            self.rebuild(1);
        }

        unsafe { gl::CallList(self.lists + layer) };
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }
}
