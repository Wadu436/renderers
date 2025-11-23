pub struct Ray {
    origin: glam::Vec3,
    direction: glam::Vec3,
}

impl Ray {
    #[inline]
    pub fn origin(&self) -> &glam::Vec3 {
        &self.origin
    }

    #[inline]
    pub fn direction(&self) -> &glam::Vec3 {
        &self.direction
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
