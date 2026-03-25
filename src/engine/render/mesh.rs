use super::{Geometry, Material};
use glam::Mat4;

pub struct Mesh {
    pub matrix: Mat4,
    pub geometry: Geometry,
    pub material: Material,
    /// Optional morph target for vertex animation
    pub morph_target: Option<Geometry>,
    /// Base geometry (stored for blending back from morph)
    pub base_geometry: Option<Geometry>,
}

impl Mesh {
    pub fn new(geometry: Geometry, material: Material) -> Self {
        Self {
            matrix: Mat4::IDENTITY,
            geometry,
            material,
            morph_target: None,
            base_geometry: None,
        }
    }

    /// Set a morph target. Saves the current geometry as the base for blending.
    pub fn set_morph_target(&mut self, target: Geometry) {
        // Clone current geometry as base
        let base = Geometry {
            vertices: self.geometry.vertices.clone(),
            indices: self.geometry.indices.clone(),
        };
        self.base_geometry = Some(base);
        self.morph_target = Some(target);
    }

    /// Apply morph blend at given weight (0.0 = base, 1.0 = target).
    /// Only does work if a morph target is set.
    pub fn apply_morph(&mut self, weight: f32) {
        if let (Some(base), Some(target)) = (&self.base_geometry, &self.morph_target) {
            self.geometry = base.blend(target, weight);
        }
    }
}
