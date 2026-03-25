use anyhow::{Context, Result};
use glam::Vec4;
use std::path::Path;

/// RGBA color buffer / texture
pub struct Texture {
    width: usize,
    height: usize,
    data: Vec<f32>, // RGBA interleaved
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height * 4],
        }
    }

    /// Load a texture from an image file (PNG, JPG, BMP, etc.).
    /// The image is converted to RGBA float [0.0, 1.0] format.
    pub fn from_image(path: &Path) -> Result<Self> {
        let img = image::open(path)
            .with_context(|| format!("Failed to load image: {:?}", path))?
            .to_rgba8();

        let width = img.width() as usize;
        let height = img.height() as usize;
        let raw = img.into_raw();

        let mut data = Vec::with_capacity(width * height * 4);
        for pixel in raw.chunks_exact(4) {
            data.push(pixel[0] as f32 / 255.0);
            data.push(pixel[1] as f32 / 255.0);
            data.push(pixel[2] as f32 / 255.0);
            data.push(pixel[3] as f32 / 255.0);
        }

        Ok(Self { width, height, data })
    }

    /// Sample the texture at normalized UV coordinates [0.0, 1.0] with wrapping.
    /// Returns the bilinear-interpolated color.
    pub fn sample_uv(&self, u: f32, v: f32) -> Vec4 {
        if self.width == 0 || self.height == 0 {
            return Vec4::ZERO;
        }

        // Clamp UVs to [0, 1]
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Map to pixel coordinates
        let px = u * (self.width as f32 - 1.0);
        let py = v * (self.height as f32 - 1.0);

        let x0 = px as usize;
        let y0 = py as usize;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let fx = px - x0 as f32;
        let fy = py - y0 as f32;

        // Bilinear interpolation
        let c00 = self.get(x0, y0);
        let c10 = self.get(x1, y0);
        let c01 = self.get(x0, y1);
        let c11 = self.get(x1, y1);

        let top = c00 * (1.0 - fx) + c10 * fx;
        let bottom = c01 * (1.0 - fx) + c11 * fx;
        top * (1.0 - fy) + bottom * fy
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Vec4 {
        debug_assert!(x < self.width && y < self.height);
        let idx = (y * self.width + x) * 4;
        unsafe {
            Vec4::new(
                *self.data.get_unchecked(idx),
                *self.data.get_unchecked(idx + 1),
                *self.data.get_unchecked(idx + 2),
                *self.data.get_unchecked(idx + 3),
            )
        }
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, color: Vec4) {
        debug_assert!(x < self.width && y < self.height);
        let idx = (y * self.width + x) * 4;
        unsafe {
            *self.data.get_unchecked_mut(idx) = color.x;
            *self.data.get_unchecked_mut(idx + 1) = color.y;
            *self.data.get_unchecked_mut(idx + 2) = color.z;
            *self.data.get_unchecked_mut(idx + 3) = color.w;
        }
    }

    /// Fast get without bounds checking (used by terminal rendering)
    #[inline(always)]
    pub fn fast_get(&self, x: usize, y: usize) -> Vec4 {
        let x = x % self.width;
        let y = y % self.height;
        let idx = (y * self.width + x) * 4;
        unsafe {
            Vec4::new(
                *self.data.get_unchecked(idx),
                *self.data.get_unchecked(idx + 1),
                *self.data.get_unchecked(idx + 2),
                *self.data.get_unchecked(idx + 3),
            )
        }
    }

    pub fn clear(&mut self, color: [f32; 4]) {
        for chunk in self.data.chunks_exact_mut(4) {
            chunk.copy_from_slice(&color);
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.data.resize(width * height * 4, 0.0);
        }
    }
}
