use crate::{
    character::cube::Cube,
    traits::{TimePreciseDrawable, TimelessDrawable},
};

pub struct ZombieModel {
    pub head: Cube,
    pub body: Cube,
    pub arm0: Cube,
    pub arm1: Cube,
    pub leg0: Cube,
    pub leg1: Cube,
}

impl ZombieModel {
    pub fn new() -> Self {
        let mut head = Cube::new(0, 0);
        head.add_box(-4.0, -8.0, -4.0, 8, 8, 8);
        let mut body = Cube::new(16, 16);
        body.add_box(-4.0, 0.0, -2.0, 8, 12, 4);
        let mut arm0 = Cube::new(40, 16);
        arm0.add_box(-3.0, -2.0, -2.0, 4, 12, 4);
        arm0.set_pos(-5.0, 2.0, 0.0);
        let mut arm1 = Cube::new(40, 16);
        arm1.add_box(-1.0, -2.0, -2.0, 4, 12, 4);
        arm1.set_pos(5.0, 2.0, 0.0);
        let mut leg0 = Cube::new(0, 16);
        leg0.add_box(-2.0, 0.0, -2.0, 4, 12, 4);
        leg0.set_pos(-2.0, 12.0, 0.0);
        let mut leg1 = Cube::new(0, 16);
        leg1.add_box(-2.0, 0.0, -2.0, 4, 12, 4);
        leg1.set_pos(2.0, 12.0, 0.0);

        Self {
            head,
            body,
            arm0,
            arm1,
            leg0,
            leg1,
        }
    }
}

impl TimePreciseDrawable for ZombieModel {
    fn render(&mut self, time: f64) {
        self.head.y_rot = ((time * 0.83).sin() * 1.0) as f32;
        self.head.x_rot = (time.sin() * 0.8) as f32;
        self.arm0.x_rot = ((time * 0.6662 + std::f64::consts::PI).sin() * 2.0) as f32;
        self.arm0.z_rot = ((time * 0.2312).sin() + 1.0) as f32;
        self.arm1.x_rot = ((time * 0.6662).sin() * 2.0) as f32;
        self.arm1.z_rot = ((time * 0.2812).sin() - 1.0) as f32;
        self.leg0.x_rot = ((time * 0.6662).sin() * 1.4) as f32;
        self.leg1.x_rot = ((time * 0.6662 + std::f64::consts::PI).sin() * 1.4) as f32;

        self.head.render();
        self.body.render();
        self.arm0.render();
        self.arm1.render();
        self.leg0.render();
        self.leg1.render();
    }
}

impl Default for ZombieModel {
    fn default() -> Self {
        Self::new()
    }
}
