use common::model::triangle::Triangle;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: glam::Vec3,
    pub max: glam::Vec3,
}
impl BoundingBox {
    pub fn intersect(&self, ray: &crate::ray::Ray) -> bool {
        self.intersect_naive(ray)
    }

    pub fn intersect_naive(&self, ray: &crate::ray::Ray) -> bool {
        let inv_dir = glam::Vec3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let x_range = self.min.x..self.max.x;
        let y_range = self.min.y..self.max.y;
        let z_range = self.min.z..self.max.z;

        {
            let t_min = (self.min - ray.origin) * inv_dir;
            let p_x = ray.at_t(t_min.x);
            let p_y = ray.at_t(t_min.y);
            let p_z = ray.at_t(t_min.z);
            if y_range.contains(&p_x.y) && z_range.contains(&p_x.z) {
                return true;
            }
            if x_range.contains(&p_y.x) && z_range.contains(&p_y.z) {
                return true;
            }
            if x_range.contains(&p_z.x) && y_range.contains(&p_z.y) {
                return true;
            }
        }

        {
            let t_max = (self.max - ray.origin) * inv_dir;
            let p_x = ray.at_t(t_max.x);
            let p_y = ray.at_t(t_max.x);
            let p_z = ray.at_t(t_max.x);
            if y_range.contains(&p_x.y) && z_range.contains(&p_x.z) {
                return true;
            }
            if x_range.contains(&p_y.x) && z_range.contains(&p_y.z) {
                return true;
            }
            if x_range.contains(&p_z.x) && y_range.contains(&p_z.y) {
                return true;
            }
        }

        false
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
