use crate::systems::System;

pub struct SystemStore {
    store: Vec<Box<dyn System>>,
}

impl SystemStore {
    pub fn new() -> Self {
        Self { store: vec![] }
    }

    pub fn add(&mut self, system: Box<dyn System>) {
        self.store.push(system);
    }

    pub fn get_mut(&mut self) -> &mut Vec<Box<dyn System>> {
        &mut self.store
    }
}
