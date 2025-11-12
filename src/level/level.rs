use std::{fs::File, io::Read};

use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use std::io::Write;

use crate::{
    java::{JBoolean, JByte, JFloat, JInt},
    level::{
        level_listener::LevelListener,
        tile::tile::{Tile, get_tiles},
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
}

impl Level {
    pub fn new(w: JInt, h: JInt, d: JInt) -> Self {
        let w: usize = w as usize;
        let h: usize = h as usize;
        let d: usize = d as usize;

        let mut blocks = vec![0i8; w * h * d];
        let light_depths = vec![0i32; w * h];

        for x in 0..w {
            for y in 0..d {
                for z in 0..h {
                    let i = (y * h + z) * w + x;
                    blocks[i] = if y <= d * 2 / 3 { Tile::ROCK.id } else { 0 } as i8;
                }
            }
        }

        let mut this = Self {
            width: w as JInt,
            height: h as JInt,
            depth: d as JInt,
            blocks,
            level_listeners: Vec::new(),
            light_depths,
        };

        this.calc_light_depths(0, 0, w as JInt, h as JInt);
        if let Err(err) = this.load() {
            eprintln!("failed to load level: {:?}", err);
        }

        this
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

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        let file = File::open("level.dat")?;
        let mut decoder = GzDecoder::new(file);

        let i8_slice: &mut [i8] = self.blocks.as_mut_slice();
        let u8_slice: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(i8_slice.as_mut_ptr() as *mut u8, i8_slice.len())
        };

        decoder.read_exact(u8_slice)?;

        self.calc_light_depths(0, 0, self.width, self.height);

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
            unsafe { std::slice::from_raw_parts(i8_slice.as_ptr() as *const u8, i8_slice.len()) };

        encoder.write_all(u8_slice)?;
        encoder.finish()?;

        Ok(())
    }

    pub fn add_listener(&mut self, level_listener: Box<dyn LevelListener>) {
        self.level_listeners.push(level_listener);
    }

    pub fn remove_listener(&mut self, level_listener: &dyn LevelListener) {
        if let Some(pos) = self
            .level_listeners
            .iter()
            .position(|p| std::ptr::eq(&**p, level_listener))
        {
            self.level_listeners.remove(pos);
        }
    }

    pub fn is_tile(&self, x: JInt, y: JInt, z: JInt) -> JBoolean {
        if x >= 0 && y >= 0 && z >= 0 {
            let width = self.width;
            let height = self.height;
            let depth = self.depth;

            if x < width && y < depth && z < height {
                let idx = (y * height + z) * width + x;
                return self.blocks[idx as usize] == 1;
            }
        }
        false
    }

    pub fn is_lit(&self, x: JInt, y: JInt, z: JInt) -> JBoolean {
        if x < 0 || y < 0 || z < 0 || x >= self.width || y >= self.depth || z >= self.height {
            true
        } else {
            y >= self.light_depths[(x + z * self.width) as usize]
        }
    }

    pub fn is_solid_tile(&self, x: JInt, y: JInt, z: JInt) -> JBoolean {
        self.is_tile(x, y, z)
    }

    pub fn is_light_blocker(&self, x: JInt, y: JInt, z: JInt) -> JBoolean {
        self.is_solid_tile(x, y, z)
    }

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

        let tiles = get_tiles().lock().unwrap();

        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    if let Some(tile) = tiles.get(&self.get_tile(x, y, z)) {
                        aabbs.push(tile.get_aabb(x, y, z));
                    }
                }
            }
        }

        aabbs
    }

    pub fn get_brightness(&self, x: JInt, y: JInt, z: JInt) -> JFloat {
        let dark = 0.8;
        let light = 1.0;

        if x < 0 || y < 0 || z < 0 || x >= self.width || y >= self.depth || z >= self.height {
            light
        } else {
            let width = self.width;

            if y < self.light_depths[(x + z * width) as usize] {
                dark
            } else {
                light
            }
        }
    }

    pub fn set_tile(&mut self, x: JInt, y: JInt, z: JInt, type_: JInt) -> JBoolean {
        let width = self.width;
        let height = self.height;

        if x >= 0 && y >= 0 && z >= 0 && x < self.width && y < self.depth && z < self.height {
            if type_ == self.blocks[((y * self.height + z) * self.width + x) as usize] as JInt {
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

    pub fn get_tile(&self, x: JInt, y: JInt, z: JInt) -> JInt {
        if x >= 0 && y >= 0 && z >= 0 && x < self.width && y < self.depth && z < self.height {
            self.blocks[((y * self.height + z) * self.width + x) as usize] as JInt
        } else {
            0
        }
    }
}
