mod ascii;
mod color;

use anyhow::Result;
use crate::engine::render::Texture;

pub use ascii::AsciiTerminal;
pub use color::ColorTerminal;

/// Terminal rendering trait
pub trait Terminal {
    /// Get terminal dimensions (width, height)
    fn size(&self) -> (usize, usize);
    
    /// Present the color buffer to the terminal
    fn present(&mut self, texture: &Texture) -> Result<()>;
}
