use core::f32;
use std::num::NonZero;

use common::model::triangle::Triangle;
use glam::Vec3;

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

// 32 bytes
pub struct BvhNode {
    kind: BvhNodeKind,         // 8 bytes
    bounding_box: BoundingBox, // 24 bytes
}

enum BvhNodeKind {
    Internal {
        second_child_offset: u32,
        split_axis: u8, // X, Y, or Z
    }, // 8 bytes
    Leaf {
        triangle_offset: u32,
        num_triangles: NonZero<u32>,
    }, // 8 bytes
}

impl Intersect for Bvh {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<Intersection> {
        let mut stack: Vec<usize> = Vec::with_capacity(32); // Vec of offsets we should visit
        stack.push(0); // Start with the root node

        let mut closest_intersection: Intersection = Intersection {
            t: f32::INFINITY,
            point: Vec3::ZERO,
            normal: Vec3::ZERO,
        };

        // Idea: use a priority queue instead of a stack to visit closer nodes first

        while let Some(node_index) = stack.pop() {
            let node = &self.nodes[node_index];
            if let Some(t) = node.bounding_box.intersect(ray)
                && t < closest_intersection.t
            {
                match node.kind {
                    BvhNodeKind::Internal {
                        second_child_offset,
                        split_axis,
                    } => {
                        // Push the children on the stack
                        let first_child_offset = node_index + 1;
                        let second_child_offset = second_child_offset as usize;

                        let first_child = &self.nodes[first_child_offset];
                        let second_child = &self.nodes[second_child_offset];

                        let first_child_distance = (first_child.bounding_box.min
                            [split_axis as usize]
                            - ray.origin[split_axis as usize])
                            .abs()
                            .min(
                                (first_child.bounding_box.max[split_axis as usize]
                                    - ray.origin[split_axis as usize])
                                    .abs(),
                            );
                        let second_child_distance = (second_child.bounding_box.min
                            [split_axis as usize]
                            - ray.origin[split_axis as usize])
                            .abs()
                            .min(
                                (second_child.bounding_box.max[split_axis as usize]
                                    - ray.origin[split_axis as usize])
                                    .abs(),
                            );

                        if first_child_distance < second_child_distance {
                            stack.push(second_child_offset); // Second child
                            stack.push(first_child_offset); // First child
                        } else {
                            stack.push(first_child_offset); // First child
                            stack.push(second_child_offset); // Second child
                        }
                    }
                    BvhNodeKind::Leaf {
                        triangle_offset,
                        num_triangles,
                    } => {
                        // Intersect with some triangles
                        for i in triangle_offset as usize
                            ..(triangle_offset + num_triangles.get()) as usize
                        {
                            let triangle = &self.triangles[i];
                            if let Some(intersection) = Intersect::intersect(triangle, ray)
                                && intersection.t < closest_intersection.t
                            {
                                closest_intersection = intersection;
                            }
                        }
                    }
                }
            }
        }

        if closest_intersection.t.is_finite() {
            Some(closest_intersection)
        } else {
            None
        }
    }
}

// Sources
// https://www.pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies#CompactBVHForTraversal
