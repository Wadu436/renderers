mod bounding_box;
mod node;
mod triangle;

use crate::intersect::Intersect;
use bounding_box::BoundingBox;
use node::BvhNode;
use triangle::BvhTriangle;

pub struct Bvh {
    triangles: Vec<BvhTriangle>,
    root: BvhNode,
}

impl Bvh {
    pub fn from_scene(scene: &common::scene::Scene) -> Self {
        let mut triangles = scene
            .meshes()
            .iter()
            .cloned()
            .flat_map(|m| m.triangles)
            .map(|t| BvhTriangle {
                triangle: t,
                bounding_box: BoundingBox::from_iter(&[t]),
            })
            .collect::<Vec<_>>();

        // Build BVH from scene meshes and their triangles
        Bvh {
            root: BvhNode::build_from_triangles(&mut triangles, 0),
            triangles,
        }
    }
}

impl Intersect for Bvh {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<crate::intersect::Intersection> {
        // Check intersection with the X-axis surfaces
        self.root.intersect(&self.triangles, ray)
    }
}
