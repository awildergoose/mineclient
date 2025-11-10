use crate::java::{JFloat, JInt, JLong, get_nano_time};

pub static NS_PER_SECOND: JLong = 1000000000;
pub static MAX_NS_PER_UPDATE: JLong = 1000000000;
pub static MAX_TICKS_PER_UPDATE: JInt = 100;

pub struct Timer {
    pub ticks_per_second: JFloat,
    pub last_time: JLong,
    pub ticks: JInt,
    pub a: JFloat,
    pub time_scale: JFloat,
    pub fps: JFloat,
    pub passed_time: JFloat,
}

impl Timer {
    pub fn new(ticks_per_second: JFloat) -> Self {
        Self {
            ticks_per_second,
            last_time: get_nano_time(),
            time_scale: 1.0,
            fps: 0.0,
            passed_time: 0.0,
            ticks: 0,
            a: 0.0,
        }
    }

    pub fn advance_time(&mut self) {
        let now = get_nano_time();
        let mut passed_ns = now - self.last_time;
        self.last_time = now;
        passed_ns = passed_ns.clamp(0, 1000000000);

        self.fps = (1000000000 / passed_ns) as JFloat;
        self.passed_time = (self.passed_time as JLong
            + passed_ns * self.time_scale as JLong * self.ticks_per_second as JLong)
            as JFloat
            / 1.0E9;
        self.ticks = self.passed_time as i32;
        if self.ticks > 100 {
            self.ticks = 100;
        }

        self.passed_time -= self.ticks as JFloat;
        self.a = self.passed_time;
    }
}
