use crate::engine::raster::Vertex;
use glam::Vec4;

/// Vertex shader trait - transforms vertices from model to clip space
pub trait VertexProgram {
    type Uniform;

    fn main(&self, uniform: &Self::Uniform, vertex: &Vertex, varying: &mut Vertex, position: &mut Vec4);
}

/// Fragment shader trait - computes pixel colors
pub trait FragmentProgram {
    type Uniform;

    fn main(&self, uniform: &Self::Uniform, varying: &Vertex, color: &mut Vec4);
}
