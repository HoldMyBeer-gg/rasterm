use crate::engine::raster::{FragmentProgram, Vertex, VertexProgram};
use crate::engine::render::Texture;
use glam::{Mat4, Vec3, Vec4};

/// Shader uniform structure
pub struct BasicUniform {
    pub projection: Mat4,
    pub view: Mat4,
    pub matrix: Mat4,
    /// Normal matrix: transpose(inverse(model_matrix)) for correct normal transforms
    pub normal_matrix: Mat4,
}

/// Uniform with a texture pointer for textured rendering
pub struct TexturedUniform {
    pub projection: Mat4,
    pub view: Mat4,
    pub matrix: Mat4,
    pub normal_matrix: Mat4,
    pub texture: *const Texture,
}

// SAFETY: TexturedUniform is only used within a single render call on one thread.
// The texture pointer is guaranteed valid for the duration of the render.
unsafe impl Send for TexturedUniform {}
unsafe impl Sync for TexturedUniform {}

/// Fixed light direction (normalized) — upper-right-front
const LIGHT_DIR: Vec3 = Vec3::new(0.48, 0.6, 0.64);

/// Compute Lambertian diffuse + ambient lighting from a world-space normal.
/// Two-sided: flips normal toward the light so both sides of thin geometry are lit.
#[inline]
fn diffuse_lighting(world_normal: Vec3) -> f32 {
    let n = world_normal.normalize_or_zero();
    let ndotl = n.dot(LIGHT_DIR).abs(); // two-sided lighting
    let ambient = 0.15;
    let diffuse = 0.85;
    ambient + diffuse * ndotl
}

/// Compute diffuse + specular for metallic materials (two-sided).
#[inline]
fn metallic_lighting(world_normal: Vec3) -> (f32, f32) {
    let n = world_normal.normalize_or_zero();
    // Flip normal toward light for two-sided lighting
    let ndotl_raw = n.dot(LIGHT_DIR);
    let n = if ndotl_raw < 0.0 { -n } else { n };
    let ndotl = ndotl_raw.abs();

    // Half-vector specular (approximating view from +Z)
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let half = (LIGHT_DIR + view_dir).normalize_or_zero();
    let spec = n.dot(half).max(0.0).powf(32.0);

    let ambient = 0.1;
    let diffuse = ambient + 0.7 * ndotl;
    let specular = 0.6 * spec;
    (diffuse, specular)
}

/// Basic vertex shader — transforms position to clip space, normal to world space
pub struct BasicVertexShader;

impl VertexProgram for BasicVertexShader {
    type Uniform = BasicUniform;

    fn main(&self, uniform: &Self::Uniform, vertex: &Vertex, varying: &mut Vertex, position: &mut Vec4) {
        let pos = Vec4::new(vertex.data[0], vertex.data[1], vertex.data[2], 1.0);

        let world_pos = uniform.matrix * pos;
        let view_pos = uniform.view * world_pos;
        *position = uniform.projection * view_pos;

        // Transform normal to world space
        let mut out_data = vertex.data.clone();
        if out_data.len() >= 6 {
            let n = Vec4::new(out_data[3], out_data[4], out_data[5], 0.0);
            let world_n = uniform.normal_matrix * n;
            out_data[3] = world_n.x;
            out_data[4] = world_n.y;
            out_data[5] = world_n.z;
        }
        *varying = Vertex::from_data(out_data);
    }
}

/// Basic fragment shader — Lambertian diffuse with directional light
pub struct BasicFragmentShader;

impl FragmentProgram for BasicFragmentShader {
    type Uniform = BasicUniform;

    fn main(&self, _uniform: &Self::Uniform, varying: &Vertex, color: &mut Vec4) {
        if varying.data.len() >= 6 {
            let normal = Vec3::new(varying.data[3], varying.data[4], varying.data[5]);
            let intensity = diffuse_lighting(normal);
            *color = Vec4::new(intensity, intensity, intensity, 1.0);
        } else {
            *color = Vec4::new(0.5, 0.5, 0.5, 1.0);
        }
    }
}

/// Gold metallic fragment shader — diffuse + specular highlights
pub struct GoldFragmentShader;

impl FragmentProgram for GoldFragmentShader {
    type Uniform = BasicUniform;

    fn main(&self, _uniform: &Self::Uniform, varying: &Vertex, color: &mut Vec4) {
        let gold = Vec3::new(1.0, 0.843, 0.0);

        if varying.data.len() >= 6 {
            let normal = Vec3::new(varying.data[3], varying.data[4], varying.data[5]);
            let (diffuse, specular) = metallic_lighting(normal);

            *color = Vec4::new(
                (gold.x * diffuse + specular).min(1.0),
                (gold.y * diffuse + specular).min(1.0),
                (gold.z * diffuse + specular).min(1.0),
                1.0,
            );
        } else {
            *color = Vec4::new(0.8, 0.67, 0.0, 1.0);
        }
    }
}

/// Vertex shader for textured rendering
pub struct TexturedVertexShader;

impl VertexProgram for TexturedVertexShader {
    type Uniform = TexturedUniform;

    fn main(&self, uniform: &Self::Uniform, vertex: &Vertex, varying: &mut Vertex, position: &mut Vec4) {
        let pos = Vec4::new(vertex.data[0], vertex.data[1], vertex.data[2], 1.0);
        let world_pos = uniform.matrix * pos;
        let view_pos = uniform.view * world_pos;
        *position = uniform.projection * view_pos;

        let mut out_data = vertex.data.clone();
        if out_data.len() >= 6 {
            let n = Vec4::new(out_data[3], out_data[4], out_data[5], 0.0);
            let world_n = uniform.normal_matrix * n;
            out_data[3] = world_n.x;
            out_data[4] = world_n.y;
            out_data[5] = world_n.z;
        }
        *varying = Vertex::from_data(out_data);
    }
}

/// Fragment shader that samples a texture and applies directional lighting
pub struct TexturedFragmentShader;

impl FragmentProgram for TexturedFragmentShader {
    type Uniform = TexturedUniform;

    fn main(&self, uniform: &Self::Uniform, varying: &Vertex, color: &mut Vec4) {
        let tex_color = if !uniform.texture.is_null() && varying.data.len() >= 8 {
            let u = varying.data[6];
            let v = varying.data[7];
            // SAFETY: texture pointer is valid for the duration of the render call
            let texture = unsafe { &*uniform.texture };
            texture.sample_uv(u, v)
        } else {
            Vec4::new(0.7, 0.7, 0.7, 1.0)
        };

        let intensity = if varying.data.len() >= 6 {
            let normal = Vec3::new(varying.data[3], varying.data[4], varying.data[5]);
            diffuse_lighting(normal)
        } else {
            1.0
        };

        *color = Vec4::new(
            tex_color.x * intensity,
            tex_color.y * intensity,
            tex_color.z * intensity,
            tex_color.w,
        );
    }
}
