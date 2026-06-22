use std::collections::HashMap;

use crate::{
    components::Component,
    ecs::{component_store::ComponentStore, entity_manager::Entity},
};

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    const BIT: u64 = 1 << 4;
    fn add_to_store(self, e: Entity, cs: &mut ComponentStore) {
        cs.velocity.insert(e, self);
    }

    fn store(cs: &mut ComponentStore) -> &mut HashMap<Entity, Self> {
        &mut cs.velocity
    }
}
