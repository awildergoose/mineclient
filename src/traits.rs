use crate::java::JFloat;

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
