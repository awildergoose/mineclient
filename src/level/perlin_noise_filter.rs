use crate::java::JInt;
use javarandom::JavaRandom;

pub struct PerlinNoiseFilter {
    levels: JInt,
    fuzz: JInt,
}

impl PerlinNoiseFilter {
    pub fn new(levels: JInt) -> Self {
        Self { levels, fuzz: 16 }
    }

    pub fn read(&mut self, width_i: JInt, height_i: JInt) -> Vec<JInt> {
        let mut random = JavaRandom::new();
        let width = width_i as usize;
        let height = height_i as usize;
        if width == 0 || height == 0 {
            return vec![];
        }

        let mut tmp = vec![0i32; width * height];
        let level = self.levels as usize;
        let step = width >> level;
        if step == 0 {
            return vec![128; width * height];
        }

        for y in (0..height).step_by(step) {
            for x in (0..width).step_by(step) {
                tmp[x + y * width] = (random.next_int_with_bound(256) - 128) * self.fuzz;
            }
        }

        let mut stepx = width >> level;
        let width_mask = width - 1;
        let height_mask = height - 1;

        while stepx > 1 {
            let val = 256 * (stepx << level);
            let ss = stepx / 2;

            for y in (0..height).step_by(stepx) {
                for x in (0..width).step_by(stepx) {
                    let ul = tmp[x % width + y % height * width];
                    let ur = tmp[(x + stepx) % width + y % height * width];
                    let dl = tmp[x % width + (y + stepx) % height * width];
                    let dr = tmp[(x + stepx) % width + (y + stepx) % height * width];
                    let m = (ul + dl + ur + dr) / 4 + random.next_int_with_bound((val * 2) as u32)
                        - val as i32;
                    tmp[x + ss + (y + ss) * width] = m;
                }
            }

            for y in (0..height).step_by(stepx) {
                for x in (0..width).step_by(stepx) {
                    let c = tmp[x + y * width];
                    let r = tmp[(x + stepx) % width + y * width];
                    let d = tmp[x + ((y + stepx) % height) * width];

                    let mu_idx = ((x + ss) & width_mask)
                        + (((y + ss).wrapping_sub(stepx)) & height_mask) * width;
                    let ml_idx = (((x + ss).wrapping_sub(stepx)) & width_mask)
                        + ((y + ss) & height_mask) * width;
                    let m = tmp[(x + ss) % width + (y + ss) % height * width];
                    let mu = tmp[mu_idx];
                    let ml = tmp[ml_idx];

                    let u = (c + r + m + mu) / 4 + random.next_int_with_bound((val * 2) as u32)
                        - val as i32;
                    let l = (c + d + m + ml) / 4 + random.next_int_with_bound((val * 2) as u32)
                        - val as i32;
                    tmp[x + ss + y * width] = u;
                    tmp[x + (y + ss) * width] = l;
                }
            }

            stepx /= 2;
        }

        let mut result = vec![0i32; width * height];
        for y in 0..height {
            for x in 0..width {
                result[x + y * width] = tmp[x % width + y % height * width] / 512 + 128;
            }
        }

        result
    }
}
