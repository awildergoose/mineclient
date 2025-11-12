use std::{cell::RefCell, rc::Rc, sync::atomic::Ordering};

use crate::{
    character::zombie::Zombie,
    gl::{
        self,
        types::{GLdouble, GLenum, GLint, GLubyte},
    },
    hit_result::HitResult,
    java::{
        JFloat, JInt, WINDOW_CTX, get_milli_time, get_mouse_dx, get_mouse_dy, grab_mouse,
        init_display, is_display_close_requested, is_key_down, is_mouse_button_down,
        update_display,
    },
    level::{chunk, frustum, level::Level, level_renderer::LevelRenderer},
    player::Player,
    timer::Timer,
    traits::{Drawable, Tickable},
};

unsafe extern "C" {
    pub fn gluErrorString(error: GLenum) -> *const GLubyte;
    pub fn gluPerspective(fovy: GLdouble, aspect: GLdouble, zNear: GLdouble, zFar: GLdouble);
    pub fn gluPickMatrix(
        x: GLdouble,
        y: GLdouble,
        delX: GLdouble,
        delY: GLdouble,
        viewport: *mut GLint,
    );
}

pub fn check_error() {
    let e = unsafe { gl::GetError() };
    if e != 0 {
        panic!("{:?}", unsafe { gluErrorString(e) });
    }
}

pub struct RubyDung {
    width: JInt,
    height: JInt,
    fog_color_0: [f32; 4],
    fog_color_1: [f32; 4],
    timer: Timer,
    level: Option<Rc<RefCell<Level>>>,
    level_renderer: Option<Rc<RefCell<LevelRenderer>>>,
    player: Option<Player>,
    viewport_buffer: [i32; 16],
    select_buffer: [i32; 2000],
    hit_result: Option<HitResult>,
    zombies: Vec<Zombie>,
    paint_texture: JInt,
}

impl RubyDung {
    pub fn new() -> RubyDung {
        Self {
            width: 0,
            height: 0,
            fog_color_0: [0.0; 4],
            fog_color_1: [0.0; 4],
            timer: Timer::new(20.0),
            level: None,
            level_renderer: None,
            player: None,
            viewport_buffer: [0; 16],
            select_buffer: [0; 2000],
            hit_result: None,
            zombies: Vec::new(),
            paint_texture: 1,
        }
    }

    pub fn init(&mut self) {
        let col0 = 16710650;
        let col1 = 920330;
        let fr = 0.5;
        let fg = 0.8;
        let fb = 1.0;
        self.fog_color_0 = [
            ((col0 >> 16) & 0xFF) as f32 / 255.0,
            ((col0 >> 8) & 0xFF) as f32 / 255.0,
            (col0 & 0xFF) as f32 / 255.0,
            1.0,
        ];
        self.fog_color_1 = [
            ((col1 >> 16) & 0xFF) as f32 / 255.0,
            ((col1 >> 8) & 0xFF) as f32 / 255.0,
            (col1 & 0xFF) as f32 / 255.0,
            1.0,
        ];
        init_display(1024, 768);

        gl::load_with(|s| {
            WINDOW_CTX.with(|ctx_cell| {
                ctx_cell
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .window
                    .get_proc_address(s)
                    .unwrap() as *const _
            })
        });

        self.width = 1024;
        self.height = 768;

        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::ShadeModel(gl::SMOOTH);
            gl::ClearColor(fr, fg, fb, 0.0);
            gl::ClearDepth(1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            gl::Enable(3008); // TODO gl constant
            gl::AlphaFunc(516, 0.5);
            gl::MatrixMode(gl::PROJECTION);
            gl::LoadIdentity();
            gl::MatrixMode(gl::MODELVIEW);
        }

        let level = Rc::new(RefCell::new(Level::new(256, 256, 64)));

        self.level = Some(level.clone());
        self.level_renderer = Some(LevelRenderer::new(level.clone()));
        self.player = Some(Player::new(level.clone()));
        // TODO create particle engine here

        grab_mouse();

        for _i in 0..100 {
            let mut zombie = Zombie::new(level.clone(), 128.0, 0.0, 128.0);
            zombie.reset_pos();
            self.zombies.push(zombie);
        }
    }

