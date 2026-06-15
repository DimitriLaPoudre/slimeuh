mod components;
mod ecs;
mod systems;

use std::time::Instant;

use crate::{
    components::{position::Position, render::Render},
    ecs::world::World,
    systems::{
        movement::Movement,
        renderer::{Renderer, RendererConfig},
    },
};

const TITLE: &str = "slime";
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const REFRESH: usize = 60;

fn main() {
    let mut world = World::new();

    world.add_system(Box::new(Movement {}));
    world.add_system(Box::new(Renderer::new(RendererConfig {
        title: String::from(TITLE),
        width: WIDTH,
        height: HEIGHT,
        refresh: REFRESH,
    })));

    let e = spawn!(
        world,
        Position { x: 10.0, y: 10.0 },
        Render {
            color: rgb!(255, 255, 255)
        }
    );

    let mut last = Instant::now();
    loop {
        let now = Instant::now();
        let dt = now.duration_since(last).as_secs_f32();
        last = now;

        world.run_systems(dt);
    }
}
