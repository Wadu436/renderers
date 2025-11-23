#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub uv: Option<glam::Vec2>,
}

impl Vertex {
    pub fn new(position: glam::Vec3, normal: glam::Vec3, uv: Option<glam::Vec2>) -> Self {
        Self {
            position,
            normal,
            uv,
        }
    }
}

#[derive(Debug, Clone, Copy)]
// 72 bytes
pub struct Triangle {
    pub v1: Vertex, // 24 bytes
    pub v2: Vertex, // 24 bytes
    pub v3: Vertex, // 24 bytes
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub bounding_box: (glam::Vec3, glam::Vec3),
    pub center: glam::Vec3,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        assert!(!triangles.is_empty(), "Tried to build an empty mesh");

        let mut bb_min = glam::Vec3::INFINITY;
        let mut bb_max = glam::Vec3::NEG_INFINITY;
        for t in triangles.iter() {
            bb_min = bb_min.min(t.v1.position);
            bb_min = bb_min.min(t.v2.position);
            bb_min = bb_min.min(t.v3.position);

            bb_max = bb_max.max(t.v1.position);
            bb_max = bb_max.max(t.v2.position);
            bb_max = bb_max.max(t.v3.position);
        }

        let center = (bb_min + bb_max) / 2.0;

        Self {
            triangles,
            bounding_box: (bb_min, bb_max),
            center,
        }
    }
}
