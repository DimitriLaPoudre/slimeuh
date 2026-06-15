use crate::{ecs::component_store::ComponentStore, systems::System};

pub struct Movement {}

impl System for Movement {
    fn run(&mut self, components: &mut ComponentStore, dt: f32) {}
}
