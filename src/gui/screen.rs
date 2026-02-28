use std::{cell::RefCell, rc::Rc};

use crate::{
    gl,
    java::{JChar, JInt, get_mouse_x, get_mouse_y, is_mouse_button_down},
    minecraft::Minecraft,
    renderer::tesselator::Tesselator,
};

pub struct Screen {
    minecraft: Rc<RefCell<Minecraft>>,
    width: JInt,
    height: JInt,
}

pub trait ScreenTrait {
    fn base(&self) -> Screen;

    fn render(&self, _t: &mut Tesselator, _x_mouse: JInt, _y_mouse: JInt) {}
    fn init(&mut self) {}
    fn fill(&self, t: &mut Tesselator, x0: JInt, y0: JInt, x1: JInt, y1: JInt, col: JInt) {
        let a = (col >> 24 & 0xFF) as f32 / 255.0;
        let r = (col >> 16 & 0xFF) as f32 / 255.0;
        let g = (col >> 8 & 0xFF) as f32 / 255.0;
        let b = (col & 0xFF) as f32 / 255.0;
        unsafe {
            gl::Enable(3042);
            gl::BlendFunc(770, 771);
            gl::Color4f(r, g, b, a);
        }
        t.begin();
        t.vertex(x0 as f32, y1 as f32, 0.0);
        t.vertex(x1 as f32, y1 as f32, 0.0);
        t.vertex(x1 as f32, y0 as f32, 0.0);
        t.vertex(x0 as f32, y0 as f32, 0.0);
        t.end();

        unsafe {
            gl::Disable(3042);
        }
    }

    fn fill_gradient(&self, x0: JInt, y0: JInt, x1: JInt, y1: JInt, col1: JInt, col2: JInt) {
        let a1 = (col1 >> 24 & 0xFF) as f32 / 255.0;
        let r1 = (col1 >> 16 & 0xFF) as f32 / 255.0;
        let g1 = (col1 >> 8 & 0xFF) as f32 / 255.0;
        let b1 = (col1 & 0xFF) as f32 / 255.0;
        let a2 = (col2 >> 24 & 0xFF) as f32 / 255.0;
        let r2 = (col2 >> 16 & 0xFF) as f32 / 255.0;
        let g2 = (col2 >> 8 & 0xFF) as f32 / 255.0;
        let b2 = (col2 & 0xFF) as f32 / 255.0;

        unsafe {
            gl::Enable(3042);
            gl::BlendFunc(770, 771);
            gl::Begin(7);
            gl::Color4f(r1, g1, b1, a1);
            gl::Vertex2f(x1 as f32, y0 as f32);
            gl::Vertex2f(x0 as f32, y0 as f32);
            gl::Color4f(r2, g2, b2, a2);
            gl::Vertex2f(x0 as f32, y1 as f32);
            gl::Vertex2f(x1 as f32, y1 as f32);
            gl::End();
            gl::Disable(3042);
        }
    }

    fn draw_centered_string(&self, t: &mut Tesselator, str: String, x: JInt, y: JInt, color: JInt) {
        let binding = self.base();
        let binding = binding.minecraft.borrow();
        let binding = binding.font.as_ref().unwrap();
        let font = binding.borrow();
        font.draw_shadow(t, str.clone(), x - font.width(&str) / 2, y, color);
    }

    fn draw_string(&self, t: &mut Tesselator, str: String, x: JInt, y: JInt, color: JInt) {
        let binding = self.base();
        let binding = binding.minecraft.borrow();
        let binding = binding.font.as_ref().unwrap();
        let font = binding.borrow();
        font.draw_shadow(t, str, x, y, color);
    }

    fn update_events(&mut self) {
        let xm = get_mouse_x() as i32 * self.base().width / self.base().minecraft.borrow().width;
        let ym = self.base().height
            - get_mouse_y() as i32 * self.base().height / self.base().minecraft.borrow().height;

        if is_mouse_button_down(0) {
            self.mouse_clicked(xm, ym, 0);
        }
        if is_mouse_button_down(1) {
            self.mouse_clicked(xm, ym, 1);
        }

        // TODO: keyPressed for keyboard events
    }

    fn key_pressed(&mut self, _event_character: JChar, _event_key: JInt) {}
    fn mouse_clicked(&mut self, _x: JInt, _y: JInt, _button: JInt) {}
    fn tick(&mut self) {}
}
