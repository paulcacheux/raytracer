use crate::math::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn point_at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}