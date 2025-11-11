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

        if passed_ns < 1 {
            passed_ns = 1;
        }
        if passed_ns > NS_PER_SECOND {
            passed_ns = NS_PER_SECOND;
        }

        let passed_seconds = (passed_ns as f64) / (NS_PER_SECOND as f64);

        self.fps = (1.0 / passed_seconds) as JFloat;

        let delta_ticks =
            passed_seconds * (self.time_scale as f64) * (self.ticks_per_second as f64);
        self.passed_time += (delta_ticks as JFloat);

        let mut ticks = self.passed_time.floor() as JInt;
        if ticks > MAX_TICKS_PER_UPDATE {
            ticks = MAX_TICKS_PER_UPDATE;
        }
        if ticks < 0 {
            ticks = 0;
        }
        self.ticks = ticks;

        self.passed_time -= self.ticks as JFloat;

        self.a = self.passed_time;
    }
}
