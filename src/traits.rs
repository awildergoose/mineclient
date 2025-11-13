use std::{cell::RefCell, rc::Rc};

use crate::{
    java::{JFloat, JInt},
    level::level::Level,
    particle::particle_engine::ParticleEngine,
};

pub trait Tickable {
    fn tick(&mut self);
}

pub trait Drawable {
    fn render(&mut self, a: JFloat);
}

pub trait TimePreciseDrawable {
    fn render(&mut self, time: f64);
}

pub trait TimelessDrawable {
    fn render(&mut self);
}

pub trait TickableTile {
    fn tick(&self, level: &mut Level, x: JInt, y: JInt, z: JInt); // omitted random because it's in level
}

pub trait TileDestructionEvent {
    fn destroy(
        &self,
        level: Rc<RefCell<Level>>,
        x: JInt,
        y: JInt,
        z: JInt,
        particle_engine: &mut ParticleEngine,
    );
}
