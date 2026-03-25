use glam::Mat4;

pub struct Object3D {
    pub matrix: Mat4,
    // TODO: Add children
}

impl Object3D {
    pub fn new() -> Self {
        Self {
            matrix: Mat4::IDENTITY,
        }
    }
}

impl Default for Object3D {
    fn default() -> Self {
        Self::new()
    }
}
