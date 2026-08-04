use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{
    entity::{Entity, EntityTrait},
    java::{JInt, is_key_down},
    level::level::Level,
    traits::Tickable,
};

pub struct Player {
    pub base: Entity,
    keys: [bool; 10],
}

impl Player {
    pub const KEY_UP: usize = 0;
    pub const KEY_DOWN: usize = 1;
    pub const KEY_LEFT: usize = 2;
    pub const KEY_RIGHT: usize = 3;
    pub const KEY_JUMP: usize = 4;

    pub fn new(level: Rc<RefCell<Level>>) -> Self {
        let mut this = Self {
            base: Entity::new(level),
            keys: [false; 10],
        };

        this.reset_pos();
        this.base.height_offset = 1.62;

        this
    }

    pub const fn set_key(&mut self, key: JInt, state: bool) {
        let mut id = if key == 200 || key == 17 { 0 } else { -1 };

        if key == 208 || key == 31 {
            id = 1;
        }

        if key == 203 || key == 30 {
            id = 2;
        }

        if key == 205 || key == 32 {
            id = 3;
        }

        if key == 57 || key == 219 {
            id = 4;
        }

        if id >= 0 {
            self.keys[id as usize] = state;
        }
    }

    pub fn release_all_keys(&mut self) {
        for key in &mut self.keys {
            *key = false;
        }
    }
}

impl Deref for Player {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Tickable for Player {
    fn tick(&mut self) {
        self.base.tick();

        let mut xa = 0.0;
        let mut ya = 0.0;
        let in_water = self.is_in_water();
        let in_lava = self.is_in_lava();

        if self.keys[Self::KEY_UP] {
            ya -= 1.0;
        }

        if self.keys[Self::KEY_DOWN] {
            ya += 1.0;
        }

        if self.keys[Self::KEY_LEFT] {
            xa -= 1.0;
        }

        if self.keys[Self::KEY_RIGHT] {
            xa += 1.0;
        }

        if self.keys[Self::KEY_JUMP] {
            if in_water || in_lava {
                self.yd += 0.04;
            } else if self.on_ground {
                self.yd = 0.42;
                self.keys[Self::KEY_JUMP] = false;
            }
        }

        let yo;
        if in_water {
            yo = self.y;
            self.move_relative(xa, ya, 0.02);
            let (xd, yd, zd) = (self.xd, self.yd, self.zd);
            self.move_(xd, yd, zd);
            self.xd *= 0.8;
            self.yd *= 0.8;
            self.zd *= 0.8;
            self.yd -= 0.02;
            if self.horizontal_collision
                && self.is_free(self.xd, self.yd + 0.6 - self.y + yo, self.zd)
            {
                self.yd = 0.3;
            }
        } else if in_lava {
            yo = self.y;
            self.move_relative(xa, ya, 0.02);
            let (xd, yd, zd) = (self.xd, self.yd, self.zd);
            self.move_(xd, yd, zd);
            self.xd *= 0.5;
            self.yd *= 0.5;
            self.zd *= 0.5;
            self.yd -= 0.02;
            if self.horizontal_collision
                && self.is_free(self.xd, self.yd + 0.6 - self.y + yo, self.zd)
            {
                self.yd = 0.3;
            }
        } else {
            let s = if self.on_ground { 0.1 } else { 0.02 };
            self.move_relative(xa, ya, s);
            let (xd, yd, zd) = (self.xd, self.yd, self.zd);
            self.move_(xd, yd, zd);
            self.xd *= 0.91;
            self.yd *= 0.98;
            self.zd *= 0.91;
            self.yd -= 0.08;

            if self.on_ground {
                self.xd *= 0.6;
                self.zd *= 0.6;
            }
        }
    }
}
