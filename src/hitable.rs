use std::sync::Arc;

use crate::ray::Ray;
use crate::math::*;
use crate::material::Material;

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

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Arc<dyn Material>
}

impl Sphere {
    pub fn new<M: Material + 'static>(center: Point, radius: f32, material: M) -> Sphere {
        Sphere {
            center,
            radius,
            material: Arc::new(material)
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitInfos> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;

        let disc = b*b - 4.0 * a * c;
        if disc < 0.0 {
            None
        } else if disc == 0.0 {
            let t = (-b) / (2.0 * a);
            let point = ray.point_at(t);
            let normal = (point - self.center) / self.radius;
            HitInfos::min_max(t, tmin, tmax, point, normal, self.material.clone())
        } else {
            let t1 = (-b - disc.sqrt()) / (2.0 * a);
            let t2 = (-b + disc.sqrt()) / (2.0 * a);
            // let (t1, t2) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };

            let point1 = ray.point_at(t1);
            let point2 = ray.point_at(t2);
            let normal1 = (point1 - self.center) / self.radius;
            let normal2 = (point2 - self.center) / self.radius;

            let t1infos = HitInfos::min_max(t1, tmin, tmax, point1, normal1, self.material.clone());
            let t2infos = HitInfos::min_max(t2, tmin, tmax, point2, normal2, self.material.clone());
            t1infos.or(t2infos)
        }
    }
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