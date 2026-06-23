use std::{cell::RefCell, rc::Rc};

use minifb::{Window, WindowOptions};

use crate::{
    components::{
        Component,
        position::{self, Position},
        render::{Render, RenderForm},
    },
    ecs::world::WorldData,
    rgb,
    systems::System,
};

pub struct RendererConfig {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub refresh: usize,
}

struct Frame {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0u32; width * height],
        }
    }

    pub fn fill(&mut self, color: u32) {
        for pixel in &mut self.buffer {
            *pixel = color;
        }
    }
}

pub struct Renderer {
    frame: Frame,
    window: Rc<RefCell<Window>>,
    refresh: usize,
}

impl Renderer {
    pub fn new(config: RendererConfig) -> Self {
        let mut window = Window::new(
            &config.title,
            config.width,
            config.height,
            WindowOptions::default(),
        )
        .expect("Frame::new(): Window::new failed");

        window.set_target_fps(config.refresh);

        Self {
            frame: Frame::new(config.width, config.height),
            window: Rc::new(RefCell::new(window)),
            refresh: config.refresh,
        }
    }

    pub fn get_window(&self) -> Rc<RefCell<Window>> {
        self.window.clone()
    }

    fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        let width = self.frame.width as i32;
        let height = self.frame.height as i32;
        if x >= 0 && y >= 0 && x < width && y < height {
            self.frame.buffer[(y * width + x) as usize] = color;
        }
    }
}

impl System for Renderer {
    fn run(&mut self, wd: &mut WorldData, dt: f32) {
        self.frame.fill(rgb!(0, 0, 0));

        for e in wd.query::<(Render, Position)>() {
            let Some(render) = wd.components.render.get(&e) else {
                continue;
            };
            let Some(position) = wd.components.position.get(&e) else {
                continue;
            };

            match render.form {
                RenderForm::Circle(radius) => {
                    let radius = radius as i32;
                    for y in -radius..=radius {
                        let dx = ((radius * radius - y * y) as f32).sqrt() as i32;

                        for x in -dx..=dx {
                            self.put_pixel(
                                position.x as i32 + x,
                                position.y as i32 + y,
                                render.color,
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        self.window
            .borrow_mut()
            .update_with_buffer(&self.frame.buffer, self.frame.width, self.frame.height)
            .expect("Failed to update buffer");
    }
}
