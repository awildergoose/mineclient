use std::{
    cell::RefCell,
    f32::consts::PI,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Mutex, MutexGuard, OnceLock},
};

use crate::{
    character::zombie_model::ZombieModel,
    entity::{Entity, EntityTrait},
    gl,
    java::{JFloat, get_nano_time, math_random},
    level::level::Level,
    renderer::textures::Textures,
    traits::TimePreciseDrawable,
};

pub struct Zombie {
    base: Entity,
    textures: Rc<RefCell<Textures>>,

    pub rot: JFloat,
    pub time_offs: JFloat,
    pub speed: JFloat,
    pub rot_a: JFloat,
}

pub fn get_model() -> MutexGuard<'static, ZombieModel> {
    static ZOMBIE_MODEL: OnceLock<Mutex<ZombieModel>> = OnceLock::new();
    ZOMBIE_MODEL
        .get_or_init(|| Mutex::new(ZombieModel::new()))
        .lock()
        .unwrap()
}

impl Zombie {
    pub fn new(
        level: Rc<RefCell<Level>>,
        textures: Rc<RefCell<Textures>>,
        x: JFloat,
        y: JFloat,
        z: JFloat,
    ) -> Self {
        let mut base = Entity::new(level);
        base.set_pos(x, y, z);

        let time_offs = math_random() * 1239813.0;
        let rot = math_random() * PI * 2.0;
        let rot_a = (math_random() + 1.0) * 0.01;
        let speed = 1.0;

        Self {
            base,
            rot,
            rot_a,
            speed,
            time_offs,
            textures,
        }
    }
}

impl Deref for Zombie {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Zombie {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EntityTrait for Zombie {
    #[allow(unused_assignments)]
    fn tick(&mut self) {
        self.base.xo = self.base.x;
        self.base.yo = self.base.y;
        self.base.zo = self.base.z;
        let mut xa = 0.0;
        let mut ya = 0.0;
        if self.base.y < -100.0 {
            self.base.remove();
        }
        self.rot += self.rot_a;
        self.rot_a *= 0.99;
        self.rot_a += (math_random() - math_random()) * math_random() * math_random() * 0.08;
        xa = self.rot.sin();
        ya = self.rot.cos();
        if self.base.on_ground && math_random() < 0.08 {
            self.base.yd = 0.5;
        }

        self.base
            .move_relative(xa, ya, if self.base.on_ground { 0.1 } else { 0.02 });
        self.yd -= 0.08;
        self.base.move_(self.base.xd, self.base.yd, self.base.zd);
        self.xd *= 0.91;
        self.yd *= 0.98;
        self.zd *= 0.91;

        if self.base.on_ground {
            self.base.xd *= 0.7;
            self.base.zd *= 0.7;
        }
    }

    fn render(&mut self, a: JFloat) {
        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture(
                gl::TEXTURE_2D,
                self.textures.borrow_mut().load_texture("char.png", 9728),
            );
            gl::PushMatrix();

            let time: f64 =
                get_nano_time() as f64 / 1.0E9 * 10.0 * self.speed as f64 + self.time_offs as f64;

            let size: f32 = 0.058333334_f32;
            let yy: f32 = (-(time * 0.6662).sin().abs() * 5.0 - 23.0) as f32;

            gl::Translatef(
                self.xo + (self.x - self.xo) * a,
                self.yo + (self.y - self.yo) * a,
                self.zo + (self.z - self.zo) * a,
            );

            gl::Scalef(1.0_f32, -1.0_f32, 1.0_f32);
            gl::Scalef(size, size, size);
            gl::Translatef(0.0_f32, yy, 0.0_f32);

            let c: f64 = 180.0 / std::f64::consts::PI;
            let angle: f32 = (self.rot as f64 * c + 180.0) as f32;
            gl::Rotatef(angle, 0.0_f32, 1.0_f32, 0.0_f32);
            get_model().render(time);
            gl::PopMatrix();
            gl::Disable(gl::TEXTURE_2D);
        }
    }

    fn is_removed(&self) -> crate::java::JBoolean {
        self.removed
    }

    fn get_x(&self) -> JFloat {
        self.x
    }

    fn get_y(&self) -> JFloat {
        self.y
    }

    fn get_z(&self) -> JFloat {
        self.z
    }

    fn get_level(&self) -> Rc<RefCell<Level>> {
        self.level.clone()
    }

    fn get_bb(&self) -> &crate::phys::aabb::AABB {
        &self.bb
    }
}
