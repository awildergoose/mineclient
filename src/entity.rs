use std::{cell::RefCell, rc::Rc};

use crate::{
    java::{math_random, JBoolean, JFloat, JInt},
    level::level::Level,
    phys::aabb::AABB,
};

pub struct Entity {
    pub level: Rc<RefCell<Level>>,
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
    pub horizontal_collision: JBoolean,
    pub removed: JBoolean,
    pub height_offset: JFloat,
    pub bb_width: JFloat,
    pub bb_height: JFloat,
}

pub trait EntityTrait {
    fn base(&self) -> &Entity;

    fn render(&mut self, _a: JFloat) {}
    fn tick(&mut self) {}

    fn is_lit(&self) -> JBoolean {
        let x_tile = self.base().x as JInt;
        let y_tile = self.base().y as JInt;
        let z_tile = self.base().z as JInt;
        self.base()
            .level
            .borrow_mut()
            .is_lit(x_tile, y_tile, z_tile)
    }

    fn is_removed(&self) -> JBoolean {
        self.base().removed
    }

    fn get_bb(&self) -> &AABB {
        &self.base().bb
    }
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
            horizontal_collision: false,
            height_offset: 0.0,
            removed: false,
            bb_width: 0.6,
            bb_height: 1.8,
        };

        this.reset_pos();

        this
    }

    pub fn reset_pos(&mut self) {
        let x = math_random().mul_add(self.level.borrow().width as f32 - 2.0, 1.0);
        let y = self.level.borrow().depth as f32 + 10.0;
        let z = math_random().mul_add(self.level.borrow().height as f32 - 2.0, 1.0);
        self.set_pos(x, y, z);
    }

    pub const fn remove(&mut self) {
        self.removed = true;
    }

    pub const fn set_size(&mut self, w: JFloat, h: JFloat) {
        self.bb_width = w;
        self.bb_height = h;
    }

    pub fn set_pos(&mut self, x: JFloat, y: JFloat, z: JFloat) {
        self.x = x;
        self.y = y;
        self.z = z;
        let w = self.bb_width / 2.0;
        let h = self.bb_height / 2.0;
        self.bb = AABB::new(x - w, y - h, z - w, x + w, y + h, z + w);
    }

    pub fn turn(&mut self, xo: JFloat, yo: JFloat) {
        self.y_rot += xo * 0.15;
        self.x_rot -= yo * 0.15;
        self.x_rot = self.x_rot.clamp(-90.0, 90.0);
    }

    #[must_use]
    pub fn is_free(&self, xa: JFloat, ya: JFloat, za: JFloat) -> JBoolean {
        let aabb = self.bb.clone_move(xa, ya, za);
        let aabbs = self.level.borrow().get_cubes(aabb.clone());

        if aabbs.is_empty() {
            !self.level.borrow().contains_any_liquid(&aabb)
        } else {
            false
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
        self.horizontal_collision = xa_org != za || za_org != za;
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

        self.x = f32::midpoint(self.bb.x0, self.bb.x1);
        self.y = self.bb.y0 + self.height_offset;
        self.z = f32::midpoint(self.bb.z0, self.bb.z1);
    }

    #[must_use]
    pub fn is_in_water(&self) -> JBoolean {
        self.level
            .borrow()
            .contains_liquid(&self.bb.grow(0.0, -0.4, 0.0), 1)
    }

    #[must_use]
    pub fn is_in_lava(&self) -> JBoolean {
        self.level.borrow().contains_liquid(&self.bb, 2)
    }

    pub fn move_relative(&mut self, mut xa: JFloat, mut za: JFloat, speed: JFloat) {
        let mut dist = xa.mul_add(xa, za * za);
        if !(dist < 0.01) {
            dist = speed / dist.sqrt();
            xa *= dist;
            za *= dist;
            let sin = f32::sin(self.y_rot.to_radians());
            let cos = f32::cos(self.y_rot.to_radians());
            self.xd += xa.mul_add(cos, -(za * sin));
            self.zd += za.mul_add(cos, xa * sin);
        }
    }

    #[must_use]
    pub fn is_lit(&self) -> JBoolean {
        let x_tile = self.x as JInt;
        let y_tile = self.y as JInt;
        let z_tile = self.z as JInt;
        self.level.borrow_mut().is_lit(x_tile, y_tile, z_tile)
    }
}

impl EntityTrait for Entity {
    fn tick(&mut self) {
        self.xo = self.x;
        self.yo = self.y;
        self.zo = self.z;
    }

    fn base(&self) -> &Entity {
        self
    }
}
