use core::f32;

use crate::{
    bvh::{
        bounding_box::BoundingBox,
        triangle::{BvhTriangle, BvhTrianglesExt},
    },
    intersect::{Intersect, Intersection},
};

// Number of splits (children) per BVH node
// The degree of the BVH will be (approximately) 2^SPLITS_PER_NODE
const SPLITS_PER_NODE: u32 = 4;

pub enum BvhNode {
    Leaf {
        bounding_box: BoundingBox,
        start_index: usize,
        end_index: usize,
    },
    Internal {
        bounding_box: BoundingBox,
        children: Vec<BvhNode>,
    },
}

impl BvhNode {
    pub fn build_from_triangles(triangles: &mut [BvhTriangle], index: usize) -> Self {
        let bounding_box = triangles.bounding_box();
        if triangles.len() <= 2_usize.pow(SPLITS_PER_NODE) {
            BvhNode::Leaf {
                bounding_box,
                start_index: index,
                end_index: index + triangles.len(),
            }
        } else {
            fn split_triangles(
                mut triangles: &mut [BvhTriangle],
                index: usize,
                splits: u32,
            ) -> Vec<BvhNode> {
                if splits == 0 {
                    vec![BvhNode::build_from_triangles(triangles, index)]
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

                    let left_children = split_triangles(left_triangles, index, splits - 1);
                    let right_children = split_triangles(right_triangles, index + mid, splits - 1);

                    let mut children = left_children;
                    children.extend(right_children);
                    children
                }
            }

            BvhNode::Internal {
                bounding_box,
                children: split_triangles(triangles, index, SPLITS_PER_NODE),
            }
        }
    }
}
impl BvhNode {
    pub fn intersect(
        &self,
        triangles: &[BvhTriangle],
        ray: &crate::ray::Ray,
        closest_t: f32,
    ) -> Option<crate::intersect::Intersection> {
        match self {
            BvhNode::Leaf {
                bounding_box,
                start_index,
                end_index,
            } => {
                if let Some(t_bounding_box) = bounding_box.intersect(ray)
                    && t_bounding_box < closest_t
                {
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
                children,
            } => {
                if let Some(t_bounding_box) = bounding_box.intersect(ray)
                    && t_bounding_box < closest_t
                {
                    let mut closest_intersection: Option<Intersection> = None;
                    for child in children {
                        if let Some(intersection) = child.intersect(
                            triangles,
                            ray,
                            closest_intersection.map(|i| i.t).unwrap_or(closest_t),
                        ) && intersection.t
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
        }
    }
}
