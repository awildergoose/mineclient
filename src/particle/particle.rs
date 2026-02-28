use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{
    entity::Entity,
    java::{JFloat, JInt, math_random},
    level::level::Level,
    renderer::tesselator::Tesselator,
    traits::Tickable,
};

pub struct Particle {
    base: Entity,
    xd: JFloat,
    yd: JFloat,
    zd: JFloat,
    tex: JInt,
    uo: JFloat,
    vo: JFloat,
    age: JInt,
    lifetime: JInt,
    size: JFloat,
}

impl Particle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        level: Rc<RefCell<Level>>,
        x: JFloat,
        y: JFloat,
        z: JFloat,
        xa: JFloat,
        ya: JFloat,
        za: JFloat,
        tex: JInt,
    ) -> Self {
        let mut base = Entity::new(level);
        base.set_size(0.2, 0.2);
        base.height_offset = base.bb_height / 2.0;
        base.set_pos(x, y, z);
        let mut xd = math_random().mul_add(2.0, -1.0).mul_add(0.4, xa);
        let mut yd = math_random().mul_add(2.0, -1.0).mul_add(0.4, ya);
        let mut zd = math_random().mul_add(2.0, -1.0).mul_add(0.4, za);
        let speed = (math_random() + math_random() + 1.0) * 0.15;
        let dd = f32::sqrt(xd * xd + yd * yd + zd * zd);
        xd = xd / dd * speed * 0.4;
        yd = (yd / dd * speed).mul_add(0.4, 0.1);
        zd = zd / dd * speed * 0.4;
        let uo = math_random() * 3.0;
        let vo = math_random() * 3.0;
        let size = math_random().mul_add(0.5, 0.5);
        let lifetime = (4.0 / math_random().mul_add(0.9, 0.1)) as JInt;
        let age = 0;

        Self {
            base,
            xd,
            yd,
            zd,
            tex,
            uo,
            vo,
            age,
            lifetime,
            size,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        t: &mut Tesselator,
        a: JFloat,
        xa: JFloat,
        ya: JFloat,
        za: JFloat,
        xa2: JFloat,
        za2: JFloat,
    ) {
        // TODO check if these are right
        let u0 = ((self.tex % 16) as f32 + self.uo / 4.0) / 16.0;
        let u1 = u0 + 0.015_609_375;
        let v0 = ((self.tex / 16) as f32 + self.vo / 4.0) / 16.0;
        let v1 = v0 + 0.015_609_375;
        let r = 0.1 * self.size;
        let x = (self.x - self.xo).mul_add(a, self.xo);
        let y = (self.y - self.yo).mul_add(a, self.yo);
        let z = (self.z - self.zo).mul_add(a, self.zo);
        t.vertex_uv(
            xa2.mul_add(-r, xa.mul_add(-r, x)),
            ya.mul_add(-r, y),
            za2.mul_add(-r, za.mul_add(-r, z)),
            u0,
            v1,
        );
        t.vertex_uv(
            xa2.mul_add(r, xa.mul_add(-r, x)),
            ya.mul_add(r, y),
            za2.mul_add(r, za.mul_add(-r, z)),
            u0,
            v0,
        );
        t.vertex_uv(
            xa2.mul_add(r, xa.mul_add(r, x)),
            ya.mul_add(r, y),
            za2.mul_add(r, za.mul_add(r, z)),
            u1,
            v0,
        );
        t.vertex_uv(
            xa2.mul_add(-r, xa.mul_add(r, x)),
            ya.mul_add(-r, y),
            za2.mul_add(-r, za.mul_add(r, z)),
            u1,
            v1,
        );
    }
}

impl Tickable for Particle {
    fn tick(&mut self) {
        self.base.xo = self.base.x;
        self.base.yo = self.base.y;
        self.base.zo = self.base.z;
        if self.age >= self.lifetime {
            self.base.remove();
        }
        self.age += 1;

        self.yd -= 0.04;
        self.base.move_(self.xd, self.yd, self.zd);
        self.xd *= 0.98;
        self.yd *= 0.98;
        self.zd *= 0.98;

        if self.on_ground {
            self.xd *= 0.7;
            self.zd *= 0.7;
        }
    }
}

impl Deref for Particle {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Particle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
