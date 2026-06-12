use crate::{
    component_store::ComponentStore,
    entity::{Entity, EntityManager},
    system_store::SystemStore,
};
use anyhow::Result;

pub struct World {
    entity_manager: EntityManager,
    systems: SystemStore,
    components: ComponentStore,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            systems: SystemStore::new(),
            components: ComponentStore::new(),
        }
    }

    pub fn create(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    pub fn bind<T: 'static>(&mut self, entity: Entity, component: T) -> Result<()> {
        self.components.link(entity, component)
    }
}

#[macro_export]
macro_rules! spawn {
    ($world:expr, $( $x:expr ),+ ) => {{
        let e = $world.create();
        $(
            $world.bind(e, $x);
        )+
        e
    }};
}
