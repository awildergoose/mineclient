use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{entity::Entity, java::is_key_down, level::level::Level, traits::Tickable};

pub struct Player {
    pub base: Entity,
}

impl Player {
    pub fn new(level: Rc<RefCell<Level>>) -> Self {
        let mut this = Self {
            base: Entity::new(level),
        };

        this.reset_pos();
        this.base.height_offset = 1.62;

        this
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
            xa -= 1.0;
        }

        if is_key_down(glfw::Key::Right) || is_key_down(glfw::Key::D) {
            xa += 1.0;
        }

        if (is_key_down(glfw::Key::Space) || is_key_down(glfw::Key::LeftSuper))
            && self.base.on_ground
        {
            self.base.yd = 0.5;
        }

        self.base
            .move_relative(xa, ya, if self.base.on_ground { 0.1 } else { 0.02 });
        self.base.yd -= 0.008;
        self.base.move_(self.base.xd, self.base.yd, self.base.zd);
        self.base.xd *= 0.91;
        self.base.yd *= 0.98;
        self.base.zd *= 0.91;
        if self.base.on_ground {
            self.base.xd *= 0.7;
            self.base.zd *= 0.7;
        }
    }
}
