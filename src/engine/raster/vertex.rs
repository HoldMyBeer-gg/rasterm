/// Vertex data structure with arbitrary attributes
#[derive(Clone, Debug)]
pub struct Vertex {
    /// Vertex attributes (position, normal, uv, etc.)
    pub data: Vec<f32>,
}

impl Vertex {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
        }
    }

    pub fn from_data(data: Vec<f32>) -> Self {
        Self { data }
    }

    /// Perspective-correct varying interpolation
    #[inline]
    pub fn correct(&self, w: f32) -> Self {
        Self {
            data: self.data.iter().map(|&v| v / w).collect(),
        }
    }

    /// Interpolate three vertices with barycentric weights
    #[inline]
    pub fn interpolate(
        v0: &Self,
        v1: &Self,
        v2: &Self,
        w0: f32,
        w1: f32,
        w2: f32,
        depth: f32,
    ) -> Self {
        let len = v0.data.len();
        let mut result = Self::new(len);
        
        for i in 0..len {
            result.data[i] = (v0.data[i] * w0 + v1.data[i] * w1 + v2.data[i] * w2) * depth;
        }
        
        result
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
