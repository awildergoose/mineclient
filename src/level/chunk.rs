use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        Mutex,
        atomic::{AtomicI32, Ordering},
    },
};

use crate::{
    gl,
    java::{JBoolean, JFloat, JInt},
    level::{level::Level, tesselator::Tesselator, tile::Tile},
    phys::aabb::AABB,
};

pub static CHUNK_TEXTURE: Mutex<u32> = Mutex::new(0);
pub static REBUILT_THIS_FRAME: AtomicI32 = AtomicI32::new(0);
pub static UPDATES: AtomicI32 = AtomicI32::new(0);

pub struct Chunk {
    pub aabb: AABB,
    pub level: Rc<RefCell<Level>>,
    pub x0: JInt,
    pub y0: JInt,
    pub z0: JInt,
    pub x1: JInt,
    pub y1: JInt,
    pub z1: JInt,
    dirty: JBoolean,
    lists: u32,
    t: Tesselator,
}

impl Chunk {
    pub fn new(
        level: Rc<RefCell<Level>>,
        x0: JInt,
        y0: JInt,
        z0: JInt,
        x1: JInt,
        y1: JInt,
        z1: JInt,
    ) -> Self {
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
        }
    }

    fn rebuild(&mut self, layer: u32) {
        if REBUILT_THIS_FRAME.load(Ordering::SeqCst) != 2 {
            self.dirty = false;
            UPDATES.fetch_add(1, Ordering::SeqCst);
            REBUILT_THIS_FRAME.fetch_add(1, Ordering::SeqCst);
            unsafe {
                gl::NewList(self.lists + layer, 4864);
                gl::Enable(3553);
                gl::BindTexture(3553, *CHUNK_TEXTURE.lock().unwrap());
            }
            self.t.init();

            for x in self.x0..self.x1 {
                for y in self.y0..self.y1 {
                    for z in self.z0..self.z1 {
                        if self.level.borrow().is_tile(x, y, z) {
                            let tex = if y == self.level.borrow().depth * 2 / 3 {
                                0
                            } else {
                                1
                            };

                            if tex == 0 {
                                Tile::ROCK.render(
                                    &mut self.t,
                                    &self.level.borrow(),
                                    layer as i32,
                                    x,
                                    y,
                                    z,
                                );
                            } else {
                                Tile::GRASS.render(
                                    &mut self.t,
                                    &self.level.borrow(),
                                    layer as i32,
                                    x,
                                    y,
                                    z,
                                );
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
