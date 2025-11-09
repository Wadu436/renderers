use std::ops::Add;

use common::model::triangle::Triangle;
use glam::Vec3;

#[derive(Debug, Clone, Copy)]
// 24 bytes
pub struct BoundingBox {
    pub min: glam::Vec3, // 12 bytes
    pub max: glam::Vec3, // 12 bytes
}
impl BoundingBox {
    pub fn intersect(&self, ray: &crate::ray::Ray) -> Option<f32> {
        // https://en.wikipedia.org/wiki/Slab_method
        let inv_dir = 1.0 / ray.direction;

        let t_min_slabs = (self.min - ray.origin) * inv_dir;
        let t_max_slabs = (self.max - ray.origin) * inv_dir;

        let t_close_slabs = t_min_slabs.min(t_max_slabs);
        let t_far_slabs = t_min_slabs.max(t_max_slabs);

        let t_close = t_close_slabs.max_element();
        let t_far = t_far_slabs.min_element();

        if t_close > 0.0 && t_close <= t_far {
            Some(t_close)
        } else {
            None
        }
    }

    pub fn area(&self) -> f32 {
        (self.max - self.min).element_product()
    }
}

impl From<&Triangle> for BoundingBox {
    fn from(value: &Triangle) -> Self {
        let min = value.v1.min(value.v2).min(value.v3);
        let max = value.v1.max(value.v2).max(value.v3);
        return Self { min, max };
    }
}
impl From<Triangle> for BoundingBox {
    fn from(value: Triangle) -> Self {
        (&value).into()
    }
}

impl<'a, V: Into<&'a BoundingBox>> FromIterator<V> for BoundingBox {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        iter.into_iter().map(|v| v.into()).fold(
            BoundingBox {
                min: Vec3::INFINITY,
                max: Vec3::NEG_INFINITY,
            },
            |mut acc, v| {
                acc.min = acc.min.min(v.min);
                acc.max = acc.max.max(v.max);
                acc
            },
        )
    }
}

impl Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }
}
