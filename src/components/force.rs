use std::collections::HashMap;

use crate::{
    components::Component,
    ecs::{
        component_store::ComponentStore,
        entity_manager::{Entity, EntityManager},
    },
};

pub struct Force {
    pub x: f32,
    pub y: f32,
}

impl Component for Force {
    const BIT: u64 = 1 << 0;

    fn add_to_store(self, e: Entity, cs: &mut ComponentStore) {
        cs.force.insert(e, self);
    }

    fn store(cs: &mut ComponentStore) -> &mut HashMap<Entity, Self> {
        &mut cs.force
    }
}
