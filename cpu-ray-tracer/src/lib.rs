use core::f32;

use tap::Tap;

use crate::{
    bvh::{Bvh, builder::BvhBuilder},
    intersect::Intersect,
};

mod bvh;
mod intersect;
mod ray;

pub struct CpuRayTracer {
    scene: common::scene::Scene,
    bvh: Bvh,
}

impl CpuRayTracer {
    pub fn new(scene: common::scene::Scene) -> Self {
        let bvh =
            BvhBuilder::new(scene.meshes().iter().flat_map(|m| &m.triangles).cloned()).build();
        Self { scene, bvh }
    }

    pub fn render(&self, surface: &mut common::surface::Surface) {
        surface.clear(common::surface::format::RGBA8::BLACK);

        let width = surface.width();
        let height = surface.height();

        let camera = self.scene.camera();

        // let x = 200;
        // let y = 150;

        for y in 0..height {
            for x in 0..width {
                let ndc = glam::Vec2::new(
                    (x as f32 + 0.5) / (width as f32),
                    -(y as f32 + 0.5) / (height as f32),
                ) * 2.0
                    + glam::Vec2::new(-1.0, 1.0);
                let ray = ray::Ray::from_camera(camera, ndc);

                // Disable the BVH for debug purposes

                // let mut closest = f32::INFINITY;

                // for m in self.scene.meshes() {
                //     for t in &m.triangles {
                //         if let Some(intersection) = t.intersect(&ray)
                //             && intersection.t < closest
                //         {
                //             if closest.is_finite() {
                //                 println!("overwriting pixel");
                //             }
                //             closest = intersection.t;
                //             // Simple shading based on angle to lightray
                //             let intensity = intersection
                //                 .normal
                //                 .dot(-ray.direction())
                //                 .tap(|i| println!("unclamped intensity: {i}"))
                //                 .clamp(0.0, 1.0);
                //             *surface.get_mut(x, y) = (glam::Vec3::ONE * intensity).into();
                //         }
                //     }
                // }

                if let Some(intersection) = self.bvh.intersect(&ray) {
                    // Simple shading based on angle to lightray
                    let intensity = intersection.normal.dot(-ray.direction()).clamp(0.0, 1.0);
                    let color = ((intersection.uv.x * 16.0).round()
                        + (intersection.uv.y * 16.0).round())
                        % 2.0;

                    *surface.get_mut(x, y) =
                        (glam::Vec3::ONE * (0.5 + color / 2.0) * intensity).into();
                }
            }
        }
    }
}
