use crate::vector::Vector2D;

#[derive(Default, Clone, Debug)]
pub struct Polygon {
    pub points: Vec<Vector2D<f32>>,
}
