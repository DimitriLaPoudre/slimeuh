mod core;
mod entity;
mod frame;
mod input;
mod macros;
mod particle;
mod polygon;
mod slime;
mod spatial_grid;
mod tortilla;
mod vector;

use std::time::Instant;

use core::Core;
use particle::ParticleSystem;
use polygon::Polygon;
use slime::Slime;
use tortilla::Tortilla;
use vector::Vector2D;

const TITLE: &str = "slime";
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const REFRESH: usize = 60;

fn main() {
    let slime = Slime::new(
        Vector2D { x: 400, y: 300 },
        Vector2D { x: 10, y: 10 },
        0.1,
        2.0,
    );

    let polygon = Polygon {
        points: vec![Vector2D { x: 0.0, y: 0.0 }],
    };

    let mut core = Core::new(TITLE, WIDTH, HEIGHT, REFRESH);
    // core.add_entity(tortilla);
    // core.add_entity(particle_system);
    core.add_entity(slime);

    let mut last = Instant::now();
    while core.is_open() {
        let now = Instant::now();
        let dt = now.duration_since(last).as_secs_f32();
        last = now;

        core.analyze_event();
        core.update(dt);
        core.draw();
        core.next_frame();
    }
}
