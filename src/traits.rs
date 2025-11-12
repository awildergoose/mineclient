use crate::java::JFloat;

pub trait Tickable {
    fn tick(&mut self);
}

pub trait Drawable {
    fn render(&mut self, a: JFloat);
}
