use crate::ecs::world::WorldData;

pub mod gravity;
pub mod input;
pub mod movement;
pub mod renderer;

pub mod rendering;

pub trait System {
    fn run(&mut self, wd: &mut WorldData, dt: f32);
}
