mod components;
mod ecs;
mod prefabs;
mod systems;
mod types;

use crate::{
    components::{position::Position, render::Render},
    ecs::world::World,
    prefabs::slime::{self, slime_spawn},
    systems::{
        gravity::Gravity, movement::Movement, renderer::RendererConfig, rendering::create_rendering,
    },
    types::vector2d::Vector2D,
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
    world.add_system(Box::new(Gravity {}));
    world.add_system(Box::new(Movement {}));
    world.add_system(Box::new(renderer));

    // let e = spawn!(
    //     world,
    //     Position { x: 10.0, y: 10.0 },
    //     Render {
    //         color: rgb!(255, 255, 255)
    //     }
    // );

    slime_spawn(
        &mut world,
        Vector2D { x: 10, y: 10 },
        Vector2D { x: 10, y: 10 },
        0.1,
        2.0,
    );

    world.run();
}
