use crate::{
    components::{
        force::Force,
        mass::Mass,
        position::{self, Position},
    },
    ecs::world::WorldData,
    systems::System,
};

pub struct Gravity {}

impl System for Gravity {
    fn run(&mut self, wd: &mut WorldData, dt: f32) {
        for e in wd.query::<(Mass, Force)>() {
            let Some(mass) = wd.components.mass.get_mut(&e) else {
                continue;
            };

            let Some(force) = wd.components.force.get_mut(&e) else {
                continue;
            };

            force.x += 0.0 * mass.m;
            force.y += 9.8 * mass.m;
        }
    }
}
