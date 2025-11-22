pub struct Ray {
    pub origin: glam::Vec3,
    pub direction: glam::Vec3,
}

impl Ray {
    pub fn new(origin: glam::Vec3, direction: glam::Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn from_camera(camera: &common::camera::Camera, ndc: glam::Vec2) -> Self {
        let origin = camera.origin();
        let direction = camera.ndc_to_viewing_direction(ndc);

        // return Ray::new(origin, direction);
        Ray { origin, direction }
    }

    pub fn at_t(&self, t: f32) -> glam::Vec3 {
        self.origin + t * self.direction
    }
}
