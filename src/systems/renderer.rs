use minifb::{Window, WindowOptions};

use crate::{
    components::{position::Position, render::Render},
    ecs::component_store::ComponentStore,
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
    window: Window,
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
            window,
            refresh: config.refresh,
        }
    }
}

impl System for Renderer {
    fn run(&mut self, components: &mut ComponentStore, dt: f32) {
        self.frame.fill(rgb!(0, 0, 0));

        for entity in components.query::<(&Render, &Position)>() {
            let render = components.get_component_mut::<Render>(entity).unwrap();
            let position = components.get_component_mut::<Position>(entity).unwrap();

            let x = position.x as i32;
            let y = position.y as i32;
            let width = self.frame.width as i32;
            let height = self.frame.height as i32;
            if x >= 0 && y >= 0 && x < width && y < height {
                self.frame.buffer[(y * width + x) as usize] = render.color;
            }
        }

        self.window
            .update_with_buffer(&self.frame.buffer, self.frame.width, self.frame.height)
            .expect("Failed to update buffer");
    }
}
