use std::{
    cell::RefCell,
    f32::consts::PI,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{
    character::cube::Cube,
    entity::Entity,
    gl,
    java::{JFloat, get_nano_time, math_random},
    level::level::Level,
    textures,
    traits::{Drawable, Tickable},
};

pub struct Zombie {
    base: Entity,

    pub head: Cube,
    pub body: Cube,
    pub arm0: Cube,
    pub arm1: Cube,
    pub leg0: Cube,
    pub leg1: Cube,
    pub rot: JFloat,
    pub time_offs: JFloat,
    pub speed: JFloat,
    pub rot_a: JFloat,
}

impl Zombie {
    pub fn new(level: Rc<RefCell<Level>>, x: JFloat, y: JFloat, z: JFloat) -> Self {
        let mut base = Entity::new(level);
        base.x = x;
        base.y = y;
        base.z = z;

        let time_offs = math_random() * 1239813.0;
        let rot = math_random() * PI * 2.0;
        let rot_a = (math_random() + 1.0) * 0.01;
        let speed = 1.0;
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
            base,
            arm0,
            arm1,
            body,
            head,
            leg0,
            leg1,
            rot,
            rot_a,
            speed,
            time_offs,
        }
    }
}

impl Tickable for Zombie {
    fn tick(&mut self) {
        self.base.xo = self.base.x;
        self.base.yo = self.base.y;
        self.base.zo = self.base.z;
        self.rot += self.rot_a;
        self.rot_a *= 0.99;
        self.rot_a += (math_random() - math_random()) * math_random() * math_random() * 0.01;
        let xa = self.rot.sin();
        let ya = self.rot.cos();
        if self.base.on_ground && math_random() < 0.01 {
            self.base.yd = 0.12;
        }

        self.base
            .move_relative(xa, ya, if self.base.on_ground { 0.02 } else { 0.005 });
        self.yd -= 0.005;
        self.base.move_(self.base.xd, self.base.yd, self.base.zd);
        self.xd *= 0.91;
        self.yd *= 0.98;
        self.zd *= 0.91;

        if self.base.y > 100.0 {
            self.base.reset_pos();
        }

        if self.base.on_ground {
            self.base.xd *= 0.8;
            self.base.zd *= 0.8;
        }
    }
}

impl Drawable for Zombie {
    fn render(&mut self, a: JFloat) {
        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, textures::load_2d_texture("char.png"));
            gl::PushMatrix();

            let time: f64 =
                get_nano_time() as f64 / 1.0E9 * 10.0 * self.speed as f64 + self.time_offs as f64;

            let size: f32 = 0.058333334_f32;
            let yy: f32 = (-(time * 0.6662).sin().abs() * 5.0 - 23.0) as f32;

            gl::Translatef(
                self.xo + (self.x - self.xo) * a,
                self.yo + (self.y - self.yo) * a,
                self.zo + (self.z - self.zo) * a,
            );

            gl::Scalef(1.0_f32, -1.0_f32, 1.0_f32);
            gl::Scalef(size, size, size);
            gl::Translatef(0.0_f32, yy, 0.0_f32);

            let c: f64 = 180.0 / std::f64::consts::PI;
            let angle: f32 = (self.rot as f64 * c + 180.0) as f32;
            gl::Rotatef(angle, 0.0_f32, 1.0_f32, 0.0_f32);

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

            gl::PopMatrix();
            gl::Disable(gl::TEXTURE_2D);
        }
    }
}

impl Deref for Zombie {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Zombie {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
