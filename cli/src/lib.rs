use color_eyre::eyre::Result;
use core::f32;
use cpu_rasterizer::CpuRasterizer;
use cpu_ray_tracer::CpuRayTracer;
use std::io::{Write, stdout};

use common::{
    camera::Camera,
    image::{ImageFormat, jxl::JpegXl, ppm},
    model::triangle::Mesh,
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

    // Render
    let mesh = bytes::Bytes::from(std::fs::read("./assets/teapot.stl")?);
    let mesh = Mesh::load_stl(mesh);

    let camera = Camera::look_at(
        glam::Vec3::new(10.0, 10.0, 10.0),
        mesh.center,
        glam::Vec3::Z,
        80.0,
        width as f32 / height as f32,
    );

    let scene = SceneBuilder::new()
        .with_camera(camera)
        .add_mesh(mesh)
        .build();

    match args.renderer {
        arguments::renderer::Renderer::Rasterizer => {
            let renderer = CpuRasterizer::new(scene);
            renderer.render(&mut surface);
        }
        arguments::renderer::Renderer::RayTracer => {
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
