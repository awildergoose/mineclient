use javarandom::JavaRandom;

use crate::java::JInt;

pub struct PerlinNoiseFilter {
    random: JavaRandom,
    levels: JInt,
    fuzz: JInt,
}

impl PerlinNoiseFilter {
    // in the original, we only take in `levels`
    // but here, we'll allow a set seed
    pub fn new(random: JavaRandom, levels: JInt) -> Self {
        Self {
            random,
            levels,
            fuzz: 16,
        }
    }

    pub fn read(&mut self, width: JInt, height: JInt) -> Vec<JInt> {
        let width = width as usize;
        let height = height as usize;
        let mut tmp = vec![0; width * height];
        let level = self.levels as usize;
        let step = width >> level;

        for y in (0..height).step_by(step) {
            for x in (0..width).step_by(step) {
                tmp[x + y * width] = (self.random.next_int_with_bound(256) - 128) * self.fuzz;
            }
        }

        let mut stepx = width >> level;
        while stepx > 1 {
            let val = 256 * (stepx << level);
            let ss = stepx / 2;

            for y in (0..height).step_by(stepx) {
                for x in (0..width).step_by(stepx) {
                    let ul = tmp[x % width + y % height * width];
                    let ur = tmp[(x + stepx) % width + y % height * width];
                    let dl = tmp[x % width + (y + stepx) % height * width];
                    let dr = tmp[(x + stepx) % width + (y + stepx) % height * width];
                    let m = ((ul + dl + ur + dr) / 4)
                        + self.random.next_int_with_bound((val * 2) as u32)
                        - val as i32;
                    tmp[x + ss + (y + ss) * width] = m;
                }
            }

            for y in (0..height).step_by(stepx) {
                for x in (0..width).step_by(stepx) {
                    let c = tmp[x + y * width];
                    let r = tmp[(x + stepx) % width + y * width];
                    let d = tmp[x + (y + stepx) % width * width];
                    let mu =
                        tmp[((x + ss) & (width - 1)) + ((y + ss - stepx) & (height - 1)) * width];
                    let ml =
                        tmp[((x + ss - stepx) & (width - 1)) + ((y + ss) & (height - 1)) * width];
                    let m = tmp[(x + ss) % width + (y + ss) % height * width];
                    let u = (c + r + m + mu) / 4
                        + self.random.next_int_with_bound((val * 2) as u32)
                        - val as i32;
                    let l = (c + d + m + ml) / 4
                        + self.random.next_int_with_bound((val * 2) as u32)
                        - val as i32;
                    tmp[x + ss + y * width] = u;
                    tmp[x + (y + ss) * width] = l;
                }
            }

            stepx /= 2;
        }

        let mut result = vec![0; width * height];

        for y in 0..height {
            for x in 0..width {
                result[x + y * width] = tmp[x % width + y % height * width] / 512 + 128;
            }
        }

        result
    }
}
