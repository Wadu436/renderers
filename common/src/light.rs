use glam::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum Light {
    Sun { direction: Vec3, intensity: f32 },
}
