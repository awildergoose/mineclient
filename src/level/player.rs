use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use crate::{
    java::{JBoolean, JFloat, is_key_down, math_random},
    level::level::Level,
    phys::aabb::AABB,
};

pub struct Player {
    level: Rc<RefCell<Level>>,
    pub xo: JFloat,
    pub yo: JFloat,
    pub zo: JFloat,
    pub x: JFloat,
    pub y: JFloat,
    pub z: JFloat,
    pub xd: JFloat,
    pub yd: JFloat,
    pub zd: JFloat,
    pub y_rot: JFloat,
    pub x_rot: JFloat,
    pub bb: AABB,
    pub on_ground: JBoolean,
}

impl Player {
    pub fn new(level: Rc<RefCell<Level>>) -> Self {
        let mut this = Self {
            level,
            xo: 0.0,
            yo: 0.0,
            zo: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            xd: 0.0,
            yd: 0.0,
            zd: 0.0,
            y_rot: 0.0,
            x_rot: 0.0,
            bb: AABB::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            on_ground: false,
        };

        this.reset_pos();

        this
    }

    fn reset_pos(&mut self) {
        let x = math_random() * self.level.borrow().width as f32;
        let y = self.level.borrow().depth as f32 + 10.0;
        let z = math_random() * self.level.borrow().height as f32;
        self.set_pos(x, y, z);
    }

    fn set_pos(&mut self, x: JFloat, y: JFloat, z: JFloat) {
        self.x = x;
        self.y = y;
        self.z = z;
        let w = 0.3;
        let h = 0.9;
        self.bb = AABB::new(x - w, y - h, z - w, x + w, y + h, z + w);
    }

    pub fn turn(&mut self, xo: JFloat, yo: JFloat) {
        self.y_rot += xo * 0.15;
        self.x_rot -= yo * 0.15;
        self.x_rot = self.x_rot.clamp(-90.0, 90.0);
    }

    pub fn tick(&mut self) {
        self.xo = self.x;
        self.yo = self.y;
        self.zo = self.z;

        let mut xa = 0.0;
        let mut ya = 0.0;

        if is_key_down(glfw::Key::R) {
            self.reset_pos();
        }

        if is_key_down(glfw::Key::Up) || is_key_down(glfw::Key::W) {
            ya -= 1.0;
        }

        if is_key_down(glfw::Key::Down) || is_key_down(glfw::Key::S) {
            ya += 1.0;
        }

        if is_key_down(glfw::Key::Left) || is_key_down(glfw::Key::A) {
            xa += 1.0;
        }

        if is_key_down(glfw::Key::Right) || is_key_down(glfw::Key::D) {
            xa -= 1.0;
        }

        if (is_key_down(glfw::Key::Space) || is_key_down(glfw::Key::LeftSuper)) && self.on_ground {
            self.yd = 0.12;
        }

        self.move_relative(xa, ya, if self.on_ground { 0.02 } else { 0.005 });
        self.yd -= 0.005;
        self.move_(self.xd, self.yd, self.zd);
        self.xd *= 0.91;
        self.yd *= 0.98;
        self.zd *= 0.91;
        if self.on_ground {
            self.xd *= 0.8;
            self.zd *= 0.8;
        }
    }

    pub fn move_(&mut self, mut xa: JFloat, mut ya: JFloat, mut za: JFloat) {
        let xa_org = xa;
        let ya_org = ya;
        let za_org = za;
        let aabbs = self.level.borrow().get_cubes(self.bb.expand(xa, ya, za));

        for el in &aabbs {
            ya = el.clip_y_collide(&self.bb, ya);
        }

        self.bb.move_(0.0, ya, 0.0);

        for el in &aabbs {
            xa = el.clip_x_collide(&self.bb, xa);
        }

        self.bb.move_(xa, 0.0, 0.0);

        for el in &aabbs {
            za = el.clip_z_collide(&self.bb, za);
        }

        self.bb.move_(0.0, 0.0, za);
        self.on_ground = ya_org != ya && ya_org < 0.0;
        if xa_org != xa {
            self.xd = 0.0;
        }

        if ya_org != ya {
            self.yd = 0.0;
        }

        if za_org != za {
            self.zd = 0.0;
        }

        self.x = (self.bb.x0 + self.bb.x1) / 2.0;
        self.y = self.bb.y0 + 1.62;
        self.z = (self.bb.z0 + self.bb.z1) / 2.0;
    }

    pub fn move_relative(&mut self, mut xa: JFloat, mut za: JFloat, speed: JFloat) {
        let mut dist = xa * xa + za * za;
        if !(dist < 0.01) {
            dist = speed / dist.sqrt();
            xa *= dist;
            za *= dist;
            let sin = f32::sin(self.y_rot * PI / 180.0);
            let cos = f32::sin(self.y_rot * PI / 180.0);
            self.xd += xa * cos - za * sin;
            self.zd += za * cos + xa * sin;
        }
    }
}
