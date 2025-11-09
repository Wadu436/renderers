use common::model::triangle::Triangle;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: glam::Vec3,
    pub max: glam::Vec3,
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
}

impl<'a> FromIterator<&'a Triangle> for BoundingBox {
    fn from_iter<I: IntoIterator<Item = &'a Triangle>>(iter: I) -> Self {
        let mut min = glam::Vec3::splat(f32::INFINITY);
        let mut max = glam::Vec3::splat(f32::NEG_INFINITY);

        for triangle in iter {
            for &vertex in &[triangle.v1, triangle.v2, triangle.v3] {
                min = min.min(vertex);
                max = max.max(vertex);
            }
        }

        Self { min, max }
    }
}

impl<'a> FromIterator<&'a BoundingBox> for BoundingBox {
    fn from_iter<I: IntoIterator<Item = &'a BoundingBox>>(iter: I) -> Self {
        let mut min = glam::Vec3::splat(f32::INFINITY);
        let mut max = glam::Vec3::splat(f32::NEG_INFINITY);

        for bb in iter {
            min = min.min(bb.min);
            max = max.max(bb.max);
        }

        Self { min, max }
    }
}
