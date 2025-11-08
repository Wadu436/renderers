#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: glam::Vec3,
    view: glam::Vec3,

    up: glam::Vec3,
    right: glam::Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::new(0.0, 0.0, -1.0),
            glam::Vec3::new(0.0, 1.0, 0.0),
            90.0,
            16.0 / 9.0,
        )
    }
}

impl Camera {
    pub fn new(
        origin: glam::Vec3,
        view: glam::Vec3,
        up: glam::Vec3,
        horizontal_fov: f32, // in degrees
        aspect_ratio: f32,   // width/height, e.g. 16:9
    ) -> Self {
        let view = view.normalize();
        let right = view.cross(up).normalize();
        let up = right.cross(view).normalize();

        // Sanity checks
        assert!((right.dot(up).abs()) < 1e-3);
        assert!((right.dot(view).abs()) < 1e-3);
        assert!((up.dot(view).abs()) < 1e-3);
        assert!((right.length() - 1.0).abs() < 1e-3);
        assert!((up.length() - 1.0).abs() < 1e-3);
        assert!((view.length() - 1.0).abs() < 1e-3);

        let width = (horizontal_fov.to_radians() * 0.5).tan();
        let height = width / aspect_ratio;

        Self {
            origin,
            view,
            up: up * height,
            right: right * width,
        }
    }

    pub fn look_at(
        origin: glam::Vec3,
        target: glam::Vec3,
        up: glam::Vec3,
        horizontal_fov: f32, // in degrees
        aspect_ratio: f32,   // width/height, e.g. 16:9
    ) -> Self {
        let direction = (target - origin).normalize();
        Self::new(origin, direction, up, horizontal_fov, aspect_ratio)
    }

    pub fn ndc_to_viewing_direction(&self, ndc: glam::Vec2) -> glam::Vec3 {
        (self.view + ndc.x * self.right + ndc.y * self.up).normalize()
    }

    pub fn origin(&self) -> glam::Vec3 {
        self.origin
    }

    // Maps world space into NDC
    pub fn projection_matrix(&self) -> glam::Vec3 {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use glam::{Vec2, Vec3};

    use super::*;

    #[test]
    fn test_ndc() {
        // Forward/View: [1, 0, 0], Up: [0, 0, 1], Right: [0, -1, 0]
        // width is 2x height
        let camera = Camera::new(Vec3::ZERO, Vec3::X, Vec3::Z, 90.0, 2.0);

        // Test some NDCs
        {
            // Center
            let ndc = Vec2::new(0.0, 0.0);
            let viewing_direction = camera.ndc_to_viewing_direction(ndc);
            assert_eq!(
                viewing_direction,
                Vec3::new(1.0, 0.0, 0.0).normalize(),
                "center"
            )
        }

        {
            // Left
            let ndc = Vec2::new(-1.0, 0.0);
            let viewing_direction = camera.ndc_to_viewing_direction(ndc);
            assert_eq!(
                viewing_direction,
                Vec3::new(1.0, 1.0, 0.0).normalize(),
                "left"
            )
        }

        {
            // Right
            let ndc = Vec2::new(1.0, 0.0);
            let viewing_direction = camera.ndc_to_viewing_direction(ndc);
            assert_eq!(
                viewing_direction,
                Vec3::new(1.0, -1.0, 0.0).normalize(),
                "right"
            )
        }

        {
            // Top
            let ndc = Vec2::new(0.0, 1.0);
            let viewing_direction = camera.ndc_to_viewing_direction(ndc);
            assert_eq!(
                viewing_direction,
                Vec3::new(1.0, 0.0, 0.5).normalize(),
                "top"
            )
        }

        {
            // Bottom
            let ndc = Vec2::new(0.0, -1.0);
            let viewing_direction = camera.ndc_to_viewing_direction(ndc);
            assert_eq!(
                viewing_direction,
                Vec3::new(1.0, 0.0, -0.5).normalize(),
                "bottom"
            )
        }

        {
            // Top Left
            let ndc = Vec2::new(-1.0, 1.0);
            let viewing_direction = camera.ndc_to_viewing_direction(ndc);
            assert_eq!(
                viewing_direction,
                Vec3::new(1.0, 1.0, 0.5).normalize(),
                "top left"
            )
        }

        {
            // Bottom Right
            let ndc = Vec2::new(1.0, -1.0);
            let viewing_direction = camera.ndc_to_viewing_direction(ndc);
            assert_eq!(
                viewing_direction,
                Vec3::new(1.0, -1.0, -0.5).normalize(),
                "bottom right"
            )
        }
    }
}
