use crate::util::BufGlamExt;
use bytes::{Buf, Bytes};
use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v1: glam::Vec3,
    pub v2: glam::Vec3,
    pub v3: glam::Vec3,
    pub normal: glam::Vec3,
}

impl Triangle {
    pub fn intersect(&self, origin: glam::Vec3, direction: glam::Vec3) -> Option<f32> {
        let e1 = self.v2 - self.v1;
        let e2 = self.v3 - self.v1;

        let ray_cross_e2 = direction.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det.abs() < f32::EPSILON {
            // Ray is parallel to triangle
            return None;
        }

        let inv_det = 1.0 / det;
        let s = origin - self.v1;
        let u = inv_det * s.dot(ray_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            // Intersection lies outside the triangle
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * direction.dot(s_cross_e1);

        if v < 0.0 || u + v > 1.0 {
            // Intersection lies outside the triangle
            return None;
        }

        let t = inv_det * e2.dot(s_cross_e1);

        if t > f32::EPSILON {
            // ray intersection
            return Some(t);
        } else {
            return None;
        }
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub bounding_box: (glam::Vec3, glam::Vec3),
    pub center: glam::Vec3,
}

impl Mesh {
    pub fn load_stl(mut bytes: Bytes) -> Self {
        bytes.advance(80); // Skip the header
        let num_triangles = bytes.get_u32_le();
        let mut triangles = Vec::with_capacity(num_triangles as usize);
        let mut bounding_box_min = Vec3::MAX;
        let mut bounding_box_max = Vec3::MIN;

        for _ in 0..num_triangles {
            let tri = Triangle {
                normal: bytes.get_vec3_le(),
                v1: bytes.get_vec3_le(),
                v2: bytes.get_vec3_le(),
                v3: bytes.get_vec3_le(),
            };
            bytes.advance(2); // attribute byte count
            bounding_box_min = bounding_box_min.min(tri.v1);
            bounding_box_max = bounding_box_max.max(tri.v1);
            bounding_box_min = bounding_box_min.min(tri.v2);
            bounding_box_max = bounding_box_max.max(tri.v2);
            bounding_box_min = bounding_box_min.min(tri.v3);
            bounding_box_max = bounding_box_max.max(tri.v3);
            triangles.push(tri);
        }

        let center = (bounding_box_min + bounding_box_max) / 2.0;

        Self {
            triangles,
            bounding_box: (bounding_box_min, bounding_box_max),
            center,
        }
    }
}
