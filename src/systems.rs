use crate::ecs::component_store::ComponentStore;

pub mod movement;
pub mod renderer;

pub trait System {
    fn run(&mut self, components: &mut ComponentStore, dt: f32);
}
