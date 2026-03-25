use super::Mesh;
use glam::Mat4;

/// Maximum number of meshes allowed in a scene
pub const MAX_MESHES: usize = 1_000;

pub struct Scene {
    pub matrix: Mat4,
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            matrix: Mat4::IDENTITY,
            meshes: Vec::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> bool {
        if self.meshes.len() >= MAX_MESHES {
            return false;
        }
        self.meshes.push(mesh);
        true
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
