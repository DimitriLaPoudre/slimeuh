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
    // let mut cells: Vec<Rc<RefCell<SlimeCell>>> = Vec::new();
    // let mut links: Vec<SlimeLink> = Vec::new();
    for offset_y in 0..size.y {
        for offset_x in 0..size.x {
            spawn!(
                w,
                Position {
                    x: pos.x as f32 + offset_x as f32 * cell_space,
                    y: pos.y as f32 + offset_y as f32 * cell_space,
                },
                Render {
                    color: rgb!(0, 255, 0)
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

            // cells.push(new_cell);
        }
    }
}
