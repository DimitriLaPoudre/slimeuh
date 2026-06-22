use crate::{
    components::position::{self, Position},
    ecs::world::WorldData,
    systems::System,
};

pub struct Movement {}

impl System for Movement {
    fn run(&mut self, wd: &mut WorldData, dt: f32) {
        for e in wd.query::<(Position)>() {
            let Some(position) = wd.components.position.get_mut(&e) else {
                continue;
            };

            position.y += 10.0 * dt
        }
    }
}
