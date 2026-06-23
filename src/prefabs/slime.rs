use crate::components::collider::{Collider, ColliderForm};
use crate::components::force::Force;
use crate::components::mass::Mass;
use crate::components::render::RenderForm;
use crate::components::velocity::Velocity;
use crate::ecs::entity_manager::Entity;
use crate::rgb;
use crate::{
    components::{position::Position, render::Render},
    ecs::world::World,
    spawn,
    types::vector2d::Vector2D,
};

pub fn slime_spawn(
    w: &mut World,
    pos: Vector2D<i32>,
    size: Vector2D<usize>,
    cell_size: f32,
    cell_space: f32,
) {
    let mut entities: Vec<Entity> = vec![];
    for offset_y in 0..size.y {
        for offset_x in 0..size.x {
            let e = spawn!(
                w,
                Position {
                    x: pos.x as f32 + offset_x as f32 * cell_space,
                    y: pos.y as f32 + offset_y as f32 * cell_space,
                },
                Render {
                    color: rgb!(0, 255, 0),
                    form: RenderForm::Circle(0.5)
                },
                Mass { m: 1.0 },
                Velocity { x: 0.0, y: 0.0 },
                Force { x: 0.0, y: 0.0 },
                Collider {
                    form: ColliderForm::Circle(0.5)
                }
            );
            // if offset_x != 0 {
            //     let other_cell = &cells[(offset_x - 1) + offset_y * size.x];
            //
            //     links.push(SlimeLink::new(
            //         new_cell.clone(),
            //         other_cell.clone(),
            //         LINK_STIFFNESS,
            //         LINK_DAMPING,
            //     ));
            // }
            // if offset_y != 0 {
            //     if offset_x != 0 {
            //         let other_cell = &cells[(offset_x - 1) + (offset_y - 1) * size.x];
            //
            //         links.push(SlimeLink::new(
            //             new_cell.clone(),
            //             other_cell.clone(),
            //             LINK_STIFFNESS,
            //             LINK_DAMPING,
            //         ));
            //     }
            //     {
            //         let other_cell = &cells[(offset_x) + (offset_y - 1) * size.x];
            //
            //         links.push(SlimeLink::new(
            //             new_cell.clone(),
            //             other_cell.clone(),
            //             LINK_STIFFNESS,
            //             LINK_DAMPING,
            //         ));
            //     }
            //     if offset_x + 1 != size.x {
            //         let other_cell = &cells[(offset_x + 1) + (offset_y - 1) * size.x];
            //
            //         links.push(SlimeLink::new(
            //             new_cell.clone(),
            //             other_cell.clone(),
            //             LINK_STIFFNESS,
            //             LINK_DAMPING,
            //         ));
            //     }
            // }

            entities.push(e);
        }
    }
}
