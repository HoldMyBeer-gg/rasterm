use crate::engine::raster::{FragmentProgram, Vertex, VertexProgram};
use crate::engine::render::Texture;
use glam::{Mat4, Vec3, Vec4};
use std::sync::Arc;

/// Shader uniform structure
pub struct BasicUniform {
    pub projection: Mat4,
    pub view: Mat4,
    pub matrix: Mat4,
    /// Normal matrix: transpose(inverse(model_matrix)) for correct normal transforms
    pub normal_matrix: Mat4,
}

/// Uniform for textured rendering. The texture is shared with the material,
/// so building a uniform per draw call only bumps a reference count.
pub struct TexturedUniform {
    pub projection: Mat4,
    pub view: Mat4,
    pub matrix: Mat4,
    pub normal_matrix: Mat4,
    pub texture: Option<Arc<Texture>>,
}

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
    // Clamp so a degenerate (NaN) normal can't poison the colour cast later.
    (ambient + diffuse * ndotl).clamp(0.0, 1.0)
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
    let diffuse = (ambient + 0.7 * ndotl).clamp(0.0, 1.0);
    let specular = (0.6 * spec).clamp(0.0, 1.0);
    (diffuse, specular)
}

/// Basic vertex shader — transforms position to clip space, normal to world space
pub struct BasicVertexShader;

impl VertexProgram for BasicVertexShader {
    type Uniform = BasicUniform;

    fn main(
        &self,
        uniform: &Self::Uniform,
        vertex: &Vertex,
        varying: &mut Vertex,
        position: &mut Vec4,
    ) {
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

    fn main(
        &self,
        uniform: &Self::Uniform,
        vertex: &Vertex,
        varying: &mut Vertex,
        position: &mut Vec4,
    ) {
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
        let tex_color = match uniform.texture.as_deref() {
            Some(texture) if varying.data.len() >= 8 => {
                texture.sample_uv(varying.data[6], varying.data[7])
            }
            _ => Vec4::new(0.7, 0.7, 0.7, 1.0),
        };

        let intensity = if varying.data.len() >= 6 {
            let normal = Vec3::new(varying.data[3], varying.data[4], varying.data[5]);
            diffuse_lighting(normal)
        } else {
            1.0
        };

        *color = (tex_color * intensity)
            .with_w(tex_color.w)
            .clamp(Vec4::ZERO, Vec4::ONE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::render::material::Material;
    use std::sync::Arc;

    fn red_texture() -> Texture {
        let mut texture = Texture::new(1, 1);
        texture.set(0, 0, Vec4::new(1.0, 0.0, 0.0, 1.0));
        texture
    }

    fn uniform_with(texture: Option<Arc<Texture>>) -> TexturedUniform {
        TexturedUniform {
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            matrix: Mat4::IDENTITY,
            normal_matrix: Mat4::IDENTITY,
            texture,
        }
    }

    // position, normal facing the light, uv
    fn lit_fragment() -> Vertex {
        Vertex::from_data(vec![0.0, 0.0, 0.0, 0.48, 0.6, 0.64, 0.5, 0.5])
    }

    #[test]
    fn textured_fragment_samples_its_texture() {
        let uniform = uniform_with(Some(Arc::new(red_texture())));
        let mut color = Vec4::ZERO;
        TexturedFragmentShader.main(&uniform, &lit_fragment(), &mut color);
        assert!(
            color.x > 0.0 && color.y == 0.0 && color.z == 0.0,
            "{color:?}"
        );
    }

    #[test]
    fn textured_fragment_without_texture_falls_back_to_grey() {
        let uniform = uniform_with(None);
        let mut color = Vec4::ZERO;
        TexturedFragmentShader.main(&uniform, &lit_fragment(), &mut color);
        assert!(
            color.x > 0.0 && color.x == color.y && color.y == color.z,
            "{color:?}"
        );
    }

    #[test]
    fn textured_uniform_shares_the_material_texture() {
        let material = Material::textured(red_texture());
        let uniform =
            material.build_textured_uniform(Mat4::IDENTITY, Mat4::IDENTITY, Mat4::IDENTITY);
        assert!(Arc::ptr_eq(
            material.texture.as_ref().unwrap(),
            uniform.texture.as_ref().unwrap()
        ));
    }

    #[test]
    fn textured_uniform_is_send_and_sync() {
        fn check<T: Send + Sync>() {}
        check::<TexturedUniform>();
    }
}
