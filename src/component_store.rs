use anyhow::{Result, anyhow};

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::entity::Entity;

pub struct ComponentStore {
    store: HashMap<TypeId, Box<dyn Any>>,
}

struct ComponentItem<T> {
    links: HashMap<Entity, T>,
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get_storage_mut<T: 'static>(&mut self) -> Option<&mut ComponentItem<T>> {
        let type_id = TypeId::of::<T>();

        self.store.entry(type_id).or_insert_with(|| {
            Box::new(ComponentItem::<T> {
                links: HashMap::new(),
            }) as Box<dyn Any>
        });

        self.store
            .get_mut(&type_id)
            .and_then(|links| links.downcast_mut::<ComponentItem<T>>())
    }

    pub fn link<T: 'static>(&mut self, entity: Entity, component: T) -> Result<()> {
        let mut item = self
            .get_storage_mut::<T>()
            .ok_or(anyhow!("ComponentStore::get_storage_mut failed"))?;

        item.links.insert(entity, component);
        Ok(())
    }
}
