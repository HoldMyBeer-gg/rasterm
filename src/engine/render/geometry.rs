use crate::engine::raster::Vertex;
use anyhow::{bail, Result};

/// Maximum number of vertices allowed per geometry to prevent unbounded allocation
pub const MAX_VERTICES: usize = 100_000;

/// Maximum number of triangle indices allowed per geometry
pub const MAX_INDICES: usize = 300_000;

pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Create a Geometry with validation of resource limits.
    /// Returns an error if vertex or index counts exceed the configured maximums.
    pub fn with_limits(vertices: Vec<Vertex>, indices: Vec<usize>) -> Result<Self> {
        if vertices.len() > MAX_VERTICES {
            bail!(
                "Geometry exceeds vertex limit: {} vertices (max {})",
                vertices.len(),
                MAX_VERTICES
            );
        }
        if indices.len() > MAX_INDICES {
            bail!(
                "Geometry exceeds index limit: {} indices (max {})",
                indices.len(),
                MAX_INDICES
            );
        }
        // Validate all indices are in bounds
        for (i, &idx) in indices.iter().enumerate() {
            if idx >= vertices.len() {
                bail!(
                    "Index out of bounds at position {}: index {} but only {} vertices",
                    i,
                    idx,
                    vertices.len()
                );
            }
        }
        Ok(Self { vertices, indices })
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Blend between this geometry and a morph target at the given weight (0.0 = self, 1.0 = target).
    /// Both geometries must have the same vertex count and vertex data layout.
    /// Returns a new Geometry with interpolated vertex positions and normals.
    pub fn blend(&self, target: &Geometry, weight: f32) -> Geometry {
        let w = weight.clamp(0.0, 1.0);
        let inv_w = 1.0 - w;

        let vertices = self.vertices.iter().zip(target.vertices.iter())
            .map(|(a, b)| {
                let len = a.data.len().min(b.data.len());
                let mut data = Vec::with_capacity(len);
                for i in 0..len {
                    data.push(a.data[i] * inv_w + b.data[i] * w);
                }
                Vertex::from_data(data)
            })
            .collect();

        Geometry {
            vertices,
            indices: self.indices.clone(),
        }
    }
}

impl Default for Geometry {
    fn default() -> Self {
        Self::new()
    }
}
