use core::f32;

use common::model::triangle::Triangle;

use crate::{
    bvh2::bounding_box::BoundingBox,
    intersect::{Intersect, Intersection},
};

mod bounding_box;
pub mod builder;

pub struct Bvh {
    nodes: Vec<BvhNode>,
    triangles: Vec<Triangle>,
}

// 48 bytes
pub struct BvhNode {
    kind: BvhNodeKind,         // 24 bytes
    bounding_box: BoundingBox, // 24 bytes
}

enum BvhNodeKind {
    Internal {
        second_child_offset: usize,
    }, // 8 bytes
    Leaf {
        triangle_offset: usize,
        num_triangles: usize,
    }, // 16 bytes
}

impl Intersect for Bvh {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<Intersection> {
        let mut stack: Vec<usize> = Vec::new(); // Vec of offsets we should visit
        stack.push(0); // Start with the root node

        let mut closest_intersection: Option<Intersection> = None;

        // Idea: use a priority queue instead of a stack to visit closer nodes first

        while let Some(node_index) = stack.pop() {
            let node = &self.nodes[node_index];
            if node.bounding_box.intersect(ray).is_some() {
                match node.kind {
                    BvhNodeKind::Internal {
                        second_child_offset,
                    } => {
                        // Push the children on the stack
                        stack.push(second_child_offset); // Second child
                        stack.push(node_index + 1); // First child
                    }
                    BvhNodeKind::Leaf {
                        triangle_offset,
                        num_triangles,
                    } => {
                        // Intersect with some triangles
                        for i in triangle_offset..triangle_offset + num_triangles {
                            let triangle = &self.triangles[i];
                            if let Some(intersection) = Intersect::intersect(triangle, ray)
                                && intersection.t
                                    < closest_intersection.map(|c| c.t).unwrap_or(f32::INFINITY)
                            {
                                closest_intersection = Some(intersection);
                            }
                        }
                    }
                }
            }
        }

        return closest_intersection;
    }
}

// Sources
// https://www.pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies#CompactBVHForTraversal
