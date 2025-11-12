use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use crate::{
    java::{JBoolean, JFloat, math_random},
    level::level::Level,
    phys::aabb::AABB,
};

pub struct Entity {
    level: Rc<RefCell<Level>>,
    height_offset: JFloat,
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

impl Entity {
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
            height_offset: 0.0,
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
        self.y = self.bb.y0 + self.height_offset;
        self.z = (self.bb.z0 + self.bb.z1) / 2.0;
    }

    pub fn move_relative(&mut self, mut xa: JFloat, mut za: JFloat, speed: JFloat) {
        let mut dist = xa * xa + za * za;
        if !(dist < 0.01) {
            dist = speed / dist.sqrt();
            xa *= dist;
            za *= dist;
            let sin = f32::sin(self.y_rot * PI / 180.0);
            let cos = f32::cos(self.y_rot * PI / 180.0);
            self.xd += xa * cos - za * sin;
            self.zd += za * cos + xa * sin;
        }
    }
}
