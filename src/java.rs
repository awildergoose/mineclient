use glfw::{Action, Context, CursorMode, GlfwReceiver, Key, MouseButton, WindowEvent};
use rand::Rng;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Instant,
};

use crate::gl;

pub type JByte = i8;
pub type JShort = i16;
pub type JInt = i32;
pub type JLong = i64;
pub type JFloat = f32;
pub type JDouble = f64;
pub type JChar = char;
pub type JBoolean = bool;

#[must_use]
pub fn get_nano_time() -> JLong {
    let ns = START_INSTANT.elapsed().as_nanos();
    if ns <= i64::MAX as u128 {
        ns as i64
    } else {
        (ns % (i64::MAX as u128 + 1)) as i64
    }
}

#[must_use]
pub fn get_milli_time() -> JLong {
    let ms = START_INSTANT.elapsed().as_millis();
    if ms <= i64::MAX as u128 {
        ms as i64
    } else {
        (ms % (i64::MAX as u128 + 1)) as i64
    }
}

#[must_use]
pub fn math_random() -> JFloat {
    let mut rng = rand::rng();
    rng.random::<f32>()
}

thread_local! {
    pub static WINDOW_CTX: RefCell<Option<WindowContext>> = const { RefCell::new(None) };
}

pub struct WindowContext {
    glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

static START_INSTANT: LazyLock<Instant> = LazyLock::new(Instant::now);
static PRESSED_KEYS: LazyLock<Mutex<HashSet<Key>>> = LazyLock::new(|| Mutex::new(HashSet::new()));
static PRESSED_MOUSE: LazyLock<Mutex<HashSet<u8>>> = LazyLock::new(|| Mutex::new(HashSet::new()));
static MOUSE_POS: LazyLock<Mutex<(f64, f64, f64, f64)>> =
    LazyLock::new(|| Mutex::new((0.0, 0.0, 0.0, 0.0)));
static MOUSE_GRABBED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
static DISPLAY_CLOSE_FLAG: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
static FRAME_COUNTER: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));
static LAST_PRESSED_KEYS: LazyLock<Mutex<HashMap<Key, u64>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static LAST_PRESSED_MOUSE: LazyLock<Mutex<HashMap<u8, u64>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[must_use]
pub fn is_key_down(key: Key) -> bool {
    let set = PRESSED_KEYS.lock().unwrap();
    set.contains(&key)
}

#[must_use]
pub fn is_mouse_button_down(button: u8) -> bool {
    let set = PRESSED_MOUSE.lock().unwrap();
    set.contains(&button)
}

#[must_use]
pub fn is_key_just_pressed(key: Key) -> bool {
    let frame = FRAME_COUNTER.load(Ordering::SeqCst);
    let map = LAST_PRESSED_KEYS.lock().unwrap();
    map.get(&key).is_some_and(|&f| f == frame)
}

#[must_use]
pub fn is_mouse_button_just_pressed(button: u8) -> bool {
    let frame = FRAME_COUNTER.load(Ordering::SeqCst);
    let map = LAST_PRESSED_MOUSE.lock().unwrap();
    map.get(&button).is_some_and(|&f| f == frame)
}

#[must_use]
pub fn get_mouse_x() -> JFloat {
    let pos = MOUSE_POS.lock().unwrap();
    pos.0 as JFloat
}

#[must_use]
pub fn get_mouse_y() -> JFloat {
    let pos = MOUSE_POS.lock().unwrap();
    pos.1 as JFloat
}

#[must_use]
pub fn get_mouse_dx() -> JFloat {
    let mut pos = MOUSE_POS.lock().unwrap();
    let dx = pos.0 - pos.2;
    pos.2 = pos.0;
    drop(pos);
    dx as JFloat
}

#[must_use]
pub fn get_mouse_dy() -> JFloat {
    let mut pos = MOUSE_POS.lock().unwrap();
    let dy = pos.1 - pos.3;
    pos.3 = pos.1;
    drop(pos);
    -dy as JFloat
}

pub fn handle_key_event(key: Key, pressed: bool) {
    let frame = FRAME_COUNTER.load(Ordering::SeqCst);
    let mut set = PRESSED_KEYS.lock().unwrap();

    if pressed {
        if !set.contains(&key) {
            let mut last = LAST_PRESSED_KEYS.lock().unwrap();
            last.insert(key, frame);
        }

        set.insert(key);
    } else {
        set.remove(&key);
    }
}

