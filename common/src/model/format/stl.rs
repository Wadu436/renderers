use std::collections::HashMap;

use crate::{
    model::triangle::{Triangle, Vertex},
    util::BufGlamExt,
};
use bytes::{Buf, Bytes};
use glam::Vec3;

use crate::model::triangle::Mesh;

pub fn load_stl(mut bytes: Bytes) -> Mesh {
    bytes.advance(80); // Skip the header
    let num_triangles = bytes.get_u32_le();
    let mut bounding_box_min = Vec3::MAX;
    let mut bounding_box_max = Vec3::MIN;

    // (position, normals)
    let mut stl_vertices: HashMap<(usize, usize, usize), (glam::Vec3, Vec<glam::Vec3>)> =
        HashMap::new();
    let mut triangle_vertex_indices: Vec<(
        (usize, usize, usize),
        (usize, usize, usize),
        (usize, usize, usize),
    )> = vec![];

    // Returns the index in the vertices array
    let mut update_vertices = |position: glam::Vec3, normal: glam::Vec3| {
        // get the coords of this vertex
        let c1 = (position.x / 1e-5) as usize;
        let c2 = (position.y / 1e-5) as usize;
        let c3 = (position.z / 1e-5) as usize;

        let grid_coords = (c1, c2, c3);

        stl_vertices
            .entry(grid_coords)
            .and_modify(|e| e.1.push(normal))
            .or_insert_with(|| (position, vec![normal]));

        grid_coords
    };

    for _ in 0..num_triangles {
        let normal = bytes.get_vec3_le();
        let v1 = bytes.get_vec3_le();
        let v2 = bytes.get_vec3_le();
        let v3 = bytes.get_vec3_le();
        bytes.advance(2); // attribute byte count

        let v1_index = update_vertices(v1, normal);
        let v2_index = update_vertices(v2, normal);
        let v3_index = update_vertices(v3, normal);

        triangle_vertex_indices.push((v1_index, v2_index, v3_index));

        bounding_box_min = bounding_box_min.min(v1).min(v2).min(v3);
        bounding_box_max = bounding_box_max.max(v1).max(v2).max(v3);
    }

    // Process the vertices to get vertex normals
    let vertices = stl_vertices
        .into_iter()
        .map(|(c, (position, normals))| {
            (
                c,
                Vertex {
                    position,
                    normal: normals.iter().sum::<glam::Vec3>() / (normals.len() as f32),
                    uv: None,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let triangles = triangle_vertex_indices
        .into_iter()
        .map(|(i1, i2, i3)| Triangle {
            v1: vertices.get(&i1).unwrap().clone(),
            v2: vertices.get(&i2).unwrap().clone(),
            v3: vertices.get(&i3).unwrap().clone(),
        })
        .collect::<Vec<_>>();

    let center = (bounding_box_min + bounding_box_max) / 2.0;

    Mesh {
        triangles,
        bounding_box: (bounding_box_min, bounding_box_max),
        center,
    }
}
