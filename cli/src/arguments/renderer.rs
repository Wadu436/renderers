#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum Renderer {
    Rasterizer,
    RayTracer,
}
