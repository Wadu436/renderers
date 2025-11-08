use common::model::triangle::Triangle;

use crate::bvh::bounding_box::BoundingBox;

#[derive(Debug, Clone, Copy)]
pub struct BvhTriangle {
    pub triangle: Triangle,
    pub bounding_box: BoundingBox,
}

pub trait BvhTrianglesExt {
    fn bounding_box(&self) -> BoundingBox;
    fn size(&self) -> glam::Vec3;
    fn sort_by_axis(&mut self, axis: usize);
}

impl BvhTrianglesExt for &mut [BvhTriangle] {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::from_iter(self.iter().map(|t| &t.bounding_box))
    }

    fn size(&self) -> glam::Vec3 {
        let bb = self.bounding_box();
        bb.max - bb.min
    }

    fn sort_by_axis(&mut self, axis: usize) {
        if axis > 2 {
            panic!("Axis must be 0, 1, or 2");
        }
        self.sort_by(|a, b| {
            a.bounding_box.min[axis]
                .partial_cmp(&b.bounding_box.min[axis])
                .unwrap()
        });
    }
}
