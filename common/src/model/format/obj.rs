use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::NonZero,
    path::Path,
};

use glam::{Vec2, Vec3};

use crate::model::triangle::{Mesh, Triangle, Vertex};

fn parse_coords_list(c: &str) -> Option<Vec<f32>> {
    c.split_whitespace()
        .map(str::parse::<f32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()
}

struct FaceVertex {
    vertex_index: NonZero<isize>,
    uv_index: Option<NonZero<isize>>,
    normal_index: Option<NonZero<isize>>,
}

fn parse_vertex_list(c: &str) -> Vec<FaceVertex> {
    c.split_whitespace()
        .map(|c| {
            let mut k = c
                .split("/")
                .flat_map(|i| (!i.is_empty()).then(|| i.parse::<isize>().unwrap()));
            let vertex_index = k.next().unwrap().try_into().unwrap();
            let uv_index = k.next().map(TryInto::try_into).map(Result::unwrap);
            let normal_index = k.next().map(TryInto::try_into).map(Result::unwrap);
            FaceVertex {
                vertex_index,
                uv_index,
                normal_index,
            }
        })
        .collect()
}

fn face_vertices_to_triangle(
    fs: [&FaceVertex; 3],
    vertices: &[Vec3],
    vertex_uvs: &[Vec2],
    vertex_normals: &[Vec3],
) -> Triangle {
    let positions: Vec<Vec3> = fs
        .iter()
        .map(|f| {
            if f.vertex_index.is_positive() {
                let index = (f.vertex_index.get() - 1) as usize;
                vertices[index]
            } else {
                let index = vertices.len() - ((-f.vertex_index.get()) as usize);
                vertices[index]
            }
        })
        .collect();

    let e1 = positions[1] - positions[0];
    let e2 = positions[2] - positions[0];
    let calculated_normal = e2.cross(e1).normalize();

    let normals: Vec<Vec3> = fs
        .iter()
        .map(|f| {
            f.normal_index
                .map(|normal_index| {
                    if normal_index.is_positive() {
                        let index = (normal_index.get() - 1) as usize;
                        vertex_normals[index]
                    } else {
                        let index = vertex_normals.len() - ((-normal_index.get()) as usize);
                        vertex_normals[index]
                    }
                })
                .unwrap_or(calculated_normal)
        })
        .collect();

    let uvs: Vec<_> = fs
        .iter()
        .map(|f| {
            f.uv_index.map(|uv_index| {
                if uv_index.is_positive() {
                    let index = (uv_index.get() - 1) as usize;
                    vertex_uvs[index]
                } else {
                    let index = vertex_uvs.len() - ((-uv_index.get()) as usize);
                    vertex_uvs[index]
                }
            })
        })
        .collect();

    let vertices: Vec<_> = positions
        .into_iter()
        .zip(normals)
        .zip(uvs)
        .map(|((p, n), uv)| Vertex::new(p, n, uv))
        .collect();

    Triangle {
        v1: vertices[0],
        v2: vertices[1],
        v3: vertices[2],
    }
}

pub fn load_obj<P: AsRef<Path>>(path: P) -> Vec<Mesh> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut vertices = vec![];
    let mut vertex_uvs = vec![];
    let mut vertex_normals = vec![];

    let mut meshes: Vec<Mesh> = Vec::new();

    let mut current_group: Option<Vec<Triangle>> = None;

    // Process the file line by line
    for l in reader.lines().map_while(Result::ok) {
        let l = l.trim();
        if l.is_empty() || l.starts_with("#") {
            // Empty line or comment
            continue;
        }
        // Vertex
        if let Some(coords) = l.strip_prefix("v ").and_then(parse_coords_list)
            && coords.len() >= 3
        {
            let vertex = glam::Vec3::from_slice(&coords);
            vertices.push(vertex);
            continue;
        }

        // Vertex Texture Coordinate
        if let Some(coords) = l.strip_prefix("vt ").and_then(parse_coords_list)
            && coords.len() >= 2
        {
            let uv = glam::Vec2::from_slice(&coords);
            vertex_uvs.push(uv);
            continue;
        }

        // Vertex Normals
        if let Some(coords) = l.strip_prefix("vn ").and_then(parse_coords_list)
            && coords.len() >= 3
        {
            let normal = glam::Vec3::from_slice(&coords);
            vertex_normals.push(normal);
            continue;
        }

        if let Some(name) = l.strip_prefix("g ").map(str::trim) {
            // If there's already a group, save it
            if let Some(mesh) = current_group.take() {
                meshes.push(Mesh::new(mesh));
            }

            current_group = Some(Vec::new());
            continue;
        }

        if let Some(vertex_indices) = l.strip_prefix("f ").map(parse_vertex_list)
            && vertex_indices.len() >= 3
        {
            let mut group = current_group.take().unwrap_or_else(|| vec![]);
            group.extend((2..vertex_indices.len()).map(|i| {
                face_vertices_to_triangle(
                    [
                        &vertex_indices[0],
                        &vertex_indices[i - 1],
                        &vertex_indices[i],
                    ],
                    &vertices,
                    &vertex_uvs,
                    &vertex_normals,
                )
            }));
            current_group = Some(group);
            continue;
        }

        println!("WARNING: unrecognized line: {l:?}")
    }

    // Also put the final group in the vec
    if let Some(mesh) = current_group.take() {
        meshes.push(Mesh::new(mesh));
    }

    meshes
}
