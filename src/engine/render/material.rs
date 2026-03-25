use crate::engine::render::shaders::{
    BasicFragmentShader, BasicUniform, BasicVertexShader, GoldFragmentShader,
    TexturedFragmentShader, TexturedUniform, TexturedVertexShader,
};
use crate::engine::render::Texture;
use glam::Mat4;
use std::path::Path;

pub enum ShaderType {
    Basic,
    Gold,
    Textured,
}

pub struct Material {
    pub vertex_program: BasicVertexShader,
    pub fragment_program_basic: BasicFragmentShader,
    pub fragment_program_gold: GoldFragmentShader,
    pub textured_vertex_program: TexturedVertexShader,
    pub textured_fragment_program: TexturedFragmentShader,
    pub shader_type: ShaderType,
    pub texture: Option<Texture>,
}

impl Material {
    pub fn new() -> Self {
        Self {
            vertex_program: BasicVertexShader,
            fragment_program_basic: BasicFragmentShader,
            fragment_program_gold: GoldFragmentShader,
            textured_vertex_program: TexturedVertexShader,
            textured_fragment_program: TexturedFragmentShader,
            shader_type: ShaderType::Basic,
            texture: None,
        }
    }

    pub fn gold() -> Self {
        Self {
            shader_type: ShaderType::Gold,
            ..Self::new()
        }
    }

    pub fn textured(texture: Texture) -> Self {
        Self {
            shader_type: ShaderType::Textured,
            texture: Some(texture),
            ..Self::new()
        }
    }

    pub fn from_image(path: &Path) -> anyhow::Result<Self> {
        let texture = Texture::from_image(path)?;
        Ok(Self::textured(texture))
    }

    /// Compute the normal matrix: transpose of the inverse of the model matrix.
    /// This correctly transforms normals even when the model matrix has non-uniform scale.
    fn normal_matrix(model: Mat4) -> Mat4 {
        model.inverse().transpose()
    }

    pub fn build_uniform(&self, projection: Mat4, view: Mat4, matrix: Mat4) -> BasicUniform {
        BasicUniform {
            projection,
            view,
            normal_matrix: Self::normal_matrix(matrix),
            matrix,
        }
    }

    pub fn build_textured_uniform(&self, projection: Mat4, view: Mat4, matrix: Mat4) -> TexturedUniform {
        TexturedUniform {
            projection,
            view,
            normal_matrix: Self::normal_matrix(matrix),
            matrix,
            texture: self.texture.as_ref().map_or(std::ptr::null(), |t| t as *const Texture),
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}
