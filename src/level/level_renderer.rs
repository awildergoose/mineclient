use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::atomic::Ordering,
};

use crate::{
    java::JInt,
    level::{
        chunk::{self, Chunk},
        level::Level,
        level_listener::LevelListener,
        tesselator::Tesselator,
    },
};

pub static CHUNK_SIZE: JInt = 16;

type LevelRef = Rc<RefCell<Level>>;

pub struct LevelRenderer {
    level: LevelRef,
    chunks: Vec<Chunk>,
    x_chunks: JInt,
    y_chunks: JInt,
    z_chunks: JInt,
    t: Tesselator,
}

impl LevelRenderer {
    pub fn new(level: LevelRef) -> Rc<RefCell<Self>> {
        let x_chunks = level.borrow().width / 16;
        let y_chunks = level.borrow().depth / 16;
        let z_chunks = level.borrow().height / 16;
        let mut chunks = Vec::with_capacity((x_chunks * y_chunks * z_chunks) as usize);

        let width = level.borrow().width;
        let height = level.borrow().height;
        let depth = level.borrow().depth;

        for x in 0..x_chunks {
            for y in 0..y_chunks {
                for z in 0..z_chunks {
                    let x0 = x * 16;
                    let y0 = y * 16;
                    let z0 = z * 16;
                    let mut x1 = (x + 1) * 16;
                    let mut y1 = (y + 1) * 16;
                    let mut z1 = (z + 1) * 16;

                    if x1 > width {
                        x1 = width;
                    }
                    if y1 > depth {
                        y1 = depth;
                    }
                    if z1 > height {
                        z1 = height;
                    }

                    let idx = (x as usize + (y as usize) * (x_chunks as usize))
                        * (z_chunks as usize)
                        + (z as usize);

                    chunks[idx] = Chunk::new(level.clone(), x0, y0, z0, x1, y1, z1);
                }
            }
        }

        let renderer = Rc::new(RefCell::new(Self {
            level: level.clone(),
            chunks,
            x_chunks,
            y_chunks,
            z_chunks,
            t: Tesselator::new(),
        }));

        let adapter = LevelRendererListener {
            renderer: Rc::downgrade(&renderer),
        };

        level.borrow_mut().add_listener(Box::new(adapter));

        renderer
    }

    // pub fn render(&mut self, player: Player, layer: JInt) {
    //     chunk::REBUILT_THIS_FRAME.store(0, Ordering::SeqCst);
    //     // TODO
    // }

    pub fn on_tile_changed(&self, a: JInt, b: JInt, c: JInt) {
        // TODO: actual logic; e.g. mark chunks dirty, etc.
    }

    pub fn on_light_column_changed(&self, a: JInt, b: JInt, c: JInt, d: JInt) {
        // TODO
    }

    pub fn on_all_changed(&self) {
        // TODO
    }
}

struct LevelRendererListener {
    renderer: Weak<RefCell<LevelRenderer>>,
}

impl LevelListener for LevelRendererListener {
    fn tile_changed(&self, a: JInt, b: JInt, c: JInt) {
        if let Some(rc) = self.renderer.upgrade() {
            rc.borrow().on_tile_changed(a, b, c);
        }
    }

    fn light_column_changed(&self, a: JInt, b: JInt, c: JInt, d: JInt) {
        if let Some(rc) = self.renderer.upgrade() {
            rc.borrow().on_light_column_changed(a, b, c, d);
        }
    }

    fn all_changed(&self) {
        if let Some(rc) = self.renderer.upgrade() {
            rc.borrow().on_all_changed();
        }
    }
}
