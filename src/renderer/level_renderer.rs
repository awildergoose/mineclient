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
        frustum::Frustum,
        level::Level,
        level_listener::LevelListener,
        tile::tile::{Tile, TileTrait, get_tile},
    },
    player::Player,
    renderer::{tesselator::Tesselator, textures::Textures},
};

pub static CHUNK_SIZE: JInt = 16;
pub static MAX_REBUILDS_PER_FRAME: JInt = 4;

pub struct LevelRenderer {
    level: Rc<RefCell<Level>>,
    textures: Rc<RefCell<Textures>>,
    chunks: Vec<Chunk>,
    sorted_chunks: Vec<Chunk>,
    x_chunks: JInt,
    y_chunks: JInt,
    z_chunks: JInt,
    surround_lists: u32,
    draw_distance: JInt,
    l_x: JFloat,
    l_y: JFloat,
    l_z: JFloat,
    pub t: Rc<RefCell<Tesselator>>,
}

impl LevelRenderer {
    pub fn new(level: Rc<RefCell<Level>>, textures: Rc<RefCell<Textures>>) -> Rc<RefCell<Self>> {
        let t = Rc::new(RefCell::new(Tesselator::new()));

        let renderer = Rc::new(RefCell::new(Self {
            level: level.clone(),
            textures,
            t,
            surround_lists: unsafe { gl::GenLists(2) },
            x_chunks: 0,
            y_chunks: 0,
            z_chunks: 0,
            chunks: vec![],
            sorted_chunks: vec![],
            draw_distance: 0,
            l_x: 0.0,
            l_y: 0.0,
            l_z: 0.0,
        }));

        let adapter = LevelRendererListener {
            renderer: Rc::downgrade(&renderer),
        };
        adapter.all_changed();

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

    pub fn render(&mut self, player: &Player, layer: u32) {
        unsafe {
            gl::Enable(3553);
            let id = self.textures.borrow_mut().load_texture("terrain.png", 9728);
            gl::BindTexture(3553, id);
        };

        let xd = player.x - self.l_x;
        let yd = player.y - self.l_y;
        let zd = player.z - self.l_z;
        if zd.mul_add(zd, xd.mul_add(xd, yd * yd)) > 64.0 {
            self.l_x = player.x;
            self.l_y = player.y;
            self.l_z = player.z;
            // Arrays.sort(this.sortedChunks, new DistanceSorter(player));
        }

        for chunk in &mut self.sorted_chunks {
            if chunk.visible {
                let dd = (256 / (1 << self.draw_distance)) as f32;

                if self.draw_distance == 0 || chunk.distance_to_sqr(player) < dd * dd {
                    chunk.render(layer);
                }
            }
        }

        unsafe {
            gl::Disable(3553);
        }
    }

    pub fn render_surrounding_ground(&self) {
        unsafe {
            gl::CallList(self.surround_lists);
        }
    }

    pub fn compile_surrounding_ground(&self) {
        unsafe {
            gl::Enable(3553);
            gl::BindTexture(3553, self.textures.borrow().load_texture("rock.png", 9728));
            gl::Color4f(1.0, 1.0, 1.0, 1.0);
        }
        let mut t = self.t.borrow_mut();
        let y = { self.level.borrow().get_ground_level() - 2.0 };
        let (lv_width, lv_height) = {
            let lv = self.level.borrow();
            (lv.width, lv.height)
        };
        let mut s = 128;
        if s > lv_width {
            s = lv_width;
        }
        if s > lv_height {
            s = lv_height;
        }
        let d = 5;
        t.begin();

        for xx in (-s * d..lv_width + s * d).step_by(s as usize) {
            for zz in (-s * d..lv_height + s * d).step_by(s as usize) {
                let yy = if xx >= 0 && zz >= 0 && xx < lv_width && zz < lv_height {
                    0.0
                } else {
                    y
                };

                t.vertex_uv(xx as f32, yy, (zz + s) as f32, 0.0, s as f32);
                t.vertex_uv((xx + s) as f32, yy, (zz + s) as f32, s as f32, s as f32);
                t.vertex_uv((xx + s) as f32, yy, zz as f32, s as f32, 0.0);
                t.vertex_uv(xx as f32, yy, zz as f32, 0.0, 0.0);
            }
        }

        t.end();
        unsafe {
            gl::BindTexture(3553, self.textures.borrow().load_texture("rock.png", 9728));
            gl::Color3f(0.8, 0.8, 0.8);
        }
        t.begin();

        for xx in (0..lv_width).step_by(s as usize) {
            t.vertex_uv(xx as f32, 0.0, 0.0, 0.0, 0.0);
            t.vertex_uv((xx + s) as f32, 0.0, 0.0, s as f32, 0.0);
            t.vertex_uv((xx + s) as f32, y, 0.0, s as f32, y);
            t.vertex_uv(xx as f32, y, 0.0, 0.0, y);
            t.vertex_uv(xx as f32, y, lv_height as f32, 0.0, y);
            t.vertex_uv((xx + s) as f32, y, lv_height as f32, s as f32, y);
            t.vertex_uv((xx + s) as f32, 0.0, lv_height as f32, s as f32, 0.0);
            t.vertex_uv(xx as f32, 0.0, lv_height as f32, 0.0, 0.0);
        }

        unsafe {
            gl::Color3f(0.6, 0.6, 0.6);
        }

        for zz in (0..lv_height).step_by(s as usize) {
            t.vertex_uv(0.0, y, zz as f32, 0.0, 0.0);
            t.vertex_uv(0.0, y, (zz + s) as f32, s as f32, 0.0);
            t.vertex_uv(0.0, 0.0, (zz + s) as f32, s as f32, y);
            t.vertex_uv(0.0, 0.0, zz as f32, 0.0, y);
            t.vertex_uv(lv_width as f32, 0.0, zz as f32, 0.0, y);
            t.vertex_uv(lv_width as f32, 0.0, (zz + s) as f32, s as f32, y);
            t.vertex_uv(lv_width as f32, y, (zz + s) as f32, s as f32, 0.0);
            t.vertex_uv(lv_width as f32, y, zz as f32, 0.0, 0.0);
        }

        t.end();
        unsafe {
            gl::Disable(3042);
            gl::Disable(3553);
        }
    }

    pub fn render_surrounding_water(&self) {
        unsafe {
            gl::CallList(self.surround_lists + 1);
        }
    }

    pub fn compile_surrounding_water(&self) {
        unsafe {
            gl::Enable(3553);
            gl::Color3f(1.0, 1.0, 1.0);
            gl::BindTexture(3553, self.textures.borrow().load_texture("water.png", 9728));
        }
        let y = { self.level.borrow().get_ground_level() };
        unsafe {
            gl::Enable(3042);
            gl::BlendFunc(770, 771);
        }
        let mut t = self.t.borrow_mut();
        let (lv_width, lv_height) = {
            let lv = self.level.borrow();
            (lv.width, lv.height)
        };
        let mut s = 128;
        if s > lv_width {
            s = lv_width;
        }
        if s > lv_height {
            s = lv_height;
        }
        let d = 5;
        t.begin();

        for xx in (-s * d..lv_width + s * d).step_by(s as usize) {
            for zz in (-s * d..lv_height + s * d).step_by(s as usize) {
                let yy = y - 0.1;

                if xx < 0 || zz < 0 || xx >= lv_width || zz >= lv_height {
                    t.vertex_uv(xx as f32, yy, (zz + s) as f32, 0.0, s as f32);
                    t.vertex_uv((xx + s) as f32, yy, (zz + s) as f32, s as f32, s as f32);
                    t.vertex_uv((xx + s) as f32, yy, zz as f32, s as f32, 0.0);
                    t.vertex_uv(xx as f32, yy, zz as f32, 0.0, 0.0);
                    t.vertex_uv(xx as f32, yy, zz as f32, 0.0, 0.0);
                    t.vertex_uv((xx + s) as f32, yy, zz as f32, s as f32, 0.0);
                    t.vertex_uv((xx + s) as f32, yy, (zz + s) as f32, s as f32, s as f32);
                    t.vertex_uv(xx as f32, yy, (zz + s) as f32, 0.0, s as f32);
                }
            }
        }

        t.end();
        unsafe {
            gl::Disable(3042);
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
        let r = 2.5;
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
                        && tile.may_pick()
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

                            t.begin();
                            tile.render_face_no_texture(player, &mut t, x, y, z, i as i32);
                            t.end();
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

    pub fn render_hit(&mut self, player: &Player, h: &HitResult, mode: JInt, tile_type: JInt) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Enable(gl::ALPHA_TEST);
            gl::BlendFunc(gl::SRC_ALPHA, 1);
            gl::Color4f(
                1.0,
                1.0,
                1.0,
                (f32::sin(get_milli_time() as JFloat / 100.0) * 0.2 + 0.4) * 0.5,
            );
        }
        let mut t = self.t.borrow_mut();
        if mode == 0 {
            t.begin();
            Tile::ROCK.render_face_no_texture(player, &mut t, h.x, h.y, h.z, h.f);
            t.end();
        } else {
            unsafe {
                gl::BlendFunc(gl::SRC_ALPHA, 771);
                let br = f32::sin(get_milli_time() as f32 / 100.0) * 0.2 + 0.8;
                gl::Color4f(
                    br,
                    br,
                    br,
                    f32::sin(get_milli_time() as f32 / 200.0) * 0.2 + 0.5,
                );
                gl::Enable(3553);
                let id = self.textures.borrow_mut().load_texture("terrain.png", 9728);
                gl::BindTexture(3553, id);

                let mut x = h.x;
                let mut y = h.y;
                let mut z = h.z;

                if h.f == 0 {
                    y -= 1;
                }

                if h.f == 1 {
                    y += 1;
                }

                if h.f == 2 {
                    z -= 1;
                }

                if h.f == 3 {
                    z += 1;
                }

                if h.f == 4 {
                    x -= 1;
                }

                if h.f == 5 {
                    x += 1;
                }

                t.begin();
                t.no_color();
                get_tile(tile_type)
                    .unwrap()
                    .render(&mut t, &self.level.borrow(), 0, x, y, z);
                get_tile(tile_type)
                    .unwrap()
                    .render(&mut t, &self.level.borrow(), 1, x, y, z);
                t.end();
                gl::Disable(3553);
            }
        }

        unsafe {
            gl::Disable(gl::BLEND);
            gl::Disable(gl::ALPHA_TEST);
        }
    }

    pub fn render_hit_outline(
        &mut self,
        _player: &Player,
        h: &HitResult,
        mode: JInt,
        _tile_type: JInt,
    ) {
        unsafe {
            gl::Enable(3042);
            gl::BlendFunc(770, 771);
            gl::Color4f(0.0, 0.0, 0.0, 0.4);
        }

        let mut x = h.x as f32;
        let mut y = h.y as f32;
        let mut z = h.z as f32;

        if mode == 1 {
            if h.f == 0 {
                y -= 1.0;
            }

            if h.f == 1 {
                y += 1.0;
            }

            if h.f == 2 {
                z -= 1.0;
            }

            if h.f == 3 {
                z += 1.0;
            }

            if h.f == 4 {
                x -= 1.0;
            }

            if h.f == 5 {
                x += 1.0;
            }
        }

        unsafe {
            gl::Begin(3);
            gl::Vertex3f(x, y, z);
            gl::Vertex3f(x + 1.0, y, z);
            gl::Vertex3f(x + 1.0, y, z + 1.0);
            gl::Vertex3f(x, y, z + 1.0);
            gl::Vertex3f(x, y, z);
            gl::End();
            gl::Begin(3);
            gl::Vertex3f(x, y + 1.0, z);
            gl::Vertex3f(x + 1.0, y + 1.0, z);
            gl::Vertex3f(x + 1.0, y + 1.0, z + 1.0);
            gl::Vertex3f(x, y + 1.0, z + 1.0);
            gl::Vertex3f(x, y + 1.0, z);
            gl::End();
            gl::Begin(1);
            gl::Vertex3f(x, y, z);
            gl::Vertex3f(x, y + 1.0, z);
            gl::Vertex3f(x + 1.0, y, z);
            gl::Vertex3f(x + 1.0, y + 1.0, z);
            gl::Vertex3f(x + 1.0, y, z + 1.0);
            gl::Vertex3f(x + 1.0, y + 1.0, z + 1.0);
            gl::Vertex3f(x, y, z + 1.0);
            gl::Vertex3f(x, y + 1.0, z + 1.0);
            gl::End();
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

    pub const fn toggle_draw_distance(&mut self) {
        self.draw_distance = (self.draw_distance + 1) % 4;
    }

    pub fn cull(&mut self, frustum: &Frustum) {
        for chunk in &mut self.chunks {
            chunk.visible = frustum.is_visible(&chunk.aabb);
        }
    }

    pub fn on_all_changed(&mut self) {
        self.l_x = -900_000.0;
        self.l_y = -900_000.0;
        self.l_z = -900_000.0;
        let (lv_width, lv_depth, lv_height) = {
            let level = self.level.borrow();
            (level.width, level.depth, level.height)
        };
        self.x_chunks = (lv_width + 16 - 1) / 16;
        self.y_chunks = (lv_depth + 16 - 1) / 16;
        self.z_chunks = (lv_height + 16 - 1) / 16;
        self.chunks = vec![];
        self.sorted_chunks = vec![];

        for x in 0..self.x_chunks {
            for y in 0..self.y_chunks {
                for z in 0..self.z_chunks {
                    let x0 = x * 16;
                    let y0 = y * 16;
                    let z0 = z * 16;
                    let mut x1 = (x + 1) * 16;
                    let mut y1 = (y + 1) * 16;
                    let mut z1 = (z + 1) * 16;

                    if x1 > lv_width {
                        x1 = lv_width;
                    }
                    if y1 > lv_depth {
                        y1 = lv_depth;
                    }
                    if z1 > lv_height {
                        z1 = lv_height;
                    }

                    // TODO: don't clone Chunk
                    let c = Chunk::new(self.level.clone(), self.t.clone(), x0, y0, z0, x1, y1, z1);
                    self.chunks.push(c.clone());
                    self.sorted_chunks.push(c);
                }
            }
        }

        unsafe {
            gl::NewList(self.surround_lists, 4864);
            self.compile_surrounding_ground();
            gl::EndList();
            gl::NewList(self.surround_lists + 1, 4864);
            self.compile_surrounding_water();
            gl::EndList();
        }

        for chunk in &mut self.chunks {
            chunk.reset();
        }
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
