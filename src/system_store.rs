use crate::system::System;

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
}
