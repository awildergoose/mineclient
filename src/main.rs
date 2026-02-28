#![allow(clippy::neg_cmp_op_on_partial_ord)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_pass_by_value)] // should we allow this?
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::float_cmp)]
#![allow(clippy::similar_names)]
#![allow(clippy::struct_field_names)]

use crate::minecraft::Minecraft;

pub mod character;
pub mod comm;
pub mod entity;
pub mod gl;
pub mod gui;
pub mod hit_result;
pub mod java;
pub mod level;
pub mod minecraft;
pub mod particle;
pub mod phys;
pub mod player;
pub mod renderer;
pub mod server;
pub mod timer;
pub mod traits;

fn main() {
    let mut mc = Minecraft::default();
    mc.run();
}
