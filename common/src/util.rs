use bytes::Buf;

pub trait BufGlamExt {
    fn get_vec3_le(&mut self) -> glam::Vec3;
}

impl<T: Buf> BufGlamExt for T {
    fn get_vec3_le(&mut self) -> glam::Vec3 {
        let x = self.get_f32_le();
        let y = self.get_f32_le();
        let z = self.get_f32_le();
        glam::Vec3 { x, y, z }
    }
}
