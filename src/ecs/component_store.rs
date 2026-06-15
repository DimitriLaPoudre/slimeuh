use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::ecs::{entity::Entity, query::Query};

pub struct ComponentStore {
    store: HashMap<TypeId, Box<dyn Any>>,
}

pub struct ComponentItem<T> {
    pub links: HashMap<Entity, T>,
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn query<Q: Query>(&self) -> Vec<Entity> {
        Q::execute(&self)
    }

    pub fn get_or_insert_storage_mut<T: 'static>(&mut self) -> &mut ComponentItem<T> {
        let type_id = TypeId::of::<T>();

        self.store.entry(type_id).or_insert_with(|| {
            Box::new(ComponentItem::<T> {
                links: HashMap::new(),
            }) as Box<dyn Any>
        });

        self.store
            .get_mut(&type_id)
            .unwrap()
            .downcast_mut::<ComponentItem<T>>()
            .unwrap()
    }
    pub fn get_storage_mut<T: 'static>(&mut self) -> Option<&mut ComponentItem<T>> {
        let type_id = TypeId::of::<T>();

        self.store
            .get_mut(&type_id)?
            .downcast_mut::<ComponentItem<T>>()
    }

    pub fn get_storage<T: 'static>(&self) -> Option<&ComponentItem<T>> {
        let type_id = TypeId::of::<T>();

        self.store.get(&type_id)?.downcast_ref::<ComponentItem<T>>()
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        let component_item = match self.store.get_mut(&type_id) {
            Some(link) => link.downcast_mut::<ComponentItem<T>>(),
            None => None,
        };

        match component_item {
            Some(item) => item.links.get_mut(&entity),
            None => None,
        }
    }

    pub fn link<T: 'static>(&mut self, entity: Entity, component: T) {
        let item = self.get_or_insert_storage_mut::<T>();

        item.links.insert(entity, component);
    }
}
