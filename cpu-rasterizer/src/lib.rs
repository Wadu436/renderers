use core::f32;

use common::{
    scene::Scene,
    surface::{Surface, format::RGBA8},
};

pub struct CpuRasterizer {
    scene: Scene,
}

impl CpuRasterizer {
    pub fn new(scene: Scene) -> Self {
        Self { scene }
    }

    pub fn render(&self, surface: &mut Surface) {
        surface.clear(RGBA8::BLACK);

        let width = surface.width() as f32;
        let height = surface.height() as f32;
        let camera = self.scene.camera();
        let origin = camera.origin();

        let mut min_ndc = glam::Vec2::MAX;
        let mut max_ndc = glam::Vec2::MIN;

        for y in 0..surface.height() {
            for x in 0..surface.width() {
                let ndc = glam::Vec2::new((x as f32 + 0.5) / (width), -(y as f32 + 0.5) / (height))
                    * 2.0
                    + glam::Vec2::new(-1.0, 1.0);
                let direction = camera.ndc_to_viewing_direction(ndc);

                min_ndc = min_ndc.min(ndc);
                max_ndc = max_ndc.max(ndc);

                let mut closest_intersection = f32::INFINITY;
                for mesh in self.scene.meshes() {
                    for triangle in &mesh.triangles {
                        todo!()
                        // if let Some((t, _, _)) = triangle.intersect(origin, direction)
                        //     && t < closest_intersection
                        // {
                        //     // Add some shading!
                        //     *surface.get_mut(x, y) = (glam::Vec3::ONE).into();
                        //     closest_intersection = t;
                        // }
                    }
                }
            }
        }
    }
}
