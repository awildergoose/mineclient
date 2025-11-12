use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use crate::{
    gl,
    java::{JFloat, JInt},
    level::{level::Level, tesselator::Tesselator},
    particle::particle::Particle,
    player::Player,
    textures,
    traits::Tickable,
};

pub struct ParticleEngine {
    #[allow(dead_code)]
    level: Rc<RefCell<Level>>,
    particles: Vec<Particle>,
    pub t: Rc<RefCell<Tesselator>>,
}

impl ParticleEngine {
    pub fn new(level: Rc<RefCell<Level>>, t: Rc<RefCell<Tesselator>>) -> Self {
        Self {
            level,
            t,
            particles: Vec::new(),
        }
    }

    pub fn add(&mut self, p: Particle) {
        self.particles.push(p);
    }

    pub fn tick(&mut self) {
        self.particles.iter_mut().for_each(|z| z.tick());
        self.particles.retain(|z| !z.removed);
    }

    pub fn render(&mut self, player: &Player, a: JFloat, layer: JInt) {
        unsafe {
            gl::Enable(3553);
            gl::BindTexture(3553, textures::load_2d_texture("terrain.png"));
        }

        let xa = -(f32::cos(player.y_rot * PI / 180.0));
        let za = -(f32::sin(player.y_rot * PI / 180.0));
        let xa2 = -za * f32::sin(player.x_rot * PI / 180.0);
        let za2 = xa * f32::sin(player.x_rot * PI / 180.0);
        let ya = f32::cos(player.x_rot * PI / 180.0);
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
