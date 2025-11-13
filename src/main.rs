#![allow(clippy::neg_cmp_op_on_partial_ord)]

use crate::minecraft::Minecraft;

pub mod character;
pub mod entity;
pub mod gl;
pub mod hit_result;
pub mod java;
pub mod level;
pub mod minecraft;
pub mod particle;
pub mod phys;
pub mod player;
pub mod renderer;
pub mod timer;
pub mod traits;

fn main() {
    let mut mc = Minecraft::default();
    mc.run();
}
