use super::clip::{ClipVertex, clip_triangle_near};
use super::{DepthBuffer, FragmentProgram, Vertex, VertexProgram};
use crate::engine::render::Texture;
use glam::{Vec2, Vec4};

pub struct Raster;

impl Raster {
    /// Render a triangle using programmable vertex and fragment shaders.
    /// Performs near-plane clipping via Sutherland-Hodgman before rasterization.
    #[allow(clippy::too_many_arguments)]
    pub fn triangle<V, F>(
        vertex_program: &V,
        fragment_program: &F,
        depth: &mut DepthBuffer,
        target: &mut Texture,
        uniform: &V::Uniform,
        vertex_0: &Vertex,
        vertex_1: &Vertex,
        vertex_2: &Vertex,
    ) where
        V: VertexProgram,
        F: FragmentProgram<Uniform = V::Uniform>,
    {
        // Execute vertex shader for each vertex
        let mut varying_0 = Vertex::new(vertex_0.len());
        let mut varying_1 = Vertex::new(vertex_1.len());
        let mut varying_2 = Vertex::new(vertex_2.len());
        let mut position_0 = Vec4::ZERO;
        let mut position_1 = Vec4::ZERO;
        let mut position_2 = Vec4::ZERO;

        vertex_program.main(uniform, vertex_0, &mut varying_0, &mut position_0);
        vertex_program.main(uniform, vertex_1, &mut varying_1, &mut position_1);
        vertex_program.main(uniform, vertex_2, &mut varying_2, &mut position_2);

        // Near-plane clip
        let cv0 = ClipVertex { position: position_0, varying: varying_0 };
        let cv1 = ClipVertex { position: position_1, varying: varying_1 };
        let cv2 = ClipVertex { position: position_2, varying: varying_2 };

        let clipped = clip_triangle_near(&cv0, &cv1, &cv2);
        if clipped.len() < 3 {
            return;
        }

        // Rasterize each resulting triangle
        for (i0, i1, i2) in clipped.triangles() {
            let clip_v0 = clipped.get(i0);
            let clip_v1 = clipped.get(i1);
            let clip_v2 = clipped.get(i2);
            Self::rasterize_clipped(
                fragment_program,
                depth,
                target,
                uniform,
                clip_v0,
                clip_v1,
                clip_v2,
            );
        }
    }

    /// Rasterize a single clipped triangle using bounding-box + edge function approach.
    /// This is simpler and more correct than scanline rasterization.
    fn rasterize_clipped<F>(
        fragment_program: &F,
        depth: &mut DepthBuffer,
        target: &mut Texture,
        uniform: &F::Uniform,
        cv0: &ClipVertex,
        cv1: &ClipVertex,
        cv2: &ClipVertex,
    ) where
        F: FragmentProgram,
    {
        let width = target.width();
        let height = target.height();
        let w_f = width as f32;
        let h_f = height as f32;
        let half_w = w_f * 0.5;
        let half_h = h_f * 0.5;

        // Transform to screen space
        let p0 = Vec2::new(
            (cv0.position.x / cv0.position.w) * w_f + half_w,
            (-cv0.position.y / cv0.position.w) * h_f + half_h,
        );
        let p1 = Vec2::new(
            (cv1.position.x / cv1.position.w) * w_f + half_w,
            (-cv1.position.y / cv1.position.w) * h_f + half_h,
        );
        let p2 = Vec2::new(
            (cv2.position.x / cv2.position.w) * w_f + half_w,
            (-cv2.position.y / cv2.position.w) * h_f + half_h,
        );

        // Triangle area (2x, signed)
        let area = Self::edge(p0, p1, p2);
        if area.abs() < 0.001 {
            return; // Degenerate
        }
        let inv_area = 1.0 / area;

        // Perspective-correct varying attributes
        let vary0 = cv0.varying.correct(cv0.position.w);
        let vary1 = cv1.varying.correct(cv1.position.w);
        let vary2 = cv2.varying.correct(cv2.position.w);
        let iz0 = 1.0 / cv0.position.w;
        let iz1 = 1.0 / cv1.position.w;
        let iz2 = 1.0 / cv2.position.w;

        // Bounding box, clamped to screen
        let min_x = p0.x.min(p1.x).min(p2.x).max(0.0) as usize;
        let max_x = (p0.x.max(p1.x).max(p2.x).ceil() as usize).min(width);
        let min_y = p0.y.min(p1.y).min(p2.y).max(0.0) as usize;
        let max_y = (p0.y.max(p1.y).max(p2.y).ceil() as usize).min(height);

        // Rasterize all pixels in bounding box, testing with edge functions
        for y in min_y..max_y {
            let fy = y as f32 + 0.5; // pixel center
            for x in min_x..max_x {
                let fx = x as f32 + 0.5; // pixel center
                let frag = Vec2::new(fx, fy);

                // Barycentric weights via edge functions
                let w0 = Self::edge(p1, p2, frag) * inv_area;
                let w1 = Self::edge(p2, p0, frag) * inv_area;
                let w2 = Self::edge(p0, p1, frag) * inv_area;

                // Inside test — pixel center must be inside triangle
                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }

                // Interpolate depth
                let frag_depth = w0 * iz0 + w1 * iz1 + w2 * iz2;

                // Depth test
                if frag_depth <= depth.get(x, y) {
                    depth.set(x, y, frag_depth);

                    // Interpolate varying (perspective-correct)
                    let frag_varying = Vertex::interpolate(
                        &vary0, &vary1, &vary2,
                        w0, w1, w2,
                        frag_depth,
                    );

                    // Execute fragment shader
                    let mut frag_color = Vec4::ZERO;
                    fragment_program.main(uniform, &frag_varying, &mut frag_color);

                    target.set(x, y, frag_color);
                }
            }
        }
    }

    /// Edge function: positive if point is on the left side of edge v0→v1
    #[inline(always)]
    fn edge(v0: Vec2, v1: Vec2, p: Vec2) -> f32 {
        (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
    }
}
