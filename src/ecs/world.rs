use std::{time::Instant, vec};

use crate::{
    components::Component,
    ecs::{
        component_store::ComponentStore,
        entity_manager::{Entity, EntityManager},
        system_store::SystemStore,
    },
    systems::System,
};

pub struct WorldData {
    pub running: bool,
    pub entity_manager: EntityManager,
    pub components: ComponentStore,
}

impl WorldData {
    pub fn create(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    pub fn bind<T: Component>(&mut self, e: Entity, c: T) {
        T::add_to_entity(e, &mut self.entity_manager);
        c.add_to_store(e, &mut self.components)
    }

    pub fn query<Q: Query>(&mut self) -> Vec<Entity> {
        Q::iter(self)
    }
}

pub trait Query {
    fn iter(wd: &mut WorldData) -> Vec<Entity>;
}

// impl<A: Component> Query for (A) {
//     fn iter(wd: &mut WorldData) -> Vec<Entity> {
//         let mut filtered_entities: Vec<Entity> = vec![];
//         for (key, bitset) in wd.entity_manager.entities.iter() {
//             if bitset & (A::BIT) != 0 {
//                 filtered_entities.push(*key);
//             }
//         }
//
//         filtered_entities
//     }
// }
//
// impl<A: Component, B: Component> Query for (A, B) {
//     fn iter(wd: &mut WorldData) -> Vec<Entity> {
//         let mut filtered_entities: Vec<Entity> = vec![];
//         for (key, bitset) in wd.entity_manager.entities.iter() {
//             if bitset & (A::BIT | B::BIT) != 0 {
//                 filtered_entities.push(*key);
//             }
//         }
//
//         filtered_entities
//     }
// }

macro_rules! impl_query {
    ($($component:ident),+) => {
        impl<$($component: Component),+> Query for ($($component,)+) {
            fn iter(wd: &mut WorldData) -> Vec<Entity> {
                let mut filtered_entities = Vec::new();

                let mask = 0 $(| $component::BIT)+;

                for (key, bitset) in wd.entity_manager.entities.iter() {
                    if bitset & mask != 0 {
                        filtered_entities.push(*key);
                    }
                }

                filtered_entities
            }
        }
    };
}

impl_query!(A);
impl_query!(A, B);
impl_query!(A, B, C);
impl_query!(A, B, C, D);
impl_query!(A, B, C, D, E);

pub struct World {
    pub data: WorldData,
    systems: SystemStore,
}

impl World {
    pub fn new() -> Self {
        Self {
            data: WorldData {
                running: false,
                entity_manager: EntityManager::new(),
                components: ComponentStore::new(),
            },
            systems: SystemStore::new(),
        }
    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.add(system);
    }

    fn run_systems(&mut self, dt: f32) {
        for system in self.systems.get_mut() {
            system.run(&mut self.data, dt);
        }
    }

    pub fn run(&mut self) {
        self.data.running = true;
        let mut last = Instant::now();
        while self.data.running {
            let now = Instant::now();
            let dt = now.duration_since(last).as_secs_f32();
            last = now;
            // need to implement a per frame value to not boil this cpu

            self.run_systems(dt);
        }
    }
}

#[macro_export]
macro_rules! spawn {
    ($world:expr, $( $x:expr ),+ ) => {{
        let e = $world.data.create();
        $(
            $world.data.bind(e, $x);
        )+
        e
    }};
}
