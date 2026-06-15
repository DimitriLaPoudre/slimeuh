use crate::ecs::{component_store::ComponentStore, entity::Entity};

pub trait Query {
    fn execute(store: &ComponentStore) -> Vec<Entity>;
}

impl<A: 'static> Query for &A {
    fn execute(store: &ComponentStore) -> Vec<Entity> {
        let a = match store.get_storage::<A>() {
            Some(a) => a,
            None => return Vec::new(),
        };
        a.links.keys().copied().collect()
    }
}

impl<A: 'static, B: 'static> Query for (&A, &B) {
    fn execute(store: &ComponentStore) -> Vec<Entity> {
        let a = match store.get_storage::<A>() {
            Some(a) => a,
            None => return Vec::new(),
        };
        let b = match store.get_storage::<B>() {
            Some(b) => b,
            None => return Vec::new(),
        };

        a.links
            .keys()
            .filter(|e| b.links.contains_key(*e))
            .copied()
            .collect()
    }
}

impl<A: 'static, B: 'static, C: 'static> Query for (&A, &B, &C) {
    fn execute(store: &ComponentStore) -> Vec<Entity> {
        let a = match store.get_storage::<A>() {
            Some(a) => a,
            None => return Vec::new(),
        };
        let b = match store.get_storage::<B>() {
            Some(b) => b,
            None => return Vec::new(),
        };
        let c = match store.get_storage::<C>() {
            Some(c) => c,
            None => return Vec::new(),
        };

        a.links
            .keys()
            .filter(|e| b.links.contains_key(*e) && c.links.contains_key(*e))
            .copied()
            .collect()
    }
}
