use std::{
    cell::RefCell,
    cmp::Ordering,
    rc::{Rc, Weak},
};

use crate::{
    gl,
    hit_result::HitResult,
    java::{JFloat, JInt, get_milli_time},
    level::{
        chunk::Chunk,
        frustum::{self, Frustum},
        level::Level,
        level_listener::LevelListener,
        tile::tile::{Tile, TileTrait, get_tile},
    },
    player::Player,
    renderer::{tesselator::Tesselator, textures},
};

pub static CHUNK_SIZE: JInt = 16;
pub static MAX_REBUILDS_PER_FRAME: JInt = 8;

type LevelRef = Rc<RefCell<Level>>;

pub struct LevelRenderer {
    level: LevelRef,
    chunks: Vec<Chunk>,
    x_chunks: JInt,
    y_chunks: JInt,
    z_chunks: JInt,
    pub t: Rc<RefCell<Tesselator>>,
}

impl LevelRenderer {
    pub fn new(level: LevelRef) -> Rc<RefCell<Self>> {
        let width = level.borrow().width;
        let height = level.borrow().height;
        let depth = level.borrow().depth;

        let x_chunks = width / 16;
        let y_chunks = depth / 16;
        let z_chunks = height / 16;
        let mut chunks = Vec::new();

        let t = Rc::new(RefCell::new(Tesselator::new()));

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

                    chunks.push(Chunk::new(level.clone(), t.clone(), x0, y0, z0, x1, y1, z1));
                }
            }
        }

        let renderer = Rc::new(RefCell::new(Self {
            level: level.clone(),
            chunks,
            x_chunks,
            y_chunks,
            z_chunks,
            t: t.clone(),
        }));

        let adapter = LevelRendererListener {
            renderer: Rc::downgrade(&renderer),
        };

        level.borrow_mut().add_listener(Box::new(adapter));

        renderer
    }

    pub fn get_all_dirty_chunks(&mut self) -> Vec<&mut Chunk> {
        let mut dirty = Vec::new();

        for c in &mut self.chunks {
            if c.is_dirty() {
                dirty.push(c);
            }
        }

        dirty
    }

    pub fn render(&mut self, _player: &Player, layer: u32) {
        unsafe {
            gl::Enable(3553);
            let id = textures::load_2d_texture("terrain.png");
            gl::BindTexture(3553, id);
        };

        let frustum = frustum::get_frustum();

        for c in &mut self.chunks {
            if frustum.lock().unwrap().is_visible(&c.aabb) {
                c.render(layer);
            }
        }

        unsafe {
            gl::Disable(3553);
        }
    }

    pub fn update_dirty_chunks(&mut self, frustum: &Frustum, player: &Player) {
        let mut dirty = self.get_all_dirty_chunks();
        if dirty.is_empty() {
            return;
        }

        let now = get_milli_time();

        dirty.sort_by(|c0, c1| {
            let i0 = frustum.is_visible(&c0.aabb);
            let i1 = frustum.is_visible(&c1.aabb);

            if i0 && !i1 {
                return Ordering::Less;
            } else if i1 && !i0 {
                return Ordering::Greater;
            }

            let t0 = ((now - c0.dirtied_time) / 2000) as i32;
            let t1 = ((now - c1.dirtied_time) / 2000) as i32;
            if t0 < t1 {
                return Ordering::Less;
            } else if t0 > t1 {
                return Ordering::Greater;
            }

            let d0 = c0.distance_to_sqr(player);
            let d1 = c1.distance_to_sqr(player);
            if d0 < d1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        for c in dirty.into_iter().take(MAX_REBUILDS_PER_FRAME as usize) {
            c.rebuild_all();
        }
    }

    pub fn pick(&mut self, player: &Player, frustum: &Frustum) {
        let mut t = self.t.borrow_mut();
        let r = 3.0;
        let pbox = player.bb.grow(r, r, r);
        let x0 = pbox.x0 as JInt;
        let x1 = pbox.x1 as JInt;
        let y0 = pbox.y0 as JInt;
        let y1 = pbox.y1 as JInt;
        let z0 = pbox.z0 as JInt;
        let z1 = pbox.z1 as JInt;
        unsafe {
            gl::InitNames();
            gl::PushName(0);
            gl::PushName(0);
        };

        for x in x0..x1 {
            unsafe {
                gl::LoadName(x as u32);
                gl::PushName(0);
            }

            for y in y0..y1 {
                unsafe {
                    gl::LoadName(y as u32);
                    gl::PushName(0);
                }

                for z in z0..z1 {
                    if let Some(tile) = get_tile(self.level.borrow_mut().get_tile(x, y, z))
                        && frustum.is_visible(&tile.get_tile_aabb(x, y, z))
                    {
                        unsafe {
                            gl::LoadName(z as u32);
                            gl::PushName(0);
                        }

                        for i in 0..6 {
                            unsafe {
                                gl::LoadName(i);
                            }

                            t.init();
                            tile.render_face_no_texture(&mut t, x, y, z, i as i32);
                            t.flush();
                        }

                        unsafe {
                            gl::PopName();
                        }
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
            gl::PopName();
        }
    }

    pub fn render_hit(&mut self, h: &HitResult) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, 1);
            gl::Color4f(
                1.0,
                1.0,
                1.0,
                (f32::sin(get_milli_time() as JFloat / 100.0) * 0.2 + 0.4) * 0.5,
            );
        }
        let mut t = self.t.borrow_mut();
        t.init();
        Tile::ROCK.render_face_no_texture(&mut t, h.x, h.y, h.z, h.f);
        t.flush();
        unsafe {
            gl::Disable(gl::BLEND);
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
        x0 /= 16;
        x1 /= 16;
        y0 /= 16;
        y1 /= 16;
        z0 /= 16;
        z1 /= 16;

        if x0 < 0 {
            x0 = 0;
        }

        if y0 < 0 {
            y0 = 0;
        }

        if z0 < 0 {
            z0 = 0;
        }

        if x1 >= self.x_chunks {
            x1 = self.x_chunks - 1;
        }
        if y1 >= self.y_chunks {
            y1 = self.y_chunks - 1;
        }
        if z1 >= self.z_chunks {
            z1 = self.z_chunks - 1;
        }

        if x1 < x0 || y1 < y0 || z1 < z0 {
            return;
        }

        for x in x0..=x1 {
            for y in y0..=y1 {
                for z in z0..=z1 {
                    let idx = ((x * self.y_chunks + y) * self.z_chunks + z) as usize;
                    self.chunks[idx].set_dirty();
                }
            }
        }
    }

    pub fn on_tile_changed(&mut self, x: JInt, y: JInt, z: JInt) {
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
