use std::sync::Arc;

use super::*;

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
    material: Arc<dyn Material>
}

impl Triangle {
    pub fn new<M: Material + 'static>(a: Point, b: Point, c: Point, material: M) -> Self {
        Triangle {
            a,
            b,
            c,
            material: Arc::new(material)
        }
    }
}

impl Hitable for Triangle {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitInfos> {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;

        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a.abs() < 0.00001 {
            return None
        }

        let f = 1.0 / a;
        let s = ray.origin - self.a;
        let u = f * s.dot(h);

        if u < 0.0 /*|| u > 1.0*/ {
            return None
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None
        }

        let t = f * edge2.dot(q);
        let point = ray.point_at(t);
        let normal = edge1.cross(edge2).normalized();
        // println!("{:?}", normal);
        
        HitInfos::min_max(t, tmin, tmax, point, normal, self.material.clone())
    }

    fn bounding_box(&self) -> Option<AABB> {
        use super::bvh::utils::fast_cmp;

        let components = [self.a, self.b, self.c];
        let mut min_x = components.iter().map(|c| c.x).min_by(fast_cmp).unwrap();
        let mut min_y = components.iter().map(|c| c.y).min_by(fast_cmp).unwrap();
        let mut min_z = components.iter().map(|c| c.z).min_by(fast_cmp).unwrap();
        let mut max_x = components.iter().map(|c| c.x).max_by(fast_cmp).unwrap();
        let mut max_y = components.iter().map(|c| c.y).max_by(fast_cmp).unwrap();
        let mut max_z = components.iter().map(|c| c.z).max_by(fast_cmp).unwrap();

        const OFFSET: f32 = 0.001;
        if min_x == max_x {
            min_x -= OFFSET;
            max_x += OFFSET;
        }

        if min_y == max_y {
            min_y -= OFFSET;
            max_y += OFFSET;
        }

        if min_z == max_z {
            min_z -= OFFSET;
            max_z += OFFSET;
        }

        let min = Vector::new(min_x, min_y, min_z);
        let max = Vector::new(max_x, max_y, max_z);
        Some(AABB { min, max })
    }
}