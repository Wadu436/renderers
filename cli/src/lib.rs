use color_eyre::eyre::Result;
use core::f32;
use cpu_rasterizer::CpuRasterizer;
use cpu_ray_tracer::CpuRayTracer;
use std::io::{Write, stdout};

use common::{
    camera::Camera,
    image::{ImageFormat, jxl::JpegXl, ppm},
    model::triangle::{Mesh, Triangle},
    scene::SceneBuilder,
    surface::Surface,
};

use crate::arguments::output::OutputFormat;

pub mod arguments;

pub fn run(args: arguments::Args) -> Result<()> {
    // Set up
    let width = 400;
    let height = 300;
    let mut surface = Surface::new(width, height);

    // Debug scene
    let debug_scene = {
        // old single triangle replaced with a hexagon made of 6 triangles
        let hex_radius = 1.0;
        let vertices: Vec<glam::Vec3> = (0..6)
            .map(|i| {
                let angle = i as f32 * (f32::consts::PI * 2.0) / 6.0;
                glam::Vec3::new(hex_radius * angle.cos(), hex_radius * angle.sin(), 0.0)
            })
            .collect();

        let center = glam::Vec3::ZERO;
        let mut triangles = Vec::with_capacity(6);
        for i in 0..6 {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % 6];
            triangles.push(Triangle {
                normal: glam::Vec3::Z,
                // triangle fan: center -> v2 -> v1
                v1: center,
                v2,
                v3: v1,
            });
        }

        let mesh = Mesh {
            triangles,
            center,
            bounding_box: (
                glam::Vec3::new(-hex_radius, -hex_radius, 0.0),
                glam::Vec3::new(hex_radius, hex_radius, 0.0),
            ),
        };

        let camera = Camera::look_at(
            glam::Vec3::new(0.0, 0.0, 5.0),
            center,
            glam::Vec3::Y,
            60.0,
            width as f32 / height as f32,
        );
        SceneBuilder::new()
            .with_camera(camera)
            .add_mesh(mesh)
            .build()
    };

    // Render
    let mesh = bytes::Bytes::from(std::fs::read("./assets/teapot.stl")?);
    let mesh = Mesh::load_stl(mesh);

    let teapot_camera = Camera::look_at(
        glam::Vec3::new(10.0, 10.0, 10.0),
        mesh.center,
        glam::Vec3::Z,
        80.0,
        width as f32 / height as f32,
    );
    let cube_camera = Camera::look_at(
        glam::Vec3::new(40.0, 40.0, 40.0),
        mesh.center,
        glam::Vec3::Z,
        80.0,
        width as f32 / height as f32,
    );

    let scene = SceneBuilder::new()
        .with_camera(teapot_camera)
        .add_mesh(mesh)
        .build();
    // let scene = debug_scene;

    match args.renderer {
        arguments::renderer::Renderer::CpuRasterizer => {
            let renderer = CpuRasterizer::new(scene);
            renderer.render(&mut surface);
        }
        arguments::renderer::Renderer::CpuRayTracer => {
            let renderer = CpuRayTracer::new(scene);
            renderer.render(&mut surface);
        }
    }

    // Write the file
    let mut stdout = stdout();
    match args.format {
        OutputFormat::JpegXl => {
            let jxl = JpegXl { lossless: true };

            jxl.save(&surface, &mut stdout)?;
        }

        OutputFormat::Ppm => {
            let ppm = ppm::Ppm {
                format: ppm::PpmFormat::Binary,
            };
            ppm.save(&surface, &mut stdout)?;
            stdout.flush()?;
        }
    }

    Ok(())
}
