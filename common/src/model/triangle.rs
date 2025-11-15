use std::collections::HashMap;

use crate::util::BufGlamExt;
use bytes::{Buf, Bytes};
use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
}

#[derive(Debug, Clone, Copy)]
// 72 bytes
pub struct Triangle {
    pub v1: Vertex, // 24 bytes
    pub v2: Vertex, // 24 bytes
    pub v3: Vertex, // 24 bytes
}

impl Triangle {
    // (t, u, v)
    pub fn intersect(&self, origin: glam::Vec3, direction: glam::Vec3) -> Option<(f32, f32, f32)> {
        let e1 = self.v2.position - self.v1.position;
        let e2 = self.v3.position - self.v1.position;

        let ray_cross_e2 = direction.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det.abs() < f32::EPSILON {
            // Ray is parallel to triangle
            return None;
        }

        let inv_det = 1.0 / det;
        let s = origin - self.v1.position;
        let u = inv_det * s.dot(ray_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            // Intersection lies outside the triangle
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * direction.dot(s_cross_e1);

        if v < 0.0 || u + v > 1.0 {
            // Intersection lies outside the triangle
            return None;
        }

        let t = inv_det * e2.dot(s_cross_e1);

        if t > f32::EPSILON {
            // ray intersection
            Some((t, u, v))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub bounding_box: (glam::Vec3, glam::Vec3),
    pub center: glam::Vec3,
}

impl Mesh {
    pub fn load_stl(mut bytes: Bytes) -> Self {
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

        Self {
            triangles,
            bounding_box: (bounding_box_min, bounding_box_max),
            center,
        }
    }
}
