mod component_store;
mod entity;
mod position;
mod system;
mod system_store;
mod world;

use crate::{position::Position, world::World};

const TITLE: &str = "slime";
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const REFRESH: usize = 60;

fn main() {
    let mut world = World::new();

    let e = spawn!(world, Position { x: 10.0, y: 10.0 });

    return;
}
