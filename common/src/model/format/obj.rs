use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::NonZero,
    path::Path,
};

use glam::{Vec2, Vec3};
use tap::{Pipe, Tap};

use crate::model::triangle::{Mesh, Triangle, Vertex};

struct ObjVertex {
    position: Vec3,
    normals: Vec<Vec3>, // (normal, weight)
}

#[derive(Debug, Clone, Copy)]
struct ObjVertexWithNormal {
    position: Vec3,
    normal: Vec3,
}

#[derive(Debug, Clone, Copy)]
struct ObjFaceVertex {
    vertex_index: usize,
    uv_index: Option<usize>,
    normal_index: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
struct ObjFace {
    vertices: [ObjFaceVertex; 3],
    normal: Vec3,
    area: f32, // needed for smooth shading
}

fn parse_coords_list(c: &str) -> Option<Vec<f32>> {
    c.split_whitespace()
        .map(str::parse::<f32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()
}

fn parse_vertex_list(
    c: &str,
    num_vertices: usize,
    num_uvs: usize,
    num_normals: usize,
) -> Vec<ObjFaceVertex> {
    c.split_whitespace()
        .map(|c| {
            let mut k = c
                .split("/")
                .flat_map(|i| (!i.is_empty()).then(|| i.parse::<isize>().unwrap()));
            let vertex_index: usize = k.next().unwrap().pipe(|i| {
                if i.is_positive() {
                    (i as usize) - 1
                } else {
                    num_vertices - (-i as usize)
                }
            });
            let uv_index = k.next().map(|i| {
                if i.is_positive() {
                    (i as usize) - 1
                } else {
                    num_uvs - (-i as usize)
                }
            });
            let normal_index = k.next().map(|i| {
                if i.is_positive() {
                    (i as usize) - 1
                } else {
                    num_normals - (-i as usize)
                }
            });
            ObjFaceVertex {
                vertex_index,
                uv_index,
                normal_index,
            }
        })
        .collect()
}

fn parse_faces(vertex_list: Vec<ObjFaceVertex>, vertices: &[ObjVertex]) -> Vec<ObjFace> {
    let positions = vertex_list
        .iter()
        .map(|v| vertices[v.vertex_index].position)
        .collect::<Vec<_>>();

    (2..vertex_list.len())
        .map(|i| {
            let e1 = positions[i - 1] - positions[0];
            let e2 = positions[i] - positions[0];

            let cross = e1.cross(e2);
            let area = cross.length() / 2.0;
            let face_normal = e1.cross(e2).normalize();

            ObjFace {
                vertices: [vertex_list[0], vertex_list[i - 1], vertex_list[i]],
                normal: face_normal,
                area,
            }
        })
        .collect()
}

fn convert_face_to_triangle(
    face: &ObjFace,
    vertices: &[ObjVertexWithNormal],
    vertex_uvs: &[Vec2],
    vertex_normals: &[Vec3],
) -> Triangle {
    let vertices = face
        .vertices
        .iter()
        .map(|v| {
            let vertex = vertices[v.vertex_index];
            let uv = v.uv_index.map(|i| vertex_uvs[i]);
            let normal = v
                .normal_index
                .map(|i| vertex_normals[i])
                .unwrap_or_else(|| vertex.normal);
            // .unwrap_or_else(|| face.normal);

            Vertex {
                position: vertex.position,
                uv,
                normal,
            }
        })
        .collect::<Vec<_>>();

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

    let mut groups: Vec<Vec<ObjFace>> = Vec::new();

    let mut current_group: Option<Vec<ObjFace>> = None;

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
            let position = glam::Vec3::from_slice(&coords);

            vertices.push(ObjVertex {
                position,
                normals: vec![],
            });
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
                groups.push(mesh);
            }

            current_group = Some(Vec::new());
            continue;
        }

        if let Some(faces) = l
            .strip_prefix("f ")
            .map(|c| parse_vertex_list(c, vertices.len(), vertex_uvs.len(), vertex_normals.len()))
            .map(|obj_vertices| parse_faces(obj_vertices, &vertices))
        {
            let mut group = current_group.take().unwrap_or_default();

            for face in faces.iter() {
                for v in &face.vertices {
                    vertices[v.vertex_index]
                        .normals
                        .push(face.normal * face.area);
                }
            }

            group.extend(faces);
            current_group = Some(group);
            // break;
            continue;
        }

        println!("WARNING: unrecognized line: {l:?}")
    }

    // Also put the final group in the vec
    if let Some(group) = current_group.take() {
        groups.push(group);
    }

    // Calculate smooth vertex normals
    let vertices_with_normals = vertices
        .iter()
        .map(|v| ObjVertexWithNormal {
            position: v.position,
            normal: v.normals.iter().sum::<Vec3>().normalize(),
        })
        .collect::<Vec<_>>();

    // turn Vec<ObjFace> into Meshes
    groups
        .iter()
        .map(|g| {
            let triangles = g
                .iter()
                .map(|f| {
                    convert_face_to_triangle(
                        f,
                        &vertices_with_normals,
                        &vertex_uvs,
                        &vertex_normals,
                    )
                })
                .collect();
            Mesh::new(triangles)
        })
        .collect()
}
