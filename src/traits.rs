use crate::{
    java::{JFloat, JInt},
    level::level::Level,
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
    fn tick(&mut self, level: &mut Level, x: JInt, y: JInt, z: JInt); // omitted random because it's in level
}

pub trait TileDestructionEvent {
    // TODO add particle engine arg
    fn destroy(&mut self, level: &mut Level, x: JInt, y: JInt, z: JInt);
}
