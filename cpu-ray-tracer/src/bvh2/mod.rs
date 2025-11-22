use core::f32;
use std::{cell::RefCell, num::NonZero};

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
        right_offset: u32,
    }, // 4 bytes
    Leaf {
        triangle_offset: u32,
        num_triangles: NonZero<u32>,
    }, // 8 bytes
}

thread_local! {
    static STACK: RefCell<Vec<(f32, u32)>> = RefCell::new(Vec::with_capacity(16));
}

impl Bvh {
    // TODO: figure out a way to make this non-allocating, instead of having to pass in a threadlocal stack
    fn intersect_loop(
        &self,
        stack: &mut Vec<(f32, u32)>,
        ray: &crate::ray::Ray,
    ) -> Option<Intersection> {
        let mut closest_intersection: Intersection = Intersection {
            t: f32::INFINITY,
            point: Vec3::ZERO,
            normal: Vec3::ZERO,
        };

        // Intersect the root node
        let mut next_item = Some((0.0, 0_u32));

        while let Some((distance, node_index)) = next_item.take().or_else(|| stack.pop()) {
            if distance > closest_intersection.t {
                continue;
            }
            let node = &self.nodes[node_index as usize];
            match node.kind {
                BvhNodeKind::Internal { right_offset, .. } => {
                    // Push the children on the stack
                    let left_offset = node_index + 1;

                    let left = &self.nodes[left_offset as usize];
                    let right = &self.nodes[right_offset as usize];

                    let left_distance = left.bounding_box.intersect(ray);
                    let right_distance = right.bounding_box.intersect(ray);

                    match (left_distance, right_distance) {
                        (Some(left_t), Some(right_t)) => {
                            if left_t < right_t {
                                stack.push((right_t, right_offset));
                                next_item = Some((left_t, left_offset));
                            } else {
                                stack.push((left_t, left_offset));
                                next_item = Some((right_t, right_offset))
                            }
                        }
                        (Some(t), _) => {
                            next_item = Some((t, left_offset));
                        }
                        (_, Some(t)) => {
                            next_item = Some((t, right_offset));
                        }
                        _ => {}
                    }
                }
                BvhNodeKind::Leaf {
                    triangle_offset,
                    num_triangles,
                } => {
                    // Intersect with some triangles
                    for i in
                        triangle_offset as usize..(triangle_offset + num_triangles.get()) as usize
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

        if closest_intersection.t.is_finite() {
            Some(closest_intersection)
        } else {
            None
        }
    }
}

impl Intersect for Bvh {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<Intersection> {
        // let mut stack = Vec::with_capacity(16);
        // return self.intersect_loop(&mut stack, ray);
        STACK.with_borrow_mut(|stack| self.intersect_loop(stack, ray))
    }
}

// Sources
// https://www.pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies#CompactBVHForTraversal
