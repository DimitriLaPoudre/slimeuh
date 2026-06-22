use crate::{ecs::world::WorldData, systems::System};

pub struct Movement {}

impl System for Movement {
    fn run(&mut self, wd: &mut WorldData, dt: f32) {}
}
