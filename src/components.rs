use std::collections::HashMap;

use crate::ecs::{
    component_store::ComponentStore,
    entity_manager::{Entity, EntityManager},
};

pub mod collider;
pub mod force;
pub mod mass;
pub mod position;
pub mod render;
pub mod velocity;

pub trait Component: Sized + 'static {
    const BIT: u64;

    fn add_to_entity(e: Entity, em: &mut EntityManager) {
        match em.entities.get_mut(&e) {
            Some(bitset) => *bitset |= Self::BIT,
            None => {}
        };
    }

    fn add_to_store(self, e: Entity, cs: &mut ComponentStore);
    fn store(cs: &mut ComponentStore) -> &mut HashMap<Entity, Self>;
}
