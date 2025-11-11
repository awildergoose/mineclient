use crate::{
    java::{JBoolean, JByte, JFloat, JInt},
    level::level_listener::LevelListener,
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
                    blocks[i] = if y <= d * 2 / 3 { 1 } else { 0 };
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
        this.load();

        this
    }

    #[allow(unused_variables)]
    pub fn calc_light_depths(&mut self, x0: JInt, y0: JInt, x1: JInt, y1: JInt) {
        // TODO stub
    }

    pub fn load(&mut self) {
        // TODO stub
    }

    pub fn save(&self) {
        // TODO stub
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
            let x = x as usize;
            let y = y as usize;
            let z = z as usize;
            let width = self.width as usize;
            let height = self.height as usize;
            let depth = self.depth as usize;

            if x < width && y < depth && z < height {
                let idx = (y * height + z) * width + x;
                return self.blocks[idx] == 1;
            }
        }
        false
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

        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    if self.is_solid_tile(x, y, z) {
                        aabbs.push(AABB::new(
                            x as f32,
                            y as f32,
                            z as f32,
                            (x + 1) as f32,
                            (y + 1) as f32,
                            (z + 1) as f32,
                        ));
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
            let x = x as usize;
            let z = z as usize;
            let width = self.width as usize;

            if y < self.light_depths[x + z * width] {
                dark
            } else {
                light
            }
        }
    }

    pub fn set_tile(&mut self, x: JInt, y: JInt, z: JInt, type_: JInt) {
        if x >= 0 && y >= 0 && z >= 0 && x < self.width && y < self.depth && z < self.height {
            let x = x as usize;
            let y = y as usize;
            let z = z as usize;
            let width = self.width as usize;
            let height = self.height as usize;

            self.blocks[(y * height + z) * width + x] = type_ as JByte;

            for ele in &self.level_listeners {
                ele.tile_changed(x as i32, y as i32, z as i32);
            }
        }
    }
}
