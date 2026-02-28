use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use javarandom::JavaRandom;

use crate::{
    java::{JByte, JInt},
    level::{noise_map::NoiseMap, tile::tile::Tile},
};

pub struct LevelGen {
    width: JInt,
    height: JInt,
    depth: JInt,
    random: Rc<RefCell<JavaRandom>>,
}

impl LevelGen {
    pub const fn new(
        random: Rc<RefCell<JavaRandom>>,
        width: JInt,
        height: JInt,
        depth: JInt,
    ) -> Self {
        Self {
            width,
            height,
            depth,
            random,
        }
    }

    pub fn generate_map(&mut self) -> Vec<JByte> {
        let mut random = self.random.borrow_mut();
        let w = self.width;
        let h = self.height;
        let d = self.depth;
        let heightmap1 = NoiseMap::new(0).read(w, h);
        let heightmap2 = NoiseMap::new(0).read(w, h);
        let cf = NoiseMap::new(1).read(w, h);
        let rock_map = NoiseMap::new(1).read(w, h);
        let mut blocks = vec![0i8; (self.width * self.height * self.depth) as usize];

        for x in 0..w {
            for y in 0..d {
                for z in 0..h {
                    let dh1 = heightmap1[(x + z * self.width) as usize];
                    let mut dh2 = heightmap2[(x + z * self.width) as usize];
                    let cfh = cf[(x + z * self.width) as usize];
                    if cfh < 128 {
                        dh2 = dh1;
                    }

                    let mut dh = if dh2 > dh1 { dh2 } else { dh1 };

                    dh = dh / 8 + d / 3;
                    let mut rh = rock_map[(x + z * self.width) as usize] / 8 + d / 3;
                    if rh > dh - 2 {
                        rh = dh - 2;
                    }

                    let i = (y * self.height + z) * self.width + x;
                    let id = if y == dh {
                        Tile::GRASS.id
                    } else if y < dh {
                        Tile::DIRT.id
                    } else if y <= rh {
                        Tile::ROCK.id
                    } else {
                        0
                    };

                    blocks[i as usize] = id as JByte;
                }
            }
        }

        let count = w * h * d / 256 / 64;

        for _ix in 0..count {
            let mut x = (random.next_float() * w as f32) as i32;
            let mut y = (random.next_float() * d as f32) as i32;
            let mut z = (random.next_float() * h as f32) as i32;
            let length = random.next_float().mul_add(150.0, random.next_float()) as JInt;
            let mut dir1 = random.next_float() * PI * 2.0;
            let mut dira1 = 0.0;
            let mut dir2 = random.next_float() * PI * 2.0;
            let mut dira2 = 0.0;

            for l in 0..length {
                x = (x as f32 + f32::sin(dir1) * f32::cos(dir2)) as i32;
                z = (z as f32 + f32::cos(dir1) * f32::cos(dir2)) as i32;
                y = (y as f32 + f32::sin(dir2)) as i32;
                dir1 += dira1 * 0.2;
                dira1 *= 0.9;
                dira1 += random.next_float() - random.next_float();
                dir2 += dira2 * 0.5;
                dir2 *= 0.5;
                dira2 *= 0.9;
                dira2 += random.next_float() - random.next_float();
                let size = (f32::sin(l as f32 * PI / length as f32) * 2.5 + 1.0) as i32;

                for xx in x - size..=x + size {
                    for yy in y - size..=y + size {
                        for zz in z - size..=z + size {
                            let xd = xx - x;
                            let yd = yy - y;
                            let zd = zz - z;
                            let dd = xd * xd + yd * yd * 2 + zd * zd;
                            if dd < size * size
                                && xx >= 1
                                && yy >= 1
                                && zz >= 1
                                && xx < self.width - 1
                                && yy < self.depth - 1
                                && zz < self.height - 1
                            {
                                let ii = ((yy * self.height + zz) * self.width + xx) as usize;

                                if blocks[ii] == Tile::ROCK.id as i8 {
                                    blocks[ii] = 0;
                                }
                            }
                        }
                    }
                }
            }
        }

        blocks
    }
}