pub fn handle_mouse_button(button: u8, pressed: bool) {
    let frame = FRAME_COUNTER.load(Ordering::SeqCst);
    let mut set = PRESSED_MOUSE.lock().unwrap();

    if pressed {
        if !set.contains(&button) {
            let mut last = LAST_PRESSED_MOUSE.lock().unwrap();
            last.insert(button, frame);
        }

        set.insert(button);
    } else {
        set.remove(&button);
    }
}

pub fn handle_cursor_pos(x: f64, y: f64) {
    let mut pos = MOUSE_POS.lock().unwrap();
    pos.0 = x;
    pos.1 = y;
}

pub fn handle_window_close_request() {
    DISPLAY_CLOSE_FLAG.store(true, Ordering::SeqCst);
}

pub fn init_display(width: JInt, height: JInt) {
    WINDOW_CTX.with(|ctx_cell| {
        let mut ctx_ref = ctx_cell.borrow_mut();
        if ctx_ref.is_some() {
            return;
        }

        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW");
        glfw.window_hint(glfw::WindowHint::ContextVersion(2, 1));

        let (mut window, events) = glfw
            .create_window(
                width.cast_unsigned(),
                height.cast_unsigned(),
                "MineClient",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_close_polling(true);

        if MOUSE_GRABBED.load(Ordering::SeqCst) {
            window.set_cursor_mode(CursorMode::Disabled);
        } else {
            window.set_cursor_mode(CursorMode::Normal);
        }

        *ctx_ref = Some(WindowContext {
            glfw,
            window,
            events,
        });
    });
}

pub fn update_display() {
    FRAME_COUNTER.fetch_add(1, Ordering::SeqCst);

    WINDOW_CTX.with(|ctx_cell| {
        let mut ctx_opt = ctx_cell.borrow_mut();
        let Some(ctx) = ctx_opt.as_mut() else { return };

        ctx.window.swap_buffers();
        ctx.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&ctx.events) {
            match event {
                WindowEvent::Key(key, _scancode, action, _mods) => {
                    let pressed = action != Action::Release;
                    handle_key_event(key, pressed);
                }
                WindowEvent::MouseButton(mouse_btn, action, _mods) => {
                    let b = match mouse_btn {
                        MouseButton::Button1 => 0u8,
                        MouseButton::Button2 => 1u8,
                        MouseButton::Button3 => 2u8,
                        _ => 255u8,
                    };
                    if b != 255 {
                        handle_mouse_button(b, action != Action::Release);
                    }
                }
                WindowEvent::CursorPos(x, y) => {
                    handle_cursor_pos(x, y);
                }
                WindowEvent::FramebufferSize(w, h) => unsafe {
                    gl::Viewport(0, 0, w, h);
                },
                WindowEvent::Close => {
                    handle_window_close_request();
                    ctx.window.set_should_close(true);
                }
                _ => {}
            }
        }
    });
}

#[must_use]
pub fn is_display_close_requested() -> JBoolean {
    let mut requested = true;
    WINDOW_CTX.with(|ctx_cell| {
        let ctx_ref = ctx_cell.borrow();
        if let Some(ctx) = ctx_ref.as_ref() {
            requested = ctx.window.should_close();
        } else {
            requested = true;
        }
    });
    requested
}

pub fn grab_mouse() {
    MOUSE_GRABBED.store(true, Ordering::SeqCst);
    WINDOW_CTX.with(|ctx_cell| {
        let mut ctx_ref = ctx_cell.borrow_mut();
        if let Some(ctx) = ctx_ref.as_mut() {
            ctx.window.set_cursor_mode(CursorMode::Disabled);
        }
    });
}

pub fn release_mouse() {
    MOUSE_GRABBED.store(false, Ordering::SeqCst);
    WINDOW_CTX.with(|ctx_cell| {
        let mut ctx_ref = ctx_cell.borrow_mut();
        if let Some(ctx) = ctx_ref.as_mut() {
            ctx.window.set_cursor_mode(CursorMode::Normal);
        }
    });
}
