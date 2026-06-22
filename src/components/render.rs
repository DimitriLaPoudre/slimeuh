use std::collections::HashMap;

use crate::{
    components::Component,
    ecs::{component_store::ComponentStore, entity_manager::Entity},
};

pub struct Render {
    pub color: u32,
}

#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        ($r << 16) | ($g << 8) | $b
    };
}

impl Component for Render {
    const BIT: u64 = 1 << 3;

    fn add_to_store(self, e: Entity, cs: &mut ComponentStore) {
        cs.render.insert(e, self);
    }

    fn store(cs: &mut ComponentStore) -> &mut HashMap<Entity, Self> {
        &mut cs.render
    }
}
