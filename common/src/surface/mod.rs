pub mod format;

use format::RGBA8;

/**
 * A simple surface representation.
 *
 * RGBA8 format, row-major order.
 */
pub struct Surface {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) buffer: Vec<RGBA8>,
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer = vec![RGBA8::BLACK; width as usize * height as usize];
        Surface {
            width,
            height,
            buffer,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> RGBA8 {
        self.buffer[y as usize * self.width as usize + x as usize]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut RGBA8 {
        &mut self.buffer[y as usize * self.width as usize + x as usize]
    }
}
