use crate::{
    entity::{Drawable, Entity, Inputable, Updatable},
    frame::Frame,
    input::Input,
    rgb,
    vector::Vector2D,
};
use std::{cell::RefCell, rc::Rc};

const LINK_STIFFNESS: f32 = 10.0;
const LINK_DAMPING: f32 = 10.0;

#[derive(Default, Clone, Debug)]
struct SlimeCell {
    pos: Vector2D<f32>,
    size: f32,
    v: Vector2D<f32>,
    f: Vector2D<f32>,
    m: f32,
}

impl SlimeCell {
    fn new(pos: Vector2D<f32>, size: f32) -> Self {
        Self {
            pos,
            size,
            v: Vector2D { x: 0.0, y: 0.0 },
            f: Vector2D { x: 0.0, y: 0.0 },
            m: 1.0,
        }
    }
}

impl Updatable for SlimeCell {
    fn update(&mut self, dt: f32) {
        self.f = self.f.add(Vector2D { x: 0.0, y: 9.8 }.vmul(self.m));
        self.v = self.v.add(self.f.vmul(dt).vdiv(self.m));
        self.pos = self.pos.add(self.v.vmul(dt));
        self.f = Vector2D { x: 0.0, y: 0.0 };
    }
}

impl Drawable for SlimeCell {
    fn draw(&self, frame: &mut Frame) {
        let x: i32 = self.pos.x as i32;
        let y: i32 = self.pos.y as i32;
        let width: i32 = frame.width as i32;
        let height: i32 = frame.height as i32;
        if x >= 0 && y >= 0 && x < width && y < height as i32 {
            frame.buffer[(y * width + x) as usize] = rgb!(50, 190, 50);
        }
    }
}

#[derive(Default, Clone, Debug)]
struct SlimeLink {
    a: Rc<RefCell<SlimeCell>>,
    b: Rc<RefCell<SlimeCell>>,
    stiffness: f32,
    rest_length: f32,
    damping_factor: f32,
}

impl SlimeLink {
    fn new(
        a: Rc<RefCell<SlimeCell>>,
        b: Rc<RefCell<SlimeCell>>,
        stiffness: f32,
        damping_factor: f32,
    ) -> Self {
        let rest_length = a.borrow().pos.delta(b.borrow().pos).length();

        Self {
            a,
            b,
            stiffness,
            rest_length,
            damping_factor,
        }
    }

    fn apply_spring_force(&mut self) {
        let delta_length =
            self.a.borrow().pos.delta(self.b.borrow().pos).length() - self.rest_length;
        let f_stiffness = self.stiffness * delta_length;

        let normalize_a_to_b = self.a.borrow().pos.delta(self.b.borrow().pos).normalize();
        let normalize_b_to_a = self.b.borrow().pos.delta(self.a.borrow().pos).normalize();

        let delta_velocity = self.a.borrow().v.delta(self.b.borrow().v);
        let dot_product = normalize_a_to_b.dot(delta_velocity);
        let f_damping = dot_product * self.damping_factor;

        let f_total = f_stiffness + f_damping;

        let f_a = self.a.borrow().f.add(normalize_a_to_b.vmul(f_total));
        let f_b = self.b.borrow().f.add(normalize_b_to_a.vmul(f_total));

        self.a.borrow_mut().f = f_a;
        self.b.borrow_mut().f = f_b;
    }
}

#[derive(Default, Clone, Debug)]
pub struct Slime {
    cells: Vec<Rc<RefCell<SlimeCell>>>,
    links: Vec<SlimeLink>,
}

impl Slime {
    pub fn new(pos: Vector2D<i32>, size: Vector2D<usize>, cell_size: f32, cell_space: f32) -> Self {
        let mut cells: Vec<Rc<RefCell<SlimeCell>>> = Vec::new();
        let mut links: Vec<SlimeLink> = Vec::new();
        for offset_y in 0..size.y {
            for offset_x in 0..size.x {
                let new_cell = Rc::new(RefCell::new(SlimeCell::new(
                    Vector2D {
                        x: pos.x as f32 + offset_x as f32 * cell_space,
                        y: pos.y as f32 + offset_y as f32 * cell_space,
                    },
                    cell_size,
                )));

                if offset_x != 0 {
                    let other_cell = &cells[(offset_x - 1) + offset_y * size.x];

                    links.push(SlimeLink::new(
                        new_cell.clone(),
                        other_cell.clone(),
                        LINK_STIFFNESS,
                        LINK_DAMPING,
                    ));
                }
                if offset_y != 0 {
                    if offset_x != 0 {
                        let other_cell = &cells[(offset_x - 1) + (offset_y - 1) * size.x];

                        links.push(SlimeLink::new(
                            new_cell.clone(),
                            other_cell.clone(),
                            LINK_STIFFNESS,
                            LINK_DAMPING,
                        ));
                    }
                    {
                        let other_cell = &cells[(offset_x) + (offset_y - 1) * size.x];

                        links.push(SlimeLink::new(
                            new_cell.clone(),
                            other_cell.clone(),
                            LINK_STIFFNESS,
                            LINK_DAMPING,
                        ));
                    }
                    if offset_x + 1 != size.x {
                        let other_cell = &cells[(offset_x + 1) + (offset_y - 1) * size.x];

                        links.push(SlimeLink::new(
                            new_cell.clone(),
                            other_cell.clone(),
                            LINK_STIFFNESS,
                            LINK_DAMPING,
                        ));
                    }
                }

                cells.push(new_cell);
            }
        }

        Self { cells, links }
    }
}

impl Inputable for Slime {
    fn handle_input(&mut self, input: Input) {}
}

impl Updatable for Slime {
    fn update(&mut self, dt: f32) {
        for link in self.links.iter_mut() {
            link.apply_spring_force();
        }
        for cell in &self.cells {
            cell.borrow_mut().update(dt);
        }
    }
}

impl Drawable for Slime {
    fn draw(&self, frame: &mut Frame) {
        for cell in &self.cells {
            cell.borrow().draw(frame);
        }
    }
}

impl Entity for Slime {}
