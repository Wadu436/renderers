#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t: f32,
    #[allow(dead_code)]
    pub point: glam::Vec3,
    pub normal: glam::Vec3,
}

pub trait Intersect {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<crate::intersect::Intersection>;
}

impl Intersect for common::model::triangle::Triangle {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<crate::intersect::Intersection> {
        let e1 = self.v2.position - self.v1.position;
        let e2 = self.v3.position - self.v1.position;

        let ray_cross_e2 = ray.direction().cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det.abs() < f32::EPSILON {
            // Ray is parallel to triangle
            // println!("None triangle intersection");
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin() - self.v1.position;
        let u = inv_det * s.dot(ray_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            // Intersection lies outside the triangle
            // println!("None triangle intersection");
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * ray.direction().dot(s_cross_e1);

        if v < 0.0 || u + v > 1.0 {
            // Intersection lies outside the triangle
            // println!("None triangle intersection");
            return None;
        }

        let t = inv_det * e2.dot(s_cross_e1);

        if t > f32::EPSILON {
            // ray intersection
            // println!("Some triangle intersection");
            let point = ray.at_t(t);
            Some(crate::intersect::Intersection {
                t,
                point,
                normal: self.v1.normal * (1.0 - u - v) + self.v2.normal * u + self.v3.normal * v,
            })
        } else {
            // println!("None triangle intersection");
            None
        }
    }
}
