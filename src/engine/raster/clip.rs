use super::Vertex;
use glam::Vec4;

/// A clip-space vertex with its associated varying data.
#[derive(Clone)]
pub struct ClipVertex {
    pub position: Vec4,
    pub varying: Vertex,
}

/// Clip a triangle against the near plane (z = 0 in clip space, i.e. w + z >= 0 for RH).
///
/// Returns 0, 1, or 2 triangles as a result of clipping.
/// Uses Sutherland-Hodgman algorithm against the near plane only.
///
/// The near plane test in clip space (before perspective divide) is: w + z >= 0
/// A vertex is inside if position.w + position.z >= 0
pub fn clip_triangle_near(
    v0: &ClipVertex,
    v1: &ClipVertex,
    v2: &ClipVertex,
) -> SmallVec {
    let input = [v0.clone(), v1.clone(), v2.clone()];
    let mut output = SmallVec::new();

    clip_against_near_plane(&input, &mut output);

    output
}

/// Compact storage for clipped polygon vertices (max 4 from a single plane clip).
pub struct SmallVec {
    data: [Option<ClipVertex>; 4],
    len: usize,
}

impl SmallVec {
    fn new() -> Self {
        Self {
            data: [None, None, None, None],
            len: 0,
        }
    }

    fn push(&mut self, v: ClipVertex) {
        if self.len < 4 {
            self.data[self.len] = Some(v);
            self.len += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> &ClipVertex {
        self.data[index].as_ref().unwrap()
    }

    /// Return triangulated indices for the polygon (fan from vertex 0).
    /// For 3 verts: [(0,1,2)], for 4 verts: [(0,1,2), (0,2,3)]
    pub fn triangles(&self) -> TriangleIter {
        TriangleIter {
            count: if self.len >= 3 { self.len - 2 } else { 0 },
            current: 0,
        }
    }
}

pub struct TriangleIter {
    count: usize,
    current: usize,
}

impl Iterator for TriangleIter {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.count {
            let tri = (0, self.current + 1, self.current + 2);
            self.current += 1;
            Some(tri)
        } else {
            None
        }
    }
}

/// Test if a vertex is inside the near plane (clip space: w + z >= 0)
#[inline]
fn is_inside_near(v: &ClipVertex) -> bool {
    v.position.w + v.position.z >= 0.0
}

/// Compute the signed distance to the near plane
#[inline]
fn near_distance(v: &ClipVertex) -> f32 {
    v.position.w + v.position.z
}

/// Interpolate between two clip vertices at parameter t
fn lerp_clip_vertex(a: &ClipVertex, b: &ClipVertex, t: f32) -> ClipVertex {
    let position = a.position + (b.position - a.position) * t;

    let len = a.varying.data.len();
    let mut data = Vec::with_capacity(len);
    for i in 0..len {
        data.push(a.varying.data[i] + (b.varying.data[i] - a.varying.data[i]) * t);
    }

    ClipVertex {
        position,
        varying: Vertex::from_data(data),
    }
}

/// Sutherland-Hodgman clip against the near plane
fn clip_against_near_plane(input: &[ClipVertex], output: &mut SmallVec) {
    let n = input.len();
    if n == 0 {
        return;
    }

    for i in 0..n {
        let current = &input[i];
        let next = &input[(i + 1) % n];

        let current_inside = is_inside_near(current);
        let next_inside = is_inside_near(next);

        match (current_inside, next_inside) {
            (true, true) => {
                // Both inside: emit next
                output.push(next.clone());
            }
            (true, false) => {
                // Leaving: emit intersection
                let d_curr = near_distance(current);
                let d_next = near_distance(next);
                let t = d_curr / (d_curr - d_next);
                output.push(lerp_clip_vertex(current, next, t));
            }
            (false, true) => {
                // Entering: emit intersection and next
                let d_curr = near_distance(current);
                let d_next = near_distance(next);
                let t = d_curr / (d_curr - d_next);
                output.push(lerp_clip_vertex(current, next, t));
                output.push(next.clone());
            }
            (false, false) => {
                // Both outside: emit nothing
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_clip_vertex(x: f32, y: f32, z: f32, w: f32) -> ClipVertex {
        ClipVertex {
            position: Vec4::new(x, y, z, w),
            varying: Vertex::from_data(vec![x, y, z, 0.0, 0.0, 1.0, 0.0, 0.0]),
        }
    }

    #[test]
    fn test_all_vertices_inside() {
        // All vertices have w + z >= 0, so no clipping needed
        let v0 = make_clip_vertex(0.0, 0.5, 0.5, 1.0); // w+z = 1.5
        let v1 = make_clip_vertex(-0.5, -0.5, 0.3, 1.0); // w+z = 1.3
        let v2 = make_clip_vertex(0.5, -0.5, 0.3, 1.0); // w+z = 1.3

        let result = clip_triangle_near(&v0, &v1, &v2);
        assert_eq!(result.len(), 3);
        assert_eq!(result.triangles().count(), 1);
    }

    #[test]
    fn test_all_vertices_outside() {
        // All vertices behind near plane: w + z < 0
        let v0 = make_clip_vertex(0.0, 0.5, -2.0, 1.0); // w+z = -1.0
        let v1 = make_clip_vertex(-0.5, -0.5, -2.0, 1.0);
        let v2 = make_clip_vertex(0.5, -0.5, -2.0, 1.0);

        let result = clip_triangle_near(&v0, &v1, &v2);
        assert_eq!(result.len(), 0);
        assert_eq!(result.triangles().count(), 0);
    }

    #[test]
    fn test_one_vertex_outside() {
        // v0 is behind, v1 and v2 are in front
        let v0 = make_clip_vertex(0.0, 0.5, -2.0, 1.0); // w+z = -1.0 (outside)
        let v1 = make_clip_vertex(-0.5, -0.5, 0.5, 1.0); // w+z = 1.5 (inside)
        let v2 = make_clip_vertex(0.5, -0.5, 0.5, 1.0); // w+z = 1.5 (inside)

        let result = clip_triangle_near(&v0, &v1, &v2);
        // One vertex clipped off produces a quad (4 vertices, 2 triangles)
        assert_eq!(result.len(), 4);
        assert_eq!(result.triangles().count(), 2);
    }

    #[test]
    fn test_two_vertices_outside() {
        // v0 is in front, v1 and v2 are behind
        let v0 = make_clip_vertex(0.0, 0.5, 0.5, 1.0); // w+z = 1.5 (inside)
        let v1 = make_clip_vertex(-0.5, -0.5, -2.0, 1.0); // w+z = -1.0 (outside)
        let v2 = make_clip_vertex(0.5, -0.5, -2.0, 1.0); // w+z = -1.0 (outside)

        let result = clip_triangle_near(&v0, &v1, &v2);
        // Two vertices clipped off produces a triangle (3 vertices)
        assert_eq!(result.len(), 3);
        assert_eq!(result.triangles().count(), 1);
    }

    #[test]
    fn test_clipped_positions_on_near_plane() {
        let v0 = make_clip_vertex(0.0, 0.5, -2.0, 1.0); // outside
        let v1 = make_clip_vertex(-0.5, -0.5, 0.5, 1.0); // inside

        // Intersection should have w + z ≈ 0
        let d0 = v0.position.w + v0.position.z; // -1.0
        let d1 = v1.position.w + v1.position.z; // 1.5
        let t = d0 / (d0 - d1); // -1.0 / (-1.0 - 1.5) = -1.0 / -2.5 = 0.4
        let intersection = lerp_clip_vertex(&v0, &v1, t);

        let near_dist = intersection.position.w + intersection.position.z;
        assert!(near_dist.abs() < 1e-5, "intersection should be on near plane, got {}", near_dist);
    }
}
