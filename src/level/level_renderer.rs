use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::atomic::Ordering,
};

use crate::{
    gl,
    hit_result::HitResult,
    java::{JFloat, JInt, get_milli_time},
    level::{
        chunk::{self, Chunk},
        frustum,
        level::Level,
        level_listener::LevelListener,
        tesselator::Tesselator,
        tile::Tile,
    },
    player::Player,
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
        let mut chunks = Vec::new();

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

                    chunks.push(Chunk::new(level.clone(), x0, y0, z0, x1, y1, z1));
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

    pub fn render(&mut self, _player: &Player, layer: u32) {
        chunk::REBUILT_THIS_FRAME.store(0, Ordering::SeqCst);
        let frustum = frustum::get_frustum();

        for c in &mut self.chunks {
            if frustum.lock().unwrap().aabb_in_frustum(&c.aabb) {
                c.render(layer);
            }
        }
    }

    pub fn pick(&mut self, player: &Player) {
        let r = 3.0;
        let pbox = player.bb.grow(r, r, r);
        let x0 = pbox.x0 as JInt;
        let x1 = pbox.x1 as JInt;
        let y0 = pbox.y0 as JInt;
        let y1 = pbox.y1 as JInt;
        let z0 = pbox.z0 as JInt;
        let z1 = pbox.z1 as JInt;
        unsafe { gl::InitNames() };

        for x in x0..x1 {
            unsafe {
                gl::PushName(x as u32);
            }

            for y in y0..y1 {
                unsafe {
                    gl::PushName(y as u32);
                }

                for z in z0..z1 {
                    unsafe {
                        gl::PushName(z as u32);
                    }

                    if self.level.borrow().is_solid_tile(x, y, z) {
                        unsafe {
                            gl::PushName(0);
                        }

                        for i in 0..6 {
                            unsafe {
                                gl::PushName(i as u32);
                            }
                            self.t.init();
                            Tile::ROCK.render_face(&mut self.t, x, y, z, i);
                            self.t.flush();
                            unsafe {
                                gl::PopName();
                            }
                        }

                        unsafe {
                            gl::PopName();
                        }
                    }

                    unsafe {
                        gl::PopName();
                    }
                }

                unsafe {
                    gl::PopName();
                }
            }

            unsafe {
                gl::PopName();
            }
        }
    }

    pub fn render_hit(&mut self, h: &HitResult) {
        unsafe {
            gl::Enable(3042);
            gl::BlendFunc(770, 1);
            gl::Color4f(
                1.0,
                1.0,
                1.0,
                f32::sin(get_milli_time() as JFloat / 100.0) * 0.2 + 0.4,
            );
        }
        self.t.init();
        Tile::ROCK.render_face(&mut self.t, h.x, h.y, h.z, h.f);
        self.t.flush();
        unsafe {
            gl::Disable(3042);
        }
    }

    pub fn set_dirty(
        &mut self,
        mut x0: JInt,
        mut y0: JInt,
        mut z0: JInt,
        mut x1: JInt,
        mut y1: JInt,
        mut z1: JInt,
    ) {
        fn floor_div(a: JInt, b: JInt) -> JInt {
            let q = a / b;
            let r = a % b;
            if r != 0 && ((r > 0) != (b > 0)) {
                q - 1
            } else {
                q
            }
        }

        x0 = floor_div(x0, 16);
        x1 = floor_div(x1, 16);
        y0 = floor_div(y0, 16);
        y1 = floor_div(y1, 16);
        z0 = floor_div(z0, 16);
        z1 = floor_div(z1, 16);

        if x0 < 0 {
            x0 = 0
        }
        if y0 < 0 {
            y0 = 0
        }
        if z0 < 0 {
            z0 = 0
        }

        if x1 >= self.x_chunks {
            x1 = self.x_chunks - 1
        }
        if y1 >= self.y_chunks {
            y1 = self.y_chunks - 1
        }
        if z1 >= self.z_chunks {
            z1 = self.z_chunks - 1
        }

        if x1 < x0 || y1 < y0 || z1 < z0 {
            return;
        }

        for x in x0..=x1 {
            for y in y0..=y1 {
                for z in z0..=z1 {
                    self.chunks[((x + y * self.x_chunks) * self.z_chunks + z) as usize].set_dirty();
                }
            }
        }
    }

    pub fn on_tile_changed(&mut self, x: JInt, y: JInt, z: JInt) {
        println!("set dirty tile");
        self.set_dirty(x - 1, y - 1, z - 1, x + 1, y + 1, z + 1);
    }

    pub fn on_light_column_changed(&mut self, x: JInt, z: JInt, y0: JInt, y1: JInt) {
        self.set_dirty(x - 1, y0 - 1, z - 1, x + 1, y1 + 1, z + 1);
    }

    pub fn on_all_changed(&mut self) {
        let (w, d, h) = {
            let level = self.level.borrow();
            (level.width, level.depth, level.height)
        };

        self.set_dirty(0, 0, 0, w, d, h);
    }
}

struct LevelRendererListener {
    renderer: Weak<RefCell<LevelRenderer>>,
}

impl LevelListener for LevelRendererListener {
    fn tile_changed(&self, x: JInt, y: JInt, z: JInt) {
        if let Some(rc) = self.renderer.upgrade() {
            rc.borrow_mut().on_tile_changed(x, y, z);
        }
    }

    fn light_column_changed(&self, x: JInt, z: JInt, y0: JInt, y1: JInt) {
        if let Some(rc) = self.renderer.upgrade() {
            rc.borrow_mut().on_light_column_changed(x, z, y0, y1);
        }
    }

    fn all_changed(&self) {
        if let Some(rc) = self.renderer.upgrade() {
            rc.borrow_mut().on_all_changed();
        }
    }
}
