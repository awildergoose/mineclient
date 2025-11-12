use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicI32, Ordering},
};

use crate::{
    gl,
    java::{JBoolean, JFloat, JInt},
    level::{level::Level, tesselator::Tesselator, tile::tile::Tile},
    phys::aabb::AABB,
    textures,
};

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
    t: Rc<RefCell<Tesselator>>,
}

impl Chunk {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        level: Rc<RefCell<Level>>,
        t: Rc<RefCell<Tesselator>>,
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
            t,
            x0,
            y0,
            z0,
            x1,
            y1,
            z1,
            dirty: true,
            lists: unsafe { gl::GenLists(2) },
        }
    }

    pub fn rebuild_all(&mut self) {
        self.rebuild(0);
        self.rebuild(1);
    }

    pub fn rebuild(&mut self, layer: u32) {
        if REBUILT_THIS_FRAME.load(Ordering::SeqCst) != 2 {
            self.dirty = false;
            UPDATES.fetch_add(1, Ordering::SeqCst);
            REBUILT_THIS_FRAME.fetch_add(1, Ordering::SeqCst);

            let tex_id = textures::load_2d_texture("terrain.png");

            unsafe {
                gl::NewList(self.lists + layer, gl::COMPILE);
                gl::Enable(gl::TEXTURE_2D);
                gl::BindTexture(gl::TEXTURE_2D, tex_id);
            }

            let mut t = self.t.borrow_mut();
            t.init();

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
                                    &mut t,
                                    &self.level.borrow(),
                                    layer as i32,
                                    x,
                                    y,
                                    z,
                                );
                            } else {
                                Tile::GRASS.render(
                                    &mut t,
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

            t.flush();

            unsafe {
                gl::Disable(gl::TEXTURE_2D);
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
