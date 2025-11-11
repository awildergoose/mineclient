use glfw::Key;
use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;
use std::time::Instant;

pub type JByte = i8;
pub type JShort = i16;
pub type JInt = i32;
pub type JLong = i64;
pub type JFloat = f32;
pub type JDouble = f64;
pub type JChar = char;
pub type JBoolean = bool;

pub fn get_nano_time() -> JLong {
    Instant::now().elapsed().as_nanos() as JLong
}

pub fn get_milli_time() -> JLong {
    Instant::now().elapsed().as_millis() as JLong
}

pub fn math_random() -> JFloat {
    let mut rng = rand::rng();
    rng.random::<f32>()
}

lazy_static! {
    static ref PRESSED_KEYS: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
}

pub fn is_key_down(key: Key) -> bool {
    let set = PRESSED_KEYS.lock().unwrap();
    set.contains(&key)
}

// LWJGL

pub fn set_display_mode(width: JInt, height: JInt) {}
pub fn grab_mouse() {}
pub fn is_display_close_requested() -> JBoolean {
    false
}

pub fn get_mouse_dx() -> JFloat {
    0.0
}

pub fn get_mouse_dy() -> JFloat {
    0.0
}
