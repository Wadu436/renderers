use common::surface::{Surface, format::RGBA8};

pub struct Rasterizer {
    surface: Surface,
}

impl Rasterizer {
    pub fn new(surface: Surface) -> Self {
        Self { surface }
    }

    pub fn clear(&mut self) {
        for y in 0..self.surface.height() {
            for x in 0..self.surface.width() {
                *self.surface.get_mut(x, y) = RGBA8::BLACK;
            }
        }
    }
}
