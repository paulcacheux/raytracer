use std::sync::Arc;

use crate::ray::{Ray, TimedRay};
use crate::math::*;
use crate::material::Material;

mod sphere;
pub use self::sphere::*;

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
}