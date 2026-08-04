use std::{
    cell::RefCell,
    fs::File,
    process,
    rc::Rc,
    sync::{Arc, atomic::Ordering},
    thread,
    time::Duration,
};

use crate::{
    character::zombie::Zombie,
    entity::EntityTrait,
    gl::{
        self,
        types::{GLdouble, GLenum, GLint, GLubyte},
    },
    gui::{font::Font, screen::ScreenTrait},
    hit_result::HitResult,
    java::{
        JBoolean, JFloat, JInt, WINDOW_CTX, get_milli_time, get_mouse_dx, get_mouse_dy,
        get_mouse_x, get_mouse_y, grab_mouse, init_display, internal_update,
        is_display_close_requested, is_key_down, is_mouse_button_just_pressed,
    },
    level::{
        chunk, frustum, level::Level, level_gen::LevelGen, level_io::LevelIo,
        level_loader_listener::LevelLoaderListener, tile::tile::get_tile,
    },
    particle::particle_engine::ParticleEngine,
    phys::aabb::AABB,
    player::Player,
    renderer::{level_renderer::LevelRenderer, textures::Textures},
    timer::Timer,
    traits::Tickable,
    user::User,
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

pub fn check_gl_error(s: &str) {
    let e = unsafe { gl::GetError() };
    if e != 0 {
        // is this a ptr? should we convert this into a CString?
        let error_string = unsafe { gluErrorString(e) };
        println!("########## GL ERROR ##########");
        println!("@ {s:?}");
        println!("{e}: {error_string:?}");
        process::exit(0);
    }
}

pub struct Minecraft {
    pub width: JInt,
    pub height: JInt,
    fog_color_0: [f32; 4],
    fog_color_1: [f32; 4],
    timer: Timer,
    level: Option<Rc<RefCell<Level>>>,
    level_renderer: Option<Rc<RefCell<LevelRenderer>>>,
    player: Option<Player>,
    viewport_buffer: [i32; 16],
    select_buffer: [i32; 2000],
    hit_result: Option<HitResult>,
    entities: Vec<Box<dyn EntityTrait>>,
    paint_texture: JInt,
    particle_engine: Option<ParticleEngine>,
    pub user: User,
    pub font: Option<Rc<RefCell<Font>>>,
    running: JBoolean,
    pause: JBoolean,
    y_mouse_axis: JFloat,
    edit_mode: JInt,
    screen: Option<Arc<dyn ScreenTrait>>,
    level_io: LevelIo,
    level_gen: LevelGen,
    fps_string: String,
    pub textures: Rc<RefCell<Textures>>,

    // rust-specific
    receiver_begin: std::sync::mpsc::Receiver<String>,
    receiver_update: std::sync::mpsc::Receiver<String>,
}

pub const VERSION_STRING: &str = "0.0.13a";

// Rust-specific
#[derive(Clone)]
struct MinecraftInner {
    begin: std::sync::mpsc::Sender<String>,
    update: std::sync::mpsc::Sender<String>,
}

impl LevelLoaderListener for MinecraftInner {
    fn begin_level_loading(&self, status: &str) {
        self.begin.send(status.to_owned()).unwrap();
    }

    fn level_load_update(&self, status: &str) {
        self.update.send(status.to_owned()).unwrap();
    }
}

impl Minecraft {
    // rust-specific
    fn poll(&self) {
        if let Ok(msg) = self.receiver_begin.try_recv() {
            println!("level begin: {msg}");
        }

        while let Ok(msg) = self.receiver_update.try_recv() {
            println!("level update: {msg}");
        }
    }

    #[must_use]
    pub fn new() -> Self {
        let (sender_begin, receiver_begin) = std::sync::mpsc::channel();
        let (sender_update, receiver_update) = std::sync::mpsc::channel();

        let inner = MinecraftInner {
            begin: sender_begin,
            update: sender_update,
        };
        let inner2 = inner.clone();

        Self {
            width: 0,
            height: 0,
            paint_texture: 1,
            fog_color_0: [0.0; 4],
            fog_color_1: [0.0; 4],
            viewport_buffer: [0; 16],
            select_buffer: [0; 2000],
            timer: Timer::new(20.0),
            entities: Vec::new(),
            textures: Rc::new(RefCell::new(Textures::new())),
            running: false,
            pause: false,
            edit_mode: 0,
            screen: None,
            level_io: LevelIo::new(Arc::new(inner)),
            level_gen: LevelGen::new(Arc::new(inner2)),
            fps_string: String::new(),
            y_mouse_axis: 1.0,
            font: None,
            level: None,
            level_renderer: None,
            player: None,
            particle_engine: None,
            user: User::new("noname".to_owned()),
            hit_result: None,
            receiver_begin,
            receiver_update,
        }
    }

    pub fn init(&mut self) {
        let col1 = 920_330;
        let fr = 0.5;
        let fg = 0.8;
        let fb = 1.0;
        self.fog_color_0 = [fr, fg, fb, 1.0];
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

        check_gl_error("Pre startup");

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
            gl::AlphaFunc(516, 0.0);
            gl::CullFace(1029);
            gl::MatrixMode(gl::PROJECTION);
            gl::LoadIdentity();
            gl::MatrixMode(gl::MODELVIEW);
        }

        check_gl_error("Startup");
        self.font = Some(Rc::new(RefCell::new(Font::new(
            "default.gif",
            self.textures.clone(),
        ))));

        let file = File::open("level.dat").unwrap();
        let mut res = self.level_io.load(&file);

        if res.is_err() {
            res = self.level_io.load_legacy(&file);

            if res.is_err() {
                res = Ok(self
                    .level_gen
                    .generate_level(self.user.name.clone(), 256, 256, 64));
            }
        }

        let level = Rc::new(RefCell::new(res.unwrap()));

        self.level = Some(level.clone());
        self.level_renderer = Some(LevelRenderer::new(level.clone(), self.textures.clone()));
        self.player = Some(Player::new(level.clone()));
        self.particle_engine = Some(ParticleEngine::new(
            level.clone(),
            self.textures.clone(),
            self.level_renderer.as_ref().unwrap().borrow().t.clone(),
        ));

        grab_mouse();

        for _i in 0..10 {
            let mut zombie = Zombie::new(level.clone(), self.textures.clone(), 128.0, 0.0, 128.0);
            zombie.reset_pos();
            self.entities.push(Box::new(zombie));
        }

        // Applet code for setting emptyCursor is excluded

        check_gl_error("Post startup");
    }

    pub fn destroy(&mut self) {
        if let Err(err) = self.level.as_ref().unwrap().borrow().save() {
            eprintln!("failed to save level: {err:?}");
        }
    }

    pub const fn stop(&mut self) {
        self.running = false;
    }

    pub fn run(&mut self) {
        self.running = true;
        self.init();

        let mut last_time = get_milli_time();
        let mut frames = 0;

        while self.running {
            if self.pause {
                thread::sleep(Duration::from_millis(100));
            } else {
                if is_display_close_requested() || is_key_down(glfw::Key::Escape) {
                    self.stop();
                }

                // rust-specific
                self.poll();

                self.timer.advance_time();

                for _ in 0..self.timer.ticks {
                    self.tick();
                }

                check_gl_error("Pre render");
                self.render(self.timer.a);
                check_gl_error("Post render");
                frames += 1;

                while get_milli_time() >= last_time + 1000 {
                    self.fps_string =
                        format!("{} fps, {}", frames, chunk::UPDATES.load(Ordering::SeqCst));
                    chunk::UPDATES.store(0, Ordering::SeqCst);
                    last_time += 1000;
                    frames = 0;
                }
            }
        }

        self.destroy();
    }

    pub fn tick(&mut self) {
        if is_key_down(glfw::Key::Enter) {
            if let Err(err) = self.level.as_ref().unwrap().borrow().save() {
                eprintln!("failed to save level: {err:?}");
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
        } else if is_key_down(glfw::Key::Y) {
            self.y_mouse_axis *= -1.0;
        } else if is_key_down(glfw::Key::G) {
            let player = self.player.as_ref().unwrap();
            self.entities.push(Box::new(Zombie::new(
                self.level.as_ref().unwrap().clone(),
                self.textures.clone(),
                player.x,
                player.y,
                player.z,
            )));
        }

        self.particle_engine.as_mut().unwrap().tick();
        self.level.as_mut().unwrap().borrow_mut().tick();

        self.entities.iter_mut().for_each(|z| z.tick());
        self.entities.retain(|z| !z.is_removed());
        self.player.as_mut().unwrap().tick();
    }

    fn move_camera_to_player(&self, a: JFloat) {
        let player = self.player.as_ref().unwrap();

        unsafe {
            gl::Translatef(0.0, 0.0, -0.3);
            gl::Rotatef(player.x_rot, 1.0, 0.0, 0.0);
            gl::Rotatef(player.y_rot, 0.0, 1.0, 0.0);
        }

        let x = (player.x - player.xo).mul_add(a, player.xo);
        let y = (player.y - player.yo).mul_add(a, player.yo);
        let z = (player.z - player.zo).mul_add(a, player.zo);

        unsafe {
            gl::Translatef(-x, -y, -z);
        }
    }

    fn setup_camera(&mut self, a: JFloat) {
        unsafe {
            gl::MatrixMode(gl::PROJECTION);
            gl::LoadIdentity();
            gluPerspective(
                70.0,
                f64::from(self.width) / f64::from(self.height),
                0.05,
                1024.0,
            );
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
                f64::from(x),
                f64::from(y),
                5.0,
                5.0,
                self.viewport_buffer.as_mut_ptr(),
            );
            gluPerspective(
                70.0,
                f64::from(self.width) / f64::from(self.height),
                0.05,
                1024.0,
            );
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
                self.select_buffer.as_mut_ptr().cast(),
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

    pub fn handle_mouse_click(&mut self) {
        let hito = self.hit_result.as_mut();
        if let Some(ref hit) = hito {
            if self.edit_mode == 0 {
                let binding = self.level.as_mut().unwrap();
                let old_type = binding.borrow().get_tile(hit.x, hit.y, hit.z);

                let changed = {
                    let mut level = binding.borrow_mut();
                    level.set_tile(hit.x, hit.y, hit.z, 0)
                };

                if let Some(otile) = get_tile(old_type).as_mut()
                    && changed
                {
                    otile.destroy(
                        binding.clone(),
                        hit.x,
                        hit.y,
                        hit.z,
                        self.particle_engine.as_mut().unwrap(),
                    );
                }
            } else {
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
                let aabb = get_tile(paint_texture).unwrap().get_aabb(x, y, z);

                if aabb.is_none() || self.is_free(aabb.unwrap()) {
                    self.level
                        .as_mut()
                        .unwrap()
                        .borrow_mut()
                        .set_tile(x, y, z, paint_texture);
                }
            }
        }
    }

    #[must_use]
    pub fn is_free(&self, aabb: AABB) -> JBoolean {
        if self.player.as_ref().unwrap().bb.intersects(aabb.clone()) {
            false
        } else {
            for e in &self.entities {
                if e.get_bb().intersects(aabb.clone()) {
                    return false;
                }
            }

            true
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn render(&mut self, a: JFloat) {
        if is_mouse_button_just_pressed(0) {
            self.handle_mouse_click();
        }

        if is_mouse_button_just_pressed(1) {
            self.edit_mode = (self.edit_mode + 1) % 2;
        }

        let xo = get_mouse_dx();
        let yo = get_mouse_dy();
        self.player
            .as_mut()
            .unwrap()
            .turn(xo, yo * self.y_mouse_axis);
        // in the original it resets mouse pos
        // to w/2 h/2 here but we dont need that
        check_gl_error("Set viewport");
        self.pick(a);
        check_gl_error("Picked");

        unsafe {
            gl::Clear(16640);
            self.setup_camera(a);
            check_gl_error("Set up camera");
            gl::Enable(gl::CULL_FACE);
            let frustum = frustum::get_frustum();

            {
                let f = frustum.lock().unwrap();
                self.level_renderer.as_mut().unwrap().borrow_mut().cull(&f);
            }

            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .update_dirty_chunks(self.player.as_ref().unwrap());
            check_gl_error("Update chunks");
            self.setup_fog(0);
            gl::Enable(2912);
            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .render(self.player.as_ref().unwrap(), 0);
            check_gl_error("Rendered level");

            for z in &mut self.entities {
                if z.is_lit() && frustum.lock().unwrap().is_visible(z.get_bb()) {
                    z.render(a);
                }
            }

            check_gl_error("Rendered entities");
            self.particle_engine
                .as_mut()
                .unwrap()
                .render(self.player.as_ref().unwrap(), a, 0);
            check_gl_error("Rendered particles");
            self.setup_fog(1);
            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .render(self.player.as_ref().unwrap(), 1);

            for z in &mut self.entities {
                if !z.is_lit() && frustum.lock().unwrap().is_visible(z.get_bb()) {
                    z.render(a);
                }
            }

            self.particle_engine
                .as_mut()
                .unwrap()
                .render(self.player.as_ref().unwrap(), a, 1);

            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .render_surrounding_ground();
            if let Some(hit) = &self.hit_result {
                gl::Disable(2896);
                gl::Disable(3008);
                self.level_renderer
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .render_hit(
                        self.player.as_ref().unwrap(),
                        hit,
                        self.edit_mode,
                        self.paint_texture,
                    );
                self.level_renderer
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .render_hit_outline(
                        self.player.as_ref().unwrap(),
                        hit,
                        self.edit_mode,
                        self.paint_texture,
                    );
                gl::Enable(3008);
                gl::Enable(2896);
            }

            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .render_surrounding_ground();

            if self.hit_result.is_some() {
                gl::Disable(2896);
                gl::Disable(3008);
            }

            gl::BlendFunc(770, 771);
            self.setup_fog(0);
            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .render_surrounding_water();
            gl::Enable(3042);
            gl::ColorMask(0, 0, 0, 0); // GL11.glColorMask(false, false, false, false);
            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .render(self.player.as_ref().unwrap(), 2);
            gl::ColorMask(1, 1, 1, 1); // GL11.glColorMask(true, true, true, true);
            self.level_renderer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .render(self.player.as_ref().unwrap(), 2);

            gl::Disable(3042);
            gl::Disable(2896);
            gl::Disable(gl::TEXTURE_2D);
            gl::Disable(2912);

            if let Some(ref hit) = self.hit_result {
                gl::DepthFunc(513);
                gl::Disable(3008);
                self.level_renderer
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .render_hit(
                        self.player.as_ref().unwrap(),
                        hit,
                        self.edit_mode,
                        self.paint_texture,
                    );
                self.level_renderer
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .render_hit_outline(
                        self.player.as_ref().unwrap(),
                        hit,
                        self.edit_mode,
                        self.paint_texture,
                    );
                gl::Enable(3008);
                gl::DepthFunc(515);
            }
        }

        self.draw_gui(a);
        check_gl_error("Rendered gui");
        internal_update();
    }

    fn draw_gui(&mut self, _a: JFloat) {
        // TODO gl constants
        let binding = self.level_renderer.as_mut().unwrap().borrow();
        let mut t = binding.t.borrow_mut();
        let screen_width = self.width * 240 / self.height;
        let screen_height = self.height * 240 / self.height;
        let _x_mouse = get_mouse_x() as f32 * screen_width as f32 / self.width as f32;
        let _y_mouse = screen_height as f32
            - get_mouse_y() as f32 * screen_height as f32 / self.height as f32
            - 1.0;

        unsafe {
            gl::Clear(256);
            gl::MatrixMode(5889);
            gl::LoadIdentity();
            gl::Ortho(
                0.0,
                screen_width.into(),
                screen_height.into(),
                0.0,
                100.0,
                300.0,
            );
            gl::MatrixMode(5888);
            gl::LoadIdentity();
            gl::Translatef(0.0, 0.0, -200.0);
            check_gl_error("GUI: Init");
            gl::PushMatrix();
            gl::Translatef((screen_width - 16) as f32, 16.0, -50.0);
            gl::Scalef(16.0, 16.0, 16.0);
            gl::Rotatef(-30.0, 1.0, 0.0, 0.0);
            gl::Rotatef(45.0, 0.0, 1.0, 0.0);
            gl::Translatef(-1.5, 0.5, 0.5);
            gl::Scalef(-1.0, -1.0, -1.0);
            let id = self.textures.borrow_mut().load_texture("terrain.png", 9728);
            gl::BindTexture(3553, id);
            gl::Enable(3553);
            t.begin();
            get_tile(self.paint_texture).unwrap().render(
                &mut t,
                &self.level.as_ref().unwrap().borrow(),
                0,
                -2,
                0,
                0,
            );
            t.end();
            gl::Disable(3553);
            gl::PopMatrix();
            check_gl_error("GUI: Draw selected");
            let font = self.font.as_mut().unwrap().borrow_mut();
            font.draw_shadow(&mut t, VERSION_STRING.to_owned(), 2, 2, 16_777_215);
            font.draw_shadow(&mut t, self.fps_string.clone(), 2, 12, 16_777_215);
            gl::Color4f(1.0, 1.0, 1.0, 1.0);
        }

        let wc = (screen_width / 2) as f32;
        let hc = (screen_height / 2) as f32;

        t.begin();
        t.vertex(wc + 1.0, hc - 4.0, 0.0);
        t.vertex(wc - 0.0, hc - 4.0, 0.0);
        t.vertex(wc - 0.0, hc + 5.0, 0.0);
        t.vertex(wc + 1.0, hc + 5.0, 0.0);
        t.vertex(wc + 5.0, hc - 0.0, 0.0);
        t.vertex(wc - 4.0, hc - 0.0, 0.0);
        t.vertex(wc - 4.0, hc + 1.0, 0.0);
        t.vertex(wc + 5.0, hc + 1.0, 0.0);
        t.end();
        check_gl_error("GUI: Draw crosshair");
    }

    fn setup_fog(&mut self, i: JInt) {
        unsafe {
            let current_tile = get_tile(self.level.as_ref().unwrap().borrow().get_tile(
                (self.player.as_ref().unwrap().x) as i32,
                (self.player.as_ref().unwrap().y + 0.12) as i32,
                (self.player.as_ref().unwrap().z) as i32,
            ));

            if let Some(tile) = current_tile
                && tile.get_liquid_type() == 1
            {
                gl::Fogi(2917, 2048);
                gl::Fogf(2914, 0.1);
                gl::Fogfv(2918, Self::get_buffer(0.02, 0.02, 0.2, 1.0).as_mut_ptr());
                gl::LightModelfv(2899, Self::get_buffer(0.3, 0.3, 0.7, 1.0).as_mut_ptr());
            } else if let Some(tile) = current_tile
                && tile.get_liquid_type() == 2
            {
                gl::Fogi(2917, 2048);
                gl::Fogf(2914, 2.0);
                gl::Fogfv(2918, Self::get_buffer(0.6, 0.1, 0.0, 1.0).as_mut_ptr());
                gl::LightModelfv(2899, Self::get_buffer(0.4, 0.3, 0.3, 1.0).as_mut_ptr());
            } else if i == 0 {
                gl::Fogi(2917, 2048);
                gl::Fogf(2914, 0.001);
                gl::Fogfv(2918, self.fog_color_0.as_mut_ptr());
                gl::LightModelfv(2899, Self::get_buffer(1.0, 1.0, 1.0, 1.0).as_mut_ptr());
            } else if i == 1 {
                gl::Fogi(2917, 2048);
                gl::Fogf(2914, 0.01);
                gl::Fogfv(2918, self.fog_color_1.as_mut_ptr());
                let br = 0.6;
                gl::LightModelfv(2899, Self::get_buffer(br, br, br, 1.0).as_mut_ptr());
            }

            gl::Enable(2903);
            gl::ColorMaterial(1028, 4608);
            gl::Enable(2896);
        }
    }

    fn get_buffer(a: JFloat, b: JFloat, c: JFloat, d: JFloat) -> Vec<JFloat> {
        vec![a, b, c, d]
    }
}

impl Default for Minecraft {
    fn default() -> Self {
        Self::new()
    }
}
