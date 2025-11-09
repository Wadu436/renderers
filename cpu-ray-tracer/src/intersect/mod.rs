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
        if let Some(t) = self.intersect(ray.origin, ray.direction) {
            let point = ray.origin + ray.direction * t;
            Some(crate::intersect::Intersection {
                t,
                point,
                normal: self.normal,
            })
        } else {
            None
        }
    }
}
