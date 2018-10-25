use std::sync::Arc;

use crate::ray::Ray;
use crate::math::*;
use crate::material::Material;

mod sphere;
mod bvh;
mod aabb;
mod triangle;
pub use self::sphere::*;
pub use self::bvh::*;
pub use self::aabb::*;
pub use self::triangle::*;

pub struct HitInfos {
    pub t: f32,
    pub point: Point,
    pub normal: Vector,
    pub material: Arc<dyn Material>
}

impl HitInfos {
    pub fn min_max(t: f32, tmin: f32, tmax: f32, point: Point, normal: Vector, material: Arc<dyn Material>) -> Option<Self> {
        if tmin <= t && t <= tmax {
            Some(HitInfos { t, point, normal, material })
        } else {
            None
        }
    }
}

pub trait Hitable: Sync + Send {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitInfos>;
    fn bounding_box(&self) -> Option<AABB>;
}

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitInfos> {
        let mut infos = None;
        let mut tmax = tmax;

        for obj in self {
            if let Some(new_infos) = obj.hit(ray, tmin, tmax) {
                tmax = new_infos.t;
                infos = Some(new_infos);
            }
        }
        infos
    }

    fn bounding_box(&self) -> Option<AABB> {
        let mut final_box = None;

        for hitable in self {
            if let Some(next_bb) = hitable.bounding_box() {
                final_box = if let Some(bb) = final_box {
                    Some(AABB::surrounding(next_bb, bb))
                } else {
                    None 
                };
            } else {
                return None
            }
        }
        final_box
    }
}