    pub fn destroy(&mut self) {
        if let Err(err) = self.level.as_ref().unwrap().borrow().save() {
            eprintln!("failed to save level: {:?}", err);
        }
    }

    pub fn run(&mut self) {
        self.init();

        let mut last_time = get_milli_time();
        let mut frames = 0;

        while !is_key_down(glfw::Key::Escape) && !is_display_close_requested() {
            self.timer.advance_time();

            for _ in 0..self.timer.ticks {
                self.tick();
            }

            self.render(self.timer.a);
            frames += 1;

            while get_milli_time() >= last_time + 1000 {
                println!("{} fps, {}", frames, chunk::UPDATES.load(Ordering::SeqCst));
                chunk::UPDATES.store(0, Ordering::SeqCst);
                last_time += 1000;
                frames = 0;
            }
        }

        self.destroy();
    }

    pub fn tick(&mut self) {
        // TODO handle keyboard events here
        if is_key_down(glfw::Key::Enter) {
            if let Err(err) = self.level.as_ref().unwrap().borrow().save() {
                eprintln!("failed to save level: {:?}", err);
            }
        } else if is_key_down(glfw::Key::Num1) {
            self.paint_texture = 1;
        } else if is_key_down(glfw::Key::Num2) {
            self.paint_texture = 2;
        } else if is_key_down(glfw::Key::Num3) {
            self.paint_texture = 3;
        } else if is_key_down(glfw::Key::Num4) {
            self.paint_texture = 4;
        } else if is_key_down(glfw::Key::Num5) {
            self.paint_texture = 5;
        } else if is_key_down(glfw::Key::Num6) {
            self.paint_texture = 6;
        }

        // TODO tick particle engine
        self.level.as_mut().unwrap().borrow_mut().tick();

        self.zombies.iter_mut().for_each(|z| z.tick());
        self.zombies.retain(|z| !z.removed);
        self.player.as_mut().unwrap().tick();
    }

    fn move_camera_to_player(&mut self, a: JFloat) {
        let player = self.player.as_ref().unwrap();

        unsafe {
            gl::Translatef(0.0, 0.0, -0.3);
            gl::Rotatef(player.x_rot, 1.0, 0.0, 0.0);
            gl::Rotatef(player.y_rot, 0.0, 1.0, 0.0);
        }

        let x = player.xo + (player.x - player.xo) * a;
        let y = player.yo + (player.y - player.yo) * a;
        let z = player.zo + (player.z - player.zo) * a;

        unsafe {
            gl::Translatef(-x, -y, -z);
        }
    }

    fn setup_camera(&mut self, a: JFloat) {
        unsafe {
            gl::MatrixMode(gl::PROJECTION);
            gl::LoadIdentity();
            gluPerspective(70.0, self.width as f64 / self.height as f64, 0.05, 1000.0);
            gl::MatrixMode(gl::MODELVIEW);
            gl::LoadIdentity();
        }
        self.move_camera_to_player(a);
    }

    fn setup_pick_camera(&mut self, a: JFloat, x: JFloat, y: JFloat) {
        unsafe {
            gl::MatrixMode(gl::PROJECTION);
            gl::LoadIdentity();
        }
        self.viewport_buffer = [0; 16];
        unsafe {
            gl::GetIntegerv(gl::VIEWPORT, self.viewport_buffer.as_mut_ptr());
            gluPickMatrix(
                x as f64,
                y as f64,
                5.0,
                5.0,
                self.viewport_buffer.as_mut_ptr(),
            );
            gluPerspective(70.0, self.width as f64 / self.height as f64, 0.05, 1000.0);
            gl::MatrixMode(gl::MODELVIEW);
            gl::LoadIdentity();
        };
        self.move_camera_to_player(a);
    }

    fn pick(&mut self, a: JFloat) {
        self.select_buffer = [0; 2000];
        unsafe {
            gl::SelectBuffer(
                self.select_buffer.len() as i32,
                self.select_buffer.as_mut_ptr() as *mut _,
            );
            gl::RenderMode(gl::SELECT);
        }
        self.setup_pick_camera(a, (self.width / 2) as f32, (self.height / 2) as f32);
        let frustum = frustum::get_frustum().lock().unwrap();
        self.level_renderer
            .as_mut()
            .unwrap()
            .borrow_mut()
            .pick(self.player.as_ref().unwrap(), &frustum);
        let hits = unsafe { gl::RenderMode(gl::RENDER) };
        let mut closest = 0;
        let mut names = [0; 10];
        let mut hit_name_count = 0;
        let mut index = 0;

        for i in 0..hits {
            let name_count = self.select_buffer[index];
            index += 1;

            let min_z = self.select_buffer[index];
            index += 1;

            index += 1;

            if min_z >= closest && i != 0 {
                index += name_count as usize;
            } else {
                closest = min_z;
                hit_name_count = name_count;

                #[allow(clippy::needless_range_loop)]
                for j in 0..name_count as usize {
                    names[j] = self.select_buffer[index];
                    index += 1;
                }
            }
        }

        if hit_name_count > 0 {
            self.hit_result = Some(HitResult::new(
                names[0], names[1], names[2], names[3], names[4],
            ));
        } else {
            self.hit_result = None;
        }
    }

    pub fn render(&mut self, a: JFloat) {
        let xo = get_mouse_dx();
        let yo = get_mouse_dy();
        self.player.as_mut().unwrap().turn(xo, yo);
        self.pick(a);

        let hito = self.hit_result.as_mut();
        if is_mouse_button_down(1)
            && let Some(ref hit) = hito
        {
            // TODO use tile.destroy here
            self.level
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set_tile(hit.x, hit.y, hit.z, 0);
        }

        if is_mouse_button_down(0)
            && let Some(ref hit) = hito
        {
            let mut x = hit.x;
            let mut y = hit.y;
            let mut z = hit.z;
            if hit.f == 0 {
                y -= 1;
            }

            if hit.f == 1 {
                y += 1;
            }

            if hit.f == 2 {
                z -= 1;
            }

            if hit.f == 3 {
                z += 1;
            }

            if hit.f == 4 {
                x -= 1;
            }

            if hit.f == 5 {
                x += 1;
            }

            let paint_texture = self.paint_texture;
            self.level
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set_tile(x, y, z, paint_texture);
        }

        unsafe {
            gl::Clear(16640);
            self.setup_camera(a);
            gl::Enable(gl::CULL_FACE);
            let frustum = frustum::get_frustum();
            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .update_dirty_chunks(&frustum.lock().unwrap(), self.player.as_ref().unwrap());
            self.setup_fog(0);
            gl::Enable(2912);
            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .render(self.player.as_ref().unwrap(), 0);

            // TODO render with particle engine
            self.setup_fog(1);
            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .render(self.player.as_ref().unwrap(), 1);

            for z in &mut self.zombies {
                if z.is_lit() && frustum.lock().unwrap().is_visible(&z.bb) {
                    z.render(a);
                }
            }

            // TODO render with particle engine
            gl::Disable(2896);
            gl::Disable(gl::TEXTURE_2D);
            gl::Disable(2912);

            if let Some(ref hit) = self.hit_result {
                gl::Disable(3008);
                self.level_renderer
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .render_hit(hit);
                gl::Enable(3008);
            }

            // TODO draw gui
            update_display();
        }
    }

    // TODO drawGui

    fn setup_fog(&mut self, i: JInt) {
        unsafe {
            if i == 0 {
                gl::Fogi(2917, 2048);
                gl::Fogf(2914, 0.001);
                gl::Fogfv(2918, self.fog_color_0.as_mut_ptr());
                gl::Disable(2896);
            } else if i == 1 {
                gl::Fogi(2917, 2048);
                gl::Fogf(2914, 0.06);
                gl::Fogfv(2918, self.fog_color_1.as_mut_ptr());
                gl::Enable(2896);
                gl::Enable(2903);
                let br = 0.6;
                gl::LightModelfv(2899, self.get_buffer(br, br, br, 1.0).as_mut_ptr());
            }
        }
    }

    fn get_buffer(&self, a: JFloat, b: JFloat, c: JFloat, d: JFloat) -> Vec<JFloat> {
        vec![a, b, c, d]
    }
}

impl Default for RubyDung {
    fn default() -> Self {
        Self::new()
    }
}
