mod components;
mod ecs;
mod systems;
mod types;

use crate::{
    components::{position::Position, render::Render},
    ecs::world::World,
    systems::{movement::Movement, renderer::RendererConfig, rendering::create_rendering},
};

const TITLE: &str = "slime";
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const REFRESH: usize = 60;

fn main() {
    let mut world = World::new();

    let (input, renderer) = create_rendering(RendererConfig {
        title: String::from(TITLE),
        width: WIDTH,
        height: HEIGHT,
        refresh: REFRESH,
    });

    world.add_system(Box::new(input));
    world.add_system(Box::new(Movement {}));
    world.add_system(Box::new(renderer));

    let e = spawn!(
        world,
        Position { x: 10.0, y: 10.0 },
        Render {
            color: rgb!(255, 255, 255)
        }
    );

    println!("{:#?}", world.data.entity_manager);

    world.run();
}
