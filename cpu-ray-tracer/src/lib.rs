mod ray;
pub struct CpuRayTracer {
    scene: common::scene::Scene,
}

impl CpuRayTracer {
    pub fn new(scene: common::scene::Scene) -> Self {
        Self { scene }
    }

    pub fn render(&self, surface: &mut common::surface::Surface) {
        surface.clear(common::surface::format::RGBA8::BLACK);

        let width = surface.width();
        let height = surface.height();

        let camera = self.scene.camera();

        for y in 0..height {
            for x in 0..width {
                let ndc = glam::Vec2::new(
                    (x as f32 + 0.5) / (width as f32),
                    -(y as f32 + 0.5) / (height as f32),
                ) * 2.0
                    + glam::Vec2::new(-1.0, 1.0);
                let ray = ray::Ray::from_camera(camera, ndc);

                let mut closest_intersection = f32::INFINITY;
                for mesh in self.scene.meshes() {
                    for triangle in &mesh.triangles {
                        if let Some(t) = ray.intersect(triangle) {
                            if t < closest_intersection {
                                // Simple shading based on angle to light
                                let intensity = triangle.normal.dot(-ray.direction).clamp(0.0, 1.0);
                                *surface.get_mut(x, y) = (glam::Vec3::ONE * intensity).into();
                                closest_intersection = t;
                            }
                        }
                    }
                }
            }
        }
    }
}
