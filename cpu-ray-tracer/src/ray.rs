pub struct Ray {
    pub origin: glam::Vec3,
    pub direction: glam::Vec3,
}

impl Ray {
    pub fn intersect(&self, triangle: &common::model::triangle::Triangle) -> Option<f32> {
        triangle.intersect(self.origin, self.direction)
    }

    pub fn new(origin: glam::Vec3, direction: glam::Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn from_camera(camera: &common::camera::Camera, ndc: glam::Vec2) -> Self {
        let origin = camera.origin();
        let direction = camera.ndc_to_viewing_direction(ndc);
        Self::new(origin, direction)
    }
}
