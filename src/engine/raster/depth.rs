
/// Depth buffer for Z-buffering
pub struct DepthBuffer {
    width: usize,
    height: usize,
    buffer: Vec<f32>,
}

impl DepthBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![f32::INFINITY; width * height],
        }
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> f32 {
        debug_assert!(x < self.width && y < self.height);
        unsafe { *self.buffer.get_unchecked(y * self.width + x) }
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, depth: f32) {
        debug_assert!(x < self.width && y < self.height);
        unsafe {
            *self.buffer.get_unchecked_mut(y * self.width + x) = depth;
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(f32::INFINITY);
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.buffer.resize(width * height, f32::INFINITY);
        }
    }
}
