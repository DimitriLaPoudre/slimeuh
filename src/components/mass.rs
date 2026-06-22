use std::collections::HashMap;

use crate::{
    components::Component,
    ecs::{
        component_store::ComponentStore,
        entity_manager::{Entity, EntityManager},
    },
};

pub struct Mass {
    pub m: f32,
}

impl Component for Mass {
    const BIT: u64 = 1 << 1;

    fn add_to_store(self, e: Entity, cs: &mut ComponentStore) {
        cs.mass.insert(e, self);
    }

    fn store(cs: &mut ComponentStore) -> &mut HashMap<Entity, Self> {
        &mut cs.mass
    }
}
