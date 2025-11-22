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
