use std::fmt::Debug;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Debug for RGBA8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{:02x?}{:02x?}{:02x?}{:02x?}",
            self.r, self.g, self.b, self.a
        )
    }
}

impl RGBA8 {
    pub const BLACK: Self = RGBA8::new(0, 0, 0, 255);
    pub const TRANSPARENT: Self = RGBA8::new(0, 0, 0, 0);
    pub const WHITE: Self = RGBA8::new(255, 255, 255, 255);
    pub const RED: Self = RGBA8::new(255, 0, 0, 255);
    pub const GREEN: Self = RGBA8::new(0, 255, 0, 255);
    pub const BLUE: Self = RGBA8::new(0, 0, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
