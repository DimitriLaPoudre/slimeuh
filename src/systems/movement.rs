use crate::{
    components::{
        force::Force,
        mass::Mass,
        position::{self, Position},
        velocity::Velocity,
    },
    ecs::world::WorldData,
    systems::System,
};

pub struct Movement {}

impl System for Movement {
    fn run(&mut self, wd: &mut WorldData, dt: f32) {
        for e in wd.query::<(Mass, Force, Velocity, Position)>() {
            let Some(mass) = wd.components.mass.get_mut(&e) else {
                continue;
            };

            let Some(force) = wd.components.force.get_mut(&e) else {
                continue;
            };

            let Some(velocity) = wd.components.velocity.get_mut(&e) else {
                continue;
            };

            let Some(position) = wd.components.position.get_mut(&e) else {
                continue;
            };

            velocity.x += force.x * dt / mass.m;
            velocity.y += force.y * dt / mass.m;

            position.x += velocity.x * dt;
            position.y += velocity.y * dt;

            force.x = 0.0;
            force.y = 0.0;
        }
    }
}
