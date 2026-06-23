use std::collections::HashMap;

use crate::{
    components::Component,
    ecs::{component_store::ComponentStore, entity_manager::Entity},
    types::vector2d::Vector2D,
};

pub enum ColliderForm {
    Circle(f32),
}

pub struct Collider {
    pub form: ColliderForm,
}

impl Component for Collider {
    const BIT: u64 = 1 << 5;
    fn add_to_store(self, e: Entity, cs: &mut ComponentStore) {
        cs.collider.insert(e, self);
    }

    fn store(cs: &mut ComponentStore) -> &mut HashMap<Entity, Self> {
        &mut cs.collider
    }
}
