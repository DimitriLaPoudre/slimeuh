use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{ecs::world::WorldData, systems::System, types::vector2d::Vector2D};

use minifb::{MouseButton, MouseMode, Window};

#[derive(Default, Copy, Clone, Debug)]
pub struct Mouse {
    pub pos: Vector2D<f32>,
    pub abs_pos: Vector2D<f32>,
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}

impl Mouse {
    fn refresh(&mut self, window: Ref<'_, Window>) {
        if let Some(pos) = window.get_mouse_pos(MouseMode::Clamp) {
            self.pos.x = pos.0;
            self.pos.y = pos.1;
        }
        if let Some(pos) = window.get_mouse_pos(MouseMode::Pass) {
            self.abs_pos.x = pos.0;
            self.abs_pos.y = pos.1;
        }
        self.left = window.get_mouse_down(MouseButton::Left);
        self.middle = window.get_mouse_down(MouseButton::Middle);
        self.right = window.get_mouse_down(MouseButton::Right);
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    pub mouse: Mouse,
    window: Rc<RefCell<Window>>,
}

impl Input {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        Self {
            mouse: Mouse::default(),
            window: window.clone(),
        }
    }
}

impl System for Input {
    fn run(&mut self, wd: &mut WorldData, dt: f32) {
        self.mouse.refresh(self.window.borrow());
        if !self.window.borrow().is_open() {
            wd.running = false
        }
    }
}
