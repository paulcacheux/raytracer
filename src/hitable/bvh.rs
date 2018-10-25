use crate::aabb::AABB;
use super::*;

use rand::{self, Rng};

enum BVHNode {
    Empty,
    Leaf(Box<dyn Hitable>),
    Pair(Box<dyn Hitable>, Box<dyn Hitable>),
}

impl BVHNode {
    fn hit_node(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitInfos> {
        match self {
            BVHNode::Empty => None,
            BVHNode::Leaf(sub) => sub.hit(ray, tmin, tmax),
            BVHNode::Pair(left, right) => {
                let left_infos = left.hit(ray, tmin, tmax);
                let right_infos = right.hit(ray, tmin, tmax);

                match (left_infos, right_infos) {
                    (Some(l), Some(r)) => if l.t <= r.t { Some(l) } else { Some(r) },
                    (_, Some(r)) => Some(r),
                    (Some(l), _) => Some(l),
                    (None, None) => None
                }
            }
        }
    }
}

pub struct BVH {
    node: BVHNode,
    aabb: Option<AABB>,
}

impl BVH {
    pub fn new(children: Vec<Box<dyn Hitable>>) -> BVH {
        if children.is_empty() {
            return BVH {
                node: BVHNode::Empty,
                aabb: None
            }
        }

        let mut children = children;

        let axis_rand: u32 = rand::thread_rng().gen_range(0, 3);
        let sort_func = match axis_rand {
            0 => utils::box_x_compare,
            1 => utils::box_y_compare,
            2 => utils::box_z_compare,
            _ => unreachable!(),
        };
        children.sort_by(sort_func);

        let (node, aabb) = if children.len() == 1 {
            let c = children.into_iter().next().unwrap();
            let bb = c.bounding_box();
            (BVHNode::Leaf(c), bb)
        } else if children.len() == 2 {
            let mut iter = children.into_iter();
            let c0 = iter.next().unwrap();
            let c1 = iter.next().unwrap();
            let bb0 = c0.bounding_box();
            let bb1 = c1.bounding_box();
            let bb = AABB::surrounding_opt(bb0, bb1);
            (BVHNode::Pair(c0, c1), bb)
        } else {
            let right_children = children.split_off(children.len() / 2);
            let left: Box<dyn Hitable> = Box::new(BVH::new(children));
            let right: Box<dyn Hitable> = Box::new(BVH::new(right_children));
            let bb = AABB::surrounding_opt(left.bounding_box(), right.bounding_box());
            (BVHNode::Pair(left, right), bb)
        };

        BVH {
            node,
            aabb
        }
    }
}

impl Hitable for BVH {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitInfos> {
        if let Some(aabb) = self.aabb {
            if !aabb.hit(ray, tmin, tmax) {
                return None
            }
        }

        self.node.hit_node(ray, tmin, tmax)
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.aabb
    }
}

mod utils {
    use std::cmp::Ordering;
    use crate::hitable::Hitable;

    pub fn fast_cmp(a: &f32, b: &f32) -> Ordering {
        if *a == *b {
            Ordering::Equal
        } else if *a <= *b {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    macro_rules! compare_xyz {
        ($component:ident, $name:ident) => {
            pub fn $name(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
                let a_box = a.bounding_box().unwrap(); // TODO
                let b_box = b.bounding_box().unwrap();

                fast_cmp(&a_box.min.$component, &b_box.min.$component)
            }
        }
    }

    compare_xyz!(x, box_x_compare);
    compare_xyz!(y, box_y_compare);
    compare_xyz!(z, box_z_compare);
}