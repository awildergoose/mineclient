use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use javarandom::JavaRandom;
use std::io::Write;

use crate::{
    java::{JBoolean, JByte, JInt},
    level::{level_gen::LevelGen, level_listener::LevelListener, tile::tile::get_tile},
    phys::aabb::AABB,
};

pub struct Level {
    pub width: JInt,
    pub height: JInt,
    pub depth: JInt,
    blocks: Vec<JByte>,
    light_depths: Vec<JInt>,
    level_listeners: Vec<Box<dyn LevelListener>>,
    pub random: Rc<RefCell<JavaRandom>>,
    unprocessed: JInt,
}

impl Level {
    #[must_use]
    pub fn new(w: JInt, h: JInt, d: JInt) -> Self {
        let w: usize = w as usize;
        let h: usize = h as usize;
        let d: usize = d as usize;

        let mut this = Self {
            width: w as JInt,
            height: h as JInt,
            depth: d as JInt,
            blocks: vec![0i8; w * h * d],
            light_depths: vec![0i32; w * h],
            level_listeners: Vec::new(),
            random: Rc::new(RefCell::new(JavaRandom::with_seed(69))),
            unprocessed: 0,
        };

        if let Err(err) = this.load() {
            eprintln!("failed to load level: {err:?}");
            this.blocks =
                LevelGen::new(this.random.clone(), w as JInt, h as JInt, d as JInt).generate_map();
        }

        this.calc_light_depths(0, 0, w as JInt, h as JInt);

        this
    }

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        let file = File::open("level.dat")?;
        let mut decoder = GzDecoder::new(file);

        let i8_slice: &mut [i8] = self.blocks.as_mut_slice();
        let u8_slice: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(i8_slice.as_mut_ptr().cast::<u8>(), i8_slice.len())
        };

        decoder.read_exact(u8_slice)?;

        for listener in &self.level_listeners {
            listener.all_changed();
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let file = File::create("level.dat")?;
        let mut encoder = GzEncoder::new(file, Compression::default());

        let i8_slice: &[i8] = self.blocks.as_slice();
        let u8_slice: &[u8] =
            unsafe { std::slice::from_raw_parts(i8_slice.as_ptr().cast::<u8>(), i8_slice.len()) };

        encoder.write_all(u8_slice)?;
        encoder.finish()?;

        Ok(())
    }

    pub fn calc_light_depths(&mut self, x0: JInt, y0: JInt, x1: JInt, y1: JInt) {
        for x in x0..x0 + x1 {
            for z in y0..y0 + y1 {
                let old_depth = self.light_depths[(x + z * self.width) as usize];
                let mut y = self.depth - 1;

                while y > 0 && !self.is_light_blocker(x, y, z) {
                    y -= 1;
                }

                self.light_depths[(x + z * self.width) as usize] = y;

                if old_depth != y {
                    let yl0 = if old_depth < y { old_depth } else { y };
                    let yl1 = if old_depth > y { old_depth } else { y };

                    for l in &self.level_listeners {
                        l.light_column_changed(x, z, yl0, yl1);
                    }
                }
            }
        }
    }

    pub fn add_listener(&mut self, level_listener: Box<dyn LevelListener>) {
        self.level_listeners.push(level_listener);
    }

    pub fn remove_listener(&mut self, level_listener: &dyn LevelListener) {
        if let Some(pos) = self
            .level_listeners
            .iter()
            .position(|p| std::ptr::eq(&raw const **p, level_listener))
        {
            self.level_listeners.remove(pos);
        }
    }

    #[must_use]
    pub fn is_light_blocker(&self, x: JInt, y: JInt, z: JInt) -> JBoolean {
        get_tile(self.get_tile(x, y, z)).is_some_and(super::tile::tile::TileTrait::blocks_light)
    }

    #[must_use]
    pub fn get_cubes(&self, aabb: AABB) -> Vec<AABB> {
        let mut aabbs = Vec::new();
        let mut x0 = aabb.x0 as JInt;
        let mut x1 = aabb.x1 as JInt + 1;
        let mut y0 = aabb.y0 as JInt;
        let mut y1 = aabb.y1 as JInt + 1;
        let mut z0 = aabb.z0 as JInt;
        let mut z1 = aabb.z1 as JInt + 1;

        if x0 < 0 {
            x0 = 0;
        }

        if y0 < 0 {
            y0 = 0;
        }

        if z0 < 0 {
            z0 = 0;
        }

        if x1 > self.width {
            x1 = self.width;
        }

        if y1 > self.depth {
            y1 = self.depth;
        }

        if z1 > self.height {
            z1 = self.height;
        }

        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    if let Some(tile) = get_tile(self.get_tile(x, y, z))
                        && let Some(aabb) = tile.get_aabb(x, y, z)
                    {
                        aabbs.push(aabb);
                    }
                }
            }
        }

        aabbs
    }

    pub fn set_tile(&mut self, x: JInt, y: JInt, z: JInt, type_: JInt) -> JBoolean {
        let width = self.width;
        let height = self.height;

        if x >= 0 && y >= 0 && z >= 0 && x < self.width && y < self.depth && z < self.height {
            if type_ == JInt::from(self.blocks[((y * self.height + z) * self.width + x) as usize]) {
                false
            } else {
                self.blocks[((y * height + z) * width + x) as usize] = type_ as JByte;
                self.calc_light_depths(x, z, 1, 1);

                for ele in &self.level_listeners {
                    ele.tile_changed(x, y, z);
                }

                true
            }
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_lit(&self, x: JInt, y: JInt, z: JInt) -> JBoolean {
        if x < 0 || y < 0 || z < 0 || x >= self.width || y >= self.depth || z >= self.height {
            true
        } else {
            y >= self.light_depths[(x + z * self.width) as usize]
        }
    }

    #[must_use]
    pub fn get_tile(&self, x: JInt, y: JInt, z: JInt) -> JInt {
        if x >= 0 && y >= 0 && z >= 0 && x < self.width && y < self.depth && z < self.height {
            unsafe {
                JInt::from(
                    *self
                        .blocks
                        .get_unchecked(((y * self.height + z) * self.width + x) as usize),
                )
            }
        } else {
            0
        }
    }

    #[must_use]
    pub fn is_solid_tile(&self, x: JInt, y: JInt, z: JInt) -> JBoolean {
        get_tile(self.get_tile(x, y, z)).is_some_and(super::tile::tile::TileTrait::is_solid)
    }

    pub fn tick(&mut self) {
        self.unprocessed += self.width * self.height * self.depth;
        let ticks = self.unprocessed / 400;
        self.unprocessed -= ticks * 400;

        let w = self.width as u32;
        let d = self.depth as u32;
        let h = self.height as u32;

        for _ in 0..ticks {
            let (x, y, z) = {
                let r = &mut self.random.borrow_mut();
                (
                    r.next_int_with_bound(w),
                    r.next_int_with_bound(d),
                    r.next_int_with_bound(h),
                )
            };
            let tile_id = self.get_tile(x, y, z);

            if let Some(tile) = get_tile(tile_id) {
                tile.tick(self, x, y, z);
            }
        }
    }
}
