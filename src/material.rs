use rand;
use rand::Rng;

use crate::ray::Ray;
use crate::math::*;
use crate::hitable::HitInfos;

#[derive(Debug, Clone, Copy)]
pub struct MaterialInfos {
    pub scattered: Ray,
    pub attenuation: Vector,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: Ray, infos: &HitInfos) -> Option<MaterialInfos>;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Vector
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, infos: &HitInfos) -> Option<MaterialInfos> {
        let target = infos.point + infos.normal + Vector::rand_in_unit_sphere(&mut rand::thread_rng());
        Some(MaterialInfos {
            scattered: Ray::new(infos.point, target - infos.point),
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Vector,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, infos: &HitInfos) -> Option<MaterialInfos> {
        let reflected = utils::reflect(ray.direction.normalized(), infos.normal);
        let scattered = Ray::new(infos.point, reflected + Vector::rand_in_unit_sphere(&mut rand::thread_rng()) * self.fuzz);
        let attenuation = self.albedo;
        if scattered.direction.dot(infos.normal) > 0.0 {
            Some(MaterialInfos {
                scattered,
                attenuation
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub ref_index: f32,
}

impl Dielectric {
    pub fn new(ref_index: f32) -> Dielectric {
        Dielectric { ref_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, infos: &HitInfos) -> Option<MaterialInfos> {
        let reflected = utils::reflect(ray.direction, infos.normal);
        let attenuation = Vector::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(infos.normal) > 0.0 {
            let cosine = self.ref_index * ray.direction.dot(infos.normal) / ray.direction.norm();
            (-infos.normal, self.ref_index, cosine)
        } else {
            let cosine = -ray.direction.dot(infos.normal) / ray.direction.norm();
            (infos.normal, 1.0 / self.ref_index, cosine)
        };

        let (reflect_prob, mut scattered) = if let Some(refracted) = utils::refract(ray.direction, outward_normal, ni_over_nt) {
            (utils::schlick(cosine, self.ref_index), Ray::new(infos.point, refracted))
        } else {
            (1.0, Ray::new(infos.point, reflected))
        };

        if rand::thread_rng().gen::<f32>() < reflect_prob {
            scattered = Ray::new(infos.point, reflected);
        }

        Some(MaterialInfos {
            scattered,
            attenuation
        })
    }
}

mod utils {
    use crate::math::*;

    pub fn reflect(v: Vector, n: Vector) -> Vector {
        v - n * 2.0 * v.dot(n)
    }

    pub fn refract(v: Vector, n: Vector, ni_over_nt: f32) -> Option<Vector> {
        let uv = v.normalized();
        let dt = uv.dot(n);
        let disc = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

        if disc > 0.0 {
            Some((uv - n * dt) * ni_over_nt - n * disc.sqrt())
        } else {
            None
        }
    }

    pub fn schlick(cosine: f32, ref_index: f32) -> f32 {
        let r0 = (1.0 - ref_index) / (1.0 + ref_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}