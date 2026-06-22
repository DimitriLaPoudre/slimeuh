use std::collections::HashMap;

pub type Entity = u32;

#[derive(Debug)]
pub struct EntityManager {
    next_id: u32,
    pub entities: HashMap<Entity, u64>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            entities: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let new_id = self.next_id;
        self.next_id += 1;
        self.entities.insert(new_id, 0);
        new_id
    }
}
