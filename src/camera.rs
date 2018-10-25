use rand;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Camera {
    lower_left_corner: Point,
    origin: Point,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vector, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = aperture / 2.0;
        
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        
        let w = (lookfrom - lookat).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let lower_left_corner = lookfrom - u * focus_dist * half_width - v * focus_dist * half_height - w * focus_dist;
        let horizontal = u * half_width * 2.0 * focus_dist;
        let vertical = v * half_height * 2.0 * focus_dist;

        Camera {
            lower_left_corner,
            origin: lookfrom,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vector::rand_in_unit_disk(&mut rand::thread_rng()) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let dir = self.lower_left_corner.as_vector() + self.horizontal * s + self.vertical * t - self.origin.as_vector() - offset;
        Ray::new(self.origin + offset, dir)
    }
}