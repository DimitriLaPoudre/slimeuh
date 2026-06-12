pub type Entity = u32;

pub struct EntityManager {
    next_id: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn create_entity(&mut self) -> Entity {
        let new_id = self.next_id;
        self.next_id += 1;
        new_id
    }
}
