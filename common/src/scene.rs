use crate::{camera::Camera, model::triangle::Mesh};

#[derive(Default)]
pub struct SceneBuilder {
    camera: Option<Camera>,
    meshes: Vec<Mesh>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn add_mesh(mut self, mesh: Mesh) -> Self {
        self.meshes.push(mesh);
        self
    }

    pub fn add_meshes(mut self, meshes: Vec<Mesh>) -> Self {
        self.meshes.extend(meshes);
        self
    }

    pub fn build(self) -> Scene {
        // We can do things like building acceleration structures here later
        Scene {
            camera: self.camera.unwrap_or_default(),
            meshes: self.meshes,
        }
    }
}

#[derive(Clone)]
pub struct Scene {
    camera: Camera,
    meshes: Vec<Mesh>,
}

impl Scene {
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn meshes(&self) -> &Vec<Mesh> {
        &self.meshes
    }
}
