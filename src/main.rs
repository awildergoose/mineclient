#![allow(clippy::neg_cmp_op_on_partial_ord)]

use crate::rubydung::RubyDung;

pub mod gl;
pub mod hit_result;
pub mod java;
pub mod level;
pub mod phys;
pub mod player;
pub mod rubydung;
pub mod textures;
pub mod timer;

fn main() {
    let mut rd = RubyDung::default();
    rd.run();
}
