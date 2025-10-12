use core::num;
use std::io::Read;

use crate::util::BufGlamExt;
use bytes::{Buf, Bytes};
pub struct Triangle {
    pub v1: glam::Vec3,
    pub v2: glam::Vec3,
    pub v3: glam::Vec3,
    pub normal: glam::Vec3,
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn load_stl(mut bytes: Bytes) -> Self {
        bytes.advance(80); // Skip the header
        let num_triangles = bytes.get_u32_le();
        let mut triangles = Vec::with_capacity(num_triangles as usize);

        for _ in 0..num_triangles {
            triangles.push(Triangle {
                normal: bytes.get_vec3_le(),
                v1: bytes.get_vec3_le(),
                v2: bytes.get_vec3_le(),
                v3: bytes.get_vec3_le(),
            });
        }

        Self { triangles }
    }
}
