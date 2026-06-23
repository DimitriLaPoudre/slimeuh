mod components;
mod ecs;
mod prefabs;
mod systems;
mod types;

use crate::{
    components::{
        collider::{Collider, ColliderForm},
        force::Force,
        mass::Mass,
        position::Position,
        render::{Render, RenderForm},
        velocity::Velocity,
    },
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

    // slime_spawn(
    //     &mut world,
    //     Vector2D { x: 10, y: 10 },
    //     Vector2D { x: 10, y: 10 },
    //     0.1,
    //     2.0,
    // );
    spawn!(
        world,
        Position { x: 10.0, y: 10.0 },
        Render {
            color: rgb!(0, 255, 0),
            form: RenderForm::Circle(10.0)
        },
        Mass { m: 1.0 },
        Velocity { x: 0.0, y: 0.0 },
        Force { x: 0.0, y: 0.0 },
        Collider {
            form: ColliderForm::Circle(10.0)
        }
    );

    spawn!(
        world,
        Position { x: 50.0, y: 10.0 },
        Render {
            color: rgb!(0, 255, 0),
            form: RenderForm::Circle(10.0)
        },
        Mass { m: 1.0 },
        Velocity { x: 0.0, y: 0.0 },
        Force { x: 0.0, y: 0.0 },
        Collider {
            form: ColliderForm::Circle(10.0)
        }
    );

    world.run();
}
