use std::{
    cell::RefCell,
    fs::File,
    io::{Read, Write},
    rc::Rc,
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use javarandom::JavaRandom;

use crate::{
    java::{JBoolean, JByte, JFloat, JInt, JLong},
    level::{
        level_listener::LevelListener,
        tile::tile::{get_tile, Tile, TileTrait},
    },
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
    rand_value: JInt,
    name: String,
    creator: String,
    create_time: JLong,
    unprocessed: JInt,
}

static TILE_UPDATE_INTERVAL: JInt = 200;
static LEVEL_MULTIPLIER: JInt = 1_664_525;
static LEVEL_ADDEND: JInt = 1_013_904_223;

impl Level {
    #[must_use]
    pub fn set_data(w: JInt, d: JInt, h: JInt, blocks: Vec<JByte>) -> Self {
        let wu: usize = w as usize;
        let hu: usize = h as usize;

        let mut random = JavaRandom::with_seed(69);
        let rand_value = random.next_int();
        let random = Rc::new(RefCell::new(random));
        let mut this = Self {
            width: w,
            height: h,
            depth: d,
            blocks,
            rand_value,
            name: String::new(),
            creator: String::new(),
            create_time: 0,
            light_depths: vec![0i32; wu * hu],
            level_listeners: Vec::new(),
            random,
            unprocessed: 0,
        };

        for ele in &this.level_listeners {
            ele.all_changed();
        }

        this.calc_light_depths(0, 0, w, h);

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

                self.light_depths[(x + z * self.width) as usize] = y + 1;

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
        let mut boxes = vec![];
        let x0 = f32::floor(aabb.x0) as JInt;
        let x1 = f32::floor(aabb.x1 + 1.0) as JInt;
        let y0 = f32::floor(aabb.y0) as JInt;
        let y1 = f32::floor(aabb.y1 + 1.0) as JInt;
        let z0 = f32::floor(aabb.z0) as JInt;
        let z1 = f32::floor(aabb.z1 + 1.0) as JInt;

        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    if x >= 0
                        && y >= 0
                        && z >= 0
                        && x < self.width
                        && y < self.depth
                        && z < self.height
                    {
                        let tile = get_tile(self.get_tile(x, y, z));

                        if let Some(tile) = tile {
                            let aabb = tile.get_aabb(x, y, z);

                            if let Some(aabb) = aabb {
                                boxes.push(aabb);
                            }
                        }
                    } else if x < 0 || y < 0 || z < 0 || x >= self.width || z >= self.height {
                        let aabb = Tile::UNBREAKABLE.get_aabb(x, y, z);

                        if let Some(aabb) = aabb {
                            boxes.push(aabb);
                        }
                    }
                }
            }
        }

        boxes
    }

    pub fn set_tile(&mut self, x: JInt, y: JInt, z: JInt, type_: JInt) -> JBoolean {
        let width = self.width;
        let height = self.height;

        if x >= 0 && y >= 0 && z >= 0 && x < self.width && y < self.depth && z < self.height {
            if type_ == JInt::from(self.blocks[((y * self.height + z) * self.width + x) as usize]) {
                false
            } else {
                self.blocks[((y * height + z) * width + x) as usize] = type_ as JByte;
                self.neighbor_changed(x - 1, y, z, type_);
                self.neighbor_changed(x + 1, y, z, type_);
                self.neighbor_changed(x, y - 1, z, type_);
                self.neighbor_changed(x, y + 1, z, type_);
                self.neighbor_changed(x, y, z - 1, type_);
                self.neighbor_changed(x, y, z + 1, type_);
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

    pub fn set_tile_no_update(&mut self, x: JInt, y: JInt, z: JInt, type_: JInt) -> JBoolean {
        if x >= 0 && y >= 0 && z >= 0 && x < self.width && y < self.depth && z < self.height {
            if type_ == self.blocks[((y * self.height + z) * self.width + x) as usize].into() {
                false
            } else {
                self.blocks[((y * self.height + z) * self.width + x) as usize] = type_ as JByte;
                true
            }
        } else {
            false
        }
    }

    pub fn neighbor_changed(&self, x: JInt, y: JInt, z: JInt, type_: JInt) {
        if x >= 0 && y >= 0 && z >= 0 && x < self.width && y < self.depth && z < self.height {
            let tile =
                get_tile(self.blocks[((y * self.height + z) * self.width + x) as usize].into());

            if let Some(tile) = tile {
                tile.neighbor_changed(self, x, y, z, type_);
            }
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
        let ticks = self.unprocessed / TILE_UPDATE_INTERVAL;
        self.unprocessed -= ticks * TILE_UPDATE_INTERVAL;

        let w = self.width;
        let d = self.depth;
        let h = self.height;

        for _ in 0..ticks {
            self.rand_value = self.rand_value * LEVEL_MULTIPLIER + LEVEL_ADDEND;
            let x = (self.rand_value >> 16) & (w - 1);
            self.rand_value = self.rand_value * LEVEL_MULTIPLIER + LEVEL_ADDEND;
            let y = (self.rand_value >> 16) & (d - 1);
            self.rand_value = self.rand_value * LEVEL_MULTIPLIER + LEVEL_ADDEND;
            let z = (self.rand_value >> 16) & (h - 1);
            let id = self.blocks[((y * h + z) * w + x) as usize].into();
            // if shouldTick[id] {
            get_tile(id)
                .unwrap()
                .tick(self, x, y, z, self.random.clone());
            // }
        }
    }

    #[must_use]
    pub const fn get_ground_level(&self) -> JFloat {
        32.0
    }

    #[must_use]
    pub fn contains_any_liquid(&self, aabb: &AABB) -> JBoolean {
        let mut x0 = f32::floor(aabb.x0) as JInt;
        let mut x1 = f32::floor(aabb.x1 + 1.0) as JInt;
        let mut y0 = f32::floor(aabb.y0) as JInt;
        let mut y1 = f32::floor(aabb.y1 + 1.0) as JInt;
        let mut z0 = f32::floor(aabb.z0) as JInt;
        let mut z1 = f32::floor(aabb.z1 + 1.0) as JInt;

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
                    let tile = get_tile(self.get_tile(x, y, z));

                    if let Some(tile) = tile
                        && tile.get_liquid_type() > 0
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    #[must_use]
    pub fn contains_liquid(&self, aabb: &AABB, liquid_id: JInt) -> JBoolean {
        let mut x0 = f32::floor(aabb.x0) as JInt;
        let mut x1 = f32::floor(aabb.x1 + 1.0) as JInt;
        let mut y0 = f32::floor(aabb.y0) as JInt;
        let mut y1 = f32::floor(aabb.y1 + 1.0) as JInt;
        let mut z0 = f32::floor(aabb.z0) as JInt;
        let mut z1 = f32::floor(aabb.z1 + 1.0) as JInt;

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
                    let tile = get_tile(self.get_tile(x, y, z));

                    if let Some(tile) = tile
                        && tile.get_liquid_type() == liquid_id
                    {
                        return true;
                    }
                }
            }
        }

        false
    }
}
