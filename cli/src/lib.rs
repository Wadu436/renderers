use color_eyre::eyre::Result;
use core::f32;
use cpu_rasterizer::CpuRasterizer;
use cpu_ray_tracer::CpuRayTracer;
use std::io::{Write, stdout};

use common::{
    camera::Camera,
    image::{ImageFormat, jxl::JpegXl, ppm},
    model::triangle::{Mesh, Triangle},
    scene::{Scene, SceneBuilder},
    surface::Surface,
};

use crate::arguments::output::OutputFormat;

pub mod arguments;

// const SCENE: (&str, f32) = ("./assets/cube.stl", 40.0);
const SCENE: (&str, f32) = ("./assets/teapot.stl", 10.0);

fn debug_scene(surface: &Surface) -> Scene {
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
        surface.width() as f32 / surface.height() as f32,
    );
    SceneBuilder::new()
        .with_camera(camera)
        .add_mesh(mesh)
        .build()
}

fn load_scene(surface: &Surface, camera_origin: Option<glam::Vec3>) -> Result<Scene> {
    let mesh = bytes::Bytes::from(std::fs::read(SCENE.0)?);
    let mesh = Mesh::load_stl(mesh);

    let camera = Camera::look_at(
        camera_origin.unwrap_or(glam::Vec3::new(SCENE.1, SCENE.1, SCENE.1)),
        mesh.center,
        glam::Vec3::Z,
        80.0,
        surface.width() as f32 / surface.height() as f32,
    );

    Ok(SceneBuilder::new()
        .with_camera(camera)
        .add_mesh(mesh)
        .build())
}

pub fn run(args: arguments::Args) -> Result<()> {
    // Set up
    let width = 400;
    let height = 300;
    let mut surface = Surface::new(width, height);

    // Render
    let camera_option =
        if let (Some(x), Some(y), Some(z)) = (args.camera_x, args.camera_y, args.camera_z) {
            Some(glam::Vec3::new(x, y, z))
        } else {
            None
        };

    let scene = if args.debug {
        debug_scene(&surface)
    } else {
        load_scene(&surface, camera_option)?
    };

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
