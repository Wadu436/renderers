use core::f32;

use crate::{
    bvh::{
        bounding_box::BoundingBox,
        triangle::{BvhTriangle, BvhTrianglesExt},
    },
    intersect::{Intersect, Intersection},
};

pub enum BvhNode {
    Leaf {
        bounding_box: BoundingBox,
        start_index: usize,
        end_index: usize,
    },
    Internal {
        bounding_box: BoundingBox,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

impl BvhNode {
    pub fn build_from_triangles(mut triangles: &mut [BvhTriangle], index: usize) -> Self {
        let bounding_box = triangles.bounding_box();
        if triangles.len() <= 2 {
            BvhNode::Leaf {
                bounding_box,
                start_index: index,
                end_index: index + triangles.len(),
            }
        } else {
            // Find the largest axis
            let size = triangles.size();
            let sort_axes = if size.x >= size.y && size.x >= size.z {
                if size.y >= size.z { [0, 1] } else { [0, 2] }
            } else if size.y >= size.x && size.y >= size.z {
                if size.x >= size.z { [1, 0] } else { [1, 2] }
            } else if size.x >= size.y {
                [2, 0]
            } else {
                [2, 1]
            };
            for &axis in sort_axes.iter().rev() {
                triangles.sort_by_axis(axis);
            }
            let mid = triangles.len() / 2;
            let (left_triangles, right_triangles) = triangles.split_at_mut(mid);

            let left_node = BvhNode::build_from_triangles(left_triangles, index);
            let right_node = BvhNode::build_from_triangles(right_triangles, index + mid);

            BvhNode::Internal {
                bounding_box,
                left: Box::new(left_node),
                right: Box::new(right_node),
            }
        }
    }
}
impl BvhNode {
    pub fn intersect(
        &self,
        triangles: &[BvhTriangle],
        ray: &crate::ray::Ray,
    ) -> Option<crate::intersect::Intersection> {
        match self {
            BvhNode::Leaf {
                bounding_box,
                start_index,
                end_index,
            } => {
                if bounding_box.intersect(ray) {
                    let mut closest_intersection: Option<Intersection> = None;
                    for t in &triangles[*start_index..*end_index] {
                        if let Some(intersection) = Intersect::intersect(&t.triangle, ray)
                            && intersection.t
                                < closest_intersection.map(|i| i.t).unwrap_or(f32::MAX)
                        {
                            closest_intersection = Some(intersection);
                        }
                    }
                    closest_intersection
                } else {
                    None
                }
            }
            BvhNode::Internal {
                bounding_box,
                left,
                right,
            } => {
                if bounding_box.intersect(ray) {
                    let left_intersection = left.intersect(triangles, ray);
                    let right_intersection = right.intersect(triangles, ray);
                    match (left_intersection, right_intersection) {
                        (None, None) => None,
                        (None, Some(i)) => Some(i),
                        (Some(i), None) => Some(i),
                        (Some(left), Some(right)) => {
                            if left.t < right.t {
                                Some(left)
                            } else {
                                Some(right)
                            }
                        }
                    }
                } else {
                    None
                }
            }
        }
    }
}
