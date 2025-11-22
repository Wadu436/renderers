use color_eyre::eyre::Result;
use core::f32;
use cpu_rasterizer::CpuRasterizer;
use cpu_ray_tracer::CpuRayTracer;
use std::{
    fs::{OpenOptions, read_dir},
    io::{BufWriter, Write, stdout},
};

use common::{
    camera::Camera,
    image::{ImageFormat, jxl::JpegXl, ppm},
    model::{
        format::obj::load_obj,
        triangle::{Mesh, Triangle, Vertex},
    },
    scene::{Scene, SceneBuilder},
    surface::Surface,
};

use crate::arguments::output::OutputFormat;

pub mod arguments;

// const SCENE: (&str, f32) = ("./assets/cube.stl", 40.0);
// const SCENE: (&str, f32) = ("./assets/teapot.stl", 10.0);
const SCENE: (&str, glam::Vec3) = ("./assets/scenes/cube", glam::Vec3::new(2.0, 1.0, 1.0));

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
            // triangle fan: center -> v2 -> v1
            v1: Vertex {
                position: center,
                normal: glam::Vec3::Z,
                uv: None,
            },
            v2: Vertex {
                position: v2,
                normal: glam::Vec3::Z,
                uv: None,
            },
            v3: Vertex {
                position: v1,
                normal: glam::Vec3::Z,
                uv: None,
            },
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
    // List all the files in the directory
    let dir = read_dir(SCENE.0)?;

    let mut meshes = Vec::new();

    for entry in dir {
        let entry = entry?;
        let file_name = entry.file_name();
        if file_name.to_string_lossy().ends_with(".obj") {
            // Load the mesh
            let mesh = load_obj(entry.path());
            meshes.extend(mesh);
        }
    }

    let center = meshes.iter().map(|m| m.center).sum::<glam::Vec3>() / (meshes.len() as f32);

    let camera = Camera::look_at(
        camera_origin.unwrap_or(SCENE.1),
        center,
        glam::Vec3::Z,
        80.0,
        surface.width() as f32 / surface.height() as f32,
    );

    Ok(SceneBuilder::new()
        .with_camera(camera)
        .add_meshes(meshes)
        .build())
}

pub fn run(args: arguments::Args) -> Result<()> {
    // Set up
    let width = args.resolution_x.unwrap_or(1920);
    let height = args.resolution_y.unwrap_or(1080);
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
    let mut writer: Box<dyn Write> = if let Some(output) = args.output {
        Box::new(BufWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(output)?,
        ))
    } else {
        Box::new(BufWriter::new(stdout()))
    };
    match args.format {
        OutputFormat::JpegXl => {
            let jxl = JpegXl { lossless: true };

            jxl.save(&surface, &mut writer)?;
        }

        OutputFormat::Ppm => {
            let ppm = ppm::Ppm {
                format: ppm::PpmFormat::Binary,
            };
            ppm.save(&surface, &mut writer)?;
        }

        OutputFormat::None => {}
    }
    writer.flush()?;

    Ok(())
}
