use std::{cell::RefCell, rc::Rc};

use crate::{
    gl,
    java::{JFloat, JInt},
    level::level::Level,
    particle::particle::Particle,
    player::Player,
    renderer::{tesselator::Tesselator, textures::Textures},
};

pub struct ParticleEngine {
    #[allow(dead_code)]
    level: Rc<RefCell<Level>>,
    textures: Rc<RefCell<Textures>>,
    particles: Vec<Particle>,
    pub t: Rc<RefCell<Tesselator>>,
}

impl ParticleEngine {
    pub const fn new(
        level: Rc<RefCell<Level>>,
        textures: Rc<RefCell<Textures>>,
        t: Rc<RefCell<Tesselator>>,
    ) -> Self {
        Self {
            level,
            t,
            textures,
            particles: Vec::new(),
        }
    }

    pub fn add(&mut self, p: Particle) {
        self.particles.push(p);
    }

    pub fn tick(&mut self) {
        self.particles
            .iter_mut()
            .for_each(super::super::traits::Tickable::tick);
        self.particles.retain(|z| !z.removed);
    }

    pub fn render(&mut self, player: &Player, a: JFloat, layer: JInt) {
        if self.particles.is_empty() {
            return;
        }

        unsafe {
            gl::Enable(3553);
            gl::BindTexture(
                3553,
                self.textures.borrow_mut().load_texture("terrain.png", 9728),
            );
        }

        let xa = -(f32::cos(player.y_rot.to_radians()));
        let za = -(f32::sin(player.y_rot.to_radians()));
        let xa2 = -za * f32::sin(player.x_rot.to_radians());
        let za2 = xa * f32::sin(player.x_rot.to_radians());
        let ya = f32::cos(player.x_rot.to_radians());
        let mut t = self.t.borrow_mut();
        unsafe { gl::Color4f(0.8, 0.8, 0.8, 1.0) }

        t.init();

        for p in &mut self.particles {
            // TODO unsure if this bitwise op is right
            if (p.is_lit()) ^ (layer == 1) {
                p.render(&mut t, a, xa, ya, za, xa2, za2);
            }
        }

        t.flush();

        unsafe { gl::Disable(3553) }
    }
}
