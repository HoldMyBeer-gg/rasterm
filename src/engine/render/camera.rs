use glam::Mat4;

pub struct Camera {
    pub projection: Mat4,
    pub view: Mat4,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
