use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicI32, AtomicI64, Ordering},
};

use crate::{
    gl,
    java::{JBoolean, JFloat, JInt, JLong, get_milli_time, get_nano_time},
    level::{
        level::Level,
        tesselator::Tesselator,
        tile::tile::{TileTrait, get_tile},
    },
    phys::aabb::AABB,
    player::Player,
};

pub static UPDATES: AtomicI32 = AtomicI32::new(0);
pub static TOTAL_TIME: AtomicI64 = AtomicI64::new(0);
pub static TOTAL_UPDATES: AtomicI32 = AtomicI32::new(0);

pub struct Chunk {
    pub aabb: AABB,
    pub level: Rc<RefCell<Level>>,
    pub x0: JInt,
    pub y0: JInt,
    pub z0: JInt,
    pub x1: JInt,
    pub y1: JInt,
    pub z1: JInt,
    pub x: JFloat,
    pub y: JFloat,
    pub z: JFloat,
    pub dirtied_time: JLong,
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
            x: (x0 + x1) as JFloat / 2.0,
            y: (y0 + y1) as JFloat / 2.0,
            z: (z0 + z1) as JFloat / 2.0,
            dirtied_time: 0,
            dirty: true,
            lists: unsafe { gl::GenLists(2) },
        }
    }

    pub fn rebuild_all(&mut self) {
        self.rebuild(0);
        self.rebuild(1);
    }

    pub fn rebuild(&mut self, layer: u32) {
        self.dirty = false;
        UPDATES.fetch_add(1, Ordering::SeqCst);

        let before = get_nano_time();
        unsafe { gl::NewList(self.lists + layer, gl::COMPILE) }
        let mut t = self.t.borrow_mut();
        t.init();

        let mut tiles = 0;
        let level = self.level.borrow_mut();

        for x in self.x0..self.x1 {
            for y in self.y0..self.y1 {
                for z in self.z0..self.z1 {
                    let tile_id = level.get_tile(x, y, z);
                    if tile_id > 0 {
                        get_tile(tile_id)
                            .unwrap()
                            .render(&mut t, &level, layer as i32, x, y, z);
                        tiles += 1;
                    }
                }
            }
        }

        t.flush();

        unsafe {
            gl::EndList();
        }

        let after = get_nano_time();
        if tiles > 0 {
            TOTAL_TIME.fetch_add(after - before, Ordering::SeqCst);
            TOTAL_UPDATES.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn render(&mut self, layer: u32) {
        unsafe { gl::CallList(self.lists + layer) };
    }

    pub fn set_dirty(&mut self) {
        if !self.dirty {
            self.dirtied_time = get_milli_time();
        }

        self.dirty = true;
    }

    pub fn is_dirty(&self) -> JBoolean {
        self.dirty
    }

    pub fn distance_to_sqr(&self, player: &Player) -> JFloat {
        let xd = player.x - self.x;
        let yd = player.y - self.y;
        let zd = player.z - self.z;
        xd * zd + yd * yd + zd * zd
    }
}
