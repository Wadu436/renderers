use core::f32;
use std::num::NonZeroU32;

use common::model::triangle::Triangle;

use super::{Bvh, BvhNode};
use crate::bvh2::{BvhNodeKind, bounding_box::BoundingBox};

#[derive(Debug, Clone)]
struct BvhPrimitive {
    triangle: Triangle,
    bounding_box: BoundingBox,
}

#[derive(Debug, Clone, Default)]
pub struct BvhBuilder {
    primitives: Vec<BvhPrimitive>,
}

impl BvhBuilder {
    pub fn new<I: Iterator<Item = Triangle>>(triangles: I) -> Self {
        // Ideas: sort the triangles/bounding boxes along a space filling curve to see if that results in better cache locality while building the BVH
        let primitives = triangles
            .map(|t| BvhPrimitive {
                bounding_box: BoundingBox::from(&t),
                triangle: t,
            })
            .collect();

        Self { primitives }
    }

    pub fn build(&self) -> Bvh {
        let indices = (0..self.primitives.len()).collect();
        let root = self.build_node(indices);

        // Now that we've made our splits, optimize the layout of the BVH for actual rendering
        let mut triangles: Vec<Triangle> = Vec::with_capacity(self.primitives.len());
        let mut nodes: Vec<BvhNode> = Vec::with_capacity(root.size());

        root.flatten(&mut triangles, &mut nodes);

        Bvh { nodes, triangles }
    }

    fn build_node<'a>(&'a self, indices: Vec<usize>) -> BvhBuilderNode<'a> {
        if indices.len() == 1 {
            let child = indices
                .get(0)
                .copied()
                .map(|i| &self.primitives[i])
                .unwrap();
            BvhBuilderNode {
                bounding_box: child.bounding_box.clone(),
                kind: BvhBuilderNodeKind::Leaf {
                    first_triangle: &child.triangle,
                    second_triangle: None,
                },
            }
        } else if indices.len() == 2 {
            let first = indices
                .get(0)
                .copied()
                .map(|i| &self.primitives[i])
                .unwrap(); // We should never get an empty indices list.
            let second = indices
                .get(1)
                .copied()
                .map(|i| &self.primitives[i])
                .unwrap();
            let bounding_box = first.bounding_box + second.bounding_box;

            BvhBuilderNode {
                bounding_box,
                kind: BvhBuilderNodeKind::Leaf {
                    first_triangle: &first.triangle,
                    second_triangle: Some(&second.triangle),
                },
            }
        } else {
            let (left_indices, right_indices) = split_along_optimal_axis(&self.primitives, indices);
            let left_child = Box::new(self.build_node(left_indices));
            let right_child = Box::new(self.build_node(right_indices));

            let bounding_box = left_child.bounding_box + right_child.bounding_box;
            BvhBuilderNode {
                bounding_box,
                kind: BvhBuilderNodeKind::Internal {
                    first_child: left_child,
                    second_child: right_child,
                },
            }
        }
    }
}

// Nodes
struct BvhBuilderNode<'a> {
    bounding_box: BoundingBox,
    kind: BvhBuilderNodeKind<'a>,
}

enum BvhBuilderNodeKind<'a> {
    Leaf {
        first_triangle: &'a Triangle,
        second_triangle: Option<&'a Triangle>,
    },
    Internal {
        first_child: Box<BvhBuilderNode<'a>>,
        second_child: Box<BvhBuilderNode<'a>>,
    },
}

impl<'a> BvhBuilderNode<'a> {
    pub fn size(&self) -> usize {
        // 1 (for itself) + size of each of the children
        let size_children = match &self.kind {
            BvhBuilderNodeKind::Leaf { .. } => 0, // triangles don't count as an additional node, they "belong" to the leaf node
            BvhBuilderNodeKind::Internal {
                first_child,
                second_child,
                ..
            } => first_child.size() + second_child.size(),
        };
        return 1 + size_children;
    }

    pub fn flatten(self, triangles: &mut Vec<Triangle>, nodes: &mut Vec<BvhNode>) {
        match self.kind {
            // if this is a leaf node, add the triangles to the triangle vector + put a Leaf node on the nodes array
            BvhBuilderNodeKind::Leaf {
                first_triangle,
                second_triangle: Some(second_triangle),
            } => {
                let triangle_offset = triangles.len();
                triangles.push(*first_triangle);
                triangles.push(*second_triangle);
                let node = BvhNode {
                    bounding_box: self.bounding_box,
                    kind: BvhNodeKind::Leaf {
                        triangle_offset: triangle_offset as u32,
                        num_triangles: NonZeroU32::new(2).unwrap(),
                    },
                };
                nodes.push(node);
            }
            BvhBuilderNodeKind::Leaf {
                first_triangle,
                second_triangle: None,
            } => {
                let triangle_offset = triangles.len();
                triangles.push(*first_triangle);
                let node = BvhNode {
                    bounding_box: self.bounding_box,
                    kind: BvhNodeKind::Leaf {
                        triangle_offset: triangle_offset as u32,
                        num_triangles: NonZeroU32::new(1).unwrap(),
                    },
                };
                nodes.push(node);
            }
            BvhBuilderNodeKind::Internal {
                first_child,
                second_child,
            } => {
                // Already put a node in the vector
                let node_index = nodes.len();
                nodes.push(BvhNode {
                    kind: BvhNodeKind::Internal {
                        right_offset: 0, // We don't know the offset yet. We'll set it later
                    },
                    bounding_box: self.bounding_box,
                });
                first_child.flatten(triangles, nodes);
                // get the index of the 2nd child
                let right_index = nodes.len();
                second_child.flatten(triangles, nodes);

                if let BvhNodeKind::Internal { right_offset, .. } = &mut nodes[node_index].kind {
                    *right_offset = right_index as u32 // Set the offset now that we've constructed the children
                } else {
                    unreachable!()
                }
            }
        };
    }
}

// Find the optimal splitting axis + split along that axis
fn split_along_optimal_axis(
    primitives: &[BvhPrimitive],
    indices: Vec<usize>,
) -> (Vec<usize>, Vec<usize>) {
    let best_score = f32::MAX;
    let mut best_indices = None;

    for axis in 0..=2 {
        let (indices_left, indices_right) = split_along_axis(primitives, indices.clone(), axis);
        let bounding_box_left =
            BoundingBox::from_iter(indices_left.iter().map(|&i| &primitives[i].bounding_box));
        let bounding_box_right =
            BoundingBox::from_iter(indices_right.iter().map(|&i| &primitives[i].bounding_box));
        let score = bounding_box_left.area() + bounding_box_right.area();
        if score < best_score {
            best_indices = Some((indices_left, indices_right));
        }
    }

    return best_indices.unwrap();
}

fn split_along_axis(
    primitives: &[BvhPrimitive],
    mut indices: Vec<usize>,
    axis: usize,
) -> (Vec<usize>, Vec<usize>) {
    assert!(axis <= 2, "Axis needs to be 0, 1, or 2.");

    indices.sort_by(|&a, &b| {
        primitives[a].bounding_box.min[axis]
            .partial_cmp(&primitives[b].bounding_box.min[axis])
            .unwrap()
    });

    // Split that shit down the middle
    let mid = indices.len() / 2;
    let (left, right) = indices.split_at(mid);
    (left.to_vec(), right.to_vec())
}
