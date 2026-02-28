use std::{env, fs::File, io::Write, path::Path};

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("bindings.rs");
    let mut file = File::create(dest).unwrap();
    file.write_all(
        b"
    mod generated {
        #![allow(clippy::all)]
        #![allow(clippy::pedantic)]
        #![allow(clippy::nursery)]
    ",
    )
    .unwrap();

    Registry::new(Api::Gl, (2, 1), Profile::Compatibility, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    file.write_all(
        b"
}
    pub use generated::*;
    ",
    )
    .unwrap();

    // https://github.com/MoAlyousef/glu-sys/blob/master/build.rs
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "macos" => println!("cargo:rustc-link-lib=framework=OpenGL"),
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=opengl32");
            println!("cargo:rustc-link-lib=dylib=glu32");
        }
        _ => {
            println!("cargo:rustc-link-lib=dylib=GL");
            println!("cargo:rustc-link-lib=dylib=GLU");
        }
    }
}
