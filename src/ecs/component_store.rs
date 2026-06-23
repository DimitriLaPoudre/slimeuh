use std::collections::{HashMap, binary_heap::Iter};

use crate::{
    components::{
        Component, collider::Collider, force::Force, mass::Mass, position::Position,
        render::Render, velocity::Velocity,
    },
    ecs::entity_manager::Entity,
};

pub struct ComponentStore {
    pub force: HashMap<Entity, Force>,
    pub position: HashMap<Entity, Position>,
    pub mass: HashMap<Entity, Mass>,
    pub render: HashMap<Entity, Render>,
    pub velocity: HashMap<Entity, Velocity>,
    pub collider: HashMap<Entity, Collider>,
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            position: HashMap::new(),
            force: HashMap::new(),
            mass: HashMap::new(),
            render: HashMap::new(),
            velocity: HashMap::new(),
            collider: HashMap::new(),
        }
    }
}
