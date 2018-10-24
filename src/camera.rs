use crate::ray::Ray;
use crate::math::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub lower_left_corner: Point,
    pub origin: Point,
    pub horizontal: Vector,
    pub vertical: Vector,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let dir = self.lower_left_corner.as_vector() + self.horizontal * u + self.vertical * v;
        Ray::new(self.origin, dir)
    }
}