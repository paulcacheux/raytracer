use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vector,
    pub max: Vector,
}

impl AABB {
    pub fn new(min: Vector, max: Vector) -> Self {
        AABB { min, max }
    }

    pub fn hit(self, ray: Ray, tmin: f32, tmax: f32) -> bool {
        macro_rules! inner {
            ($component:ident, $tmin:ident, $tmax:ident) => {
                let inv_d = 1.0 / ray.direction.$component;
                let mut t0 = (self.min.$component - ray.origin.$component) * inv_d;
                let mut t1 = (self.max.$component - ray.origin.$component) * inv_d;
                if inv_d < 0.0 {
                    std::mem::swap(&mut t0, &mut t1);
                }
                $tmin = utils::fast_max(t0, $tmin);
                $tmax = utils::fast_min(t1, $tmax);
                if $tmax <= $tmin {
                    return false;
                }
            }
        }

        let (mut tmin, mut tmax) = (tmin, tmax);

        inner!(x, tmin, tmax);
        inner!(y, tmin, tmax);
        inner!(z, tmin, tmax);
        true
    }

    pub fn surrounding(a: AABB, b: AABB) -> AABB {
        let small = Vector::new(utils::fast_min(a.min.x, b.min.x), utils::fast_min(a.min.y, b.min.y), utils::fast_min(a.min.z, b.min.z));
        let big = Vector::new(utils::fast_max(a.max.x, b.max.x), utils::fast_max(a.max.y, b.max.y), utils::fast_max(a.max.z, b.max.z));
        AABB::new(small, big)
    }

    pub fn surrounding_opt(a: Option<AABB>, b: Option<AABB>) -> Option<AABB> {
        if let (Some(aa), Some(bb)) = (a, b) {
            Some(AABB::surrounding(aa, bb))
        } else {
            None
        }
    }
}

mod utils {
    pub fn fast_min(a: f32, b: f32) -> f32 {
        if a < b { a } else { b }
    }

    pub fn fast_max(a: f32, b: f32) -> f32 {
        if a > b { a } else { b }
    }
}