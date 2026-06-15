use crate::{
    ecs::{
        component_store::ComponentStore,
        entity::{Entity, EntityManager},
        system_store::SystemStore,
    },
    systems::System,
};

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

    pub fn bind<T: 'static>(&mut self, entity: Entity, component: T) {
        self.components.link(entity, component)
    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.add(system);
    }

    pub fn run_systems(&mut self, dt: f32) {
        for system in self.systems.get_mut() {
            system.run(&mut self.components, dt);
        }
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
