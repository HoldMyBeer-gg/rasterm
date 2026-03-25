use super::{Camera, Mesh, Scene, Texture};
use crate::engine::raster::{DepthBuffer, Raster};
use glam::Mat4;

pub struct Renderer {
    pub depth_buffer: DepthBuffer,
    pub color_buffer: Texture,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new() -> Self {
        let width = 8;
        let height = 8;
        Self {
            depth_buffer: DepthBuffer::new(width, height),
            color_buffer: Texture::new(width, height),
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.depth_buffer.resize(width, height);
            self.color_buffer.resize(width, height);
        }
    }

    pub fn clear(&mut self, color: [f32; 4]) {
        self.color_buffer.clear(color);
        self.depth_buffer.clear();
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        self.render_scene(camera, scene, &scene.matrix);
    }

    fn render_scene(&mut self, camera: &Camera, scene: &Scene, transform: &Mat4) {
        for mesh in &scene.meshes {
            let matrix = *transform * mesh.matrix;
            self.render_mesh(camera, mesh, &matrix);
        }
    }

    fn render_mesh(&mut self, camera: &Camera, mesh: &Mesh, transform: &Mat4) {
        use crate::engine::render::material::ShaderType;

        let geometry = &mesh.geometry;
        let material = &mesh.material;

        match material.shader_type {
            ShaderType::Basic => {
                let vertex_program = &material.vertex_program;
                let fragment_program = &material.fragment_program_basic;
                let uniform = material.build_uniform(camera.projection, camera.view, *transform);
                self.rasterize_triangles(vertex_program, fragment_program, &uniform, geometry);
            }
            ShaderType::Gold => {
                let vertex_program = &material.vertex_program;
                let fragment_program = &material.fragment_program_gold;
                let uniform = material.build_uniform(camera.projection, camera.view, *transform);
                self.rasterize_triangles(vertex_program, fragment_program, &uniform, geometry);
            }
            ShaderType::Textured => {
                let vertex_program = &material.textured_vertex_program;
                let fragment_program = &material.textured_fragment_program;
                let uniform = material.build_textured_uniform(camera.projection, camera.view, *transform);
                self.rasterize_triangles(vertex_program, fragment_program, &uniform, geometry);
            }
        }
    }

    fn rasterize_triangles<V, F>(
        &mut self,
        vertex_program: &V,
        fragment_program: &F,
        uniform: &V::Uniform,
        geometry: &super::Geometry,
    ) where
        V: crate::engine::raster::VertexProgram,
        F: crate::engine::raster::FragmentProgram<Uniform = V::Uniform>,
    {
        for i in (0..geometry.indices.len()).step_by(3) {
            let v0 = &geometry.vertices[geometry.indices[i]];
            let v1 = &geometry.vertices[geometry.indices[i + 1]];
            let v2 = &geometry.vertices[geometry.indices[i + 2]];

            Raster::triangle(
                vertex_program,
                fragment_program,
                &mut self.depth_buffer,
                &mut self.color_buffer,
                uniform,
                v0,
                v1,
                v2,
            );
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
