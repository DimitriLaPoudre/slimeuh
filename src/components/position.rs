use std::collections::HashMap;

use crate::{
    components::Component,
    ecs::{component_store::ComponentStore, entity_manager::Entity},
};

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    const BIT: u64 = 1 << 2;

    fn add_to_store(self, e: Entity, cs: &mut ComponentStore) {
        cs.position.insert(e, self);
    }

    fn store(cs: &mut ComponentStore) -> &mut HashMap<Entity, Self> {
        &mut cs.position
    }
}
