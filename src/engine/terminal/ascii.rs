use super::Terminal;
use crate::engine::render::Texture;
use anyhow::Result;
use crossterm::{cursor, execute, terminal};
use std::io::{self, Write};

pub struct AsciiTerminal {
    width: usize,
    height: usize,
    gradient: Vec<char>,
}

impl AsciiTerminal {
    pub fn new() -> Result<Self> {
        let (cols, rows) = terminal::size()?;
        Ok(Self {
            width: cols as usize,
            height: rows as usize,
            gradient: " .:;+=xX$#&".chars().collect(),
        })
    }

    fn compute_gradient_index(&self, r: f32, g: f32, b: f32) -> usize {
        let average = (r + g + b) / 3.0;
        let shade = average.clamp(0.0, 0.999);
        (shade * (self.gradient.len() - 1) as f32) as usize
    }
}

impl Terminal for AsciiTerminal {
    fn size(&self) -> (usize, usize) {
        if let Ok((cols, rows)) = terminal::size() {
            (cols as usize, rows as usize)
        } else {
            (self.width, self.height)
        }
    }

    fn present(&mut self, texture: &Texture) -> Result<()> {
        let (width, height) = self.size();

        #[cfg(feature = "parallel")]
        let output = self.present_parallel(texture, width, height);

        #[cfg(not(feature = "parallel"))]
        let output = self.present_sequential(texture, width, height);

        let mut stdout = io::stdout();
        execute!(stdout, cursor::MoveTo(0, 0))?;
        write!(stdout, "{}", output)?;
        stdout.flush()?;

        Ok(())
    }
}

impl AsciiTerminal {
    #[cfg(feature = "parallel")]
    fn present_parallel(&self, texture: &Texture, width: usize, height: usize) -> String {
        use rayon::prelude::*;

        let rows: Vec<String> = (0..height)
            .into_par_iter()
            .map(|y| {
                let mut row = String::with_capacity(width);
                for x in 0..width {
                    let color = texture.fast_get(x % texture.width(), y % texture.height());
                    let index = self.compute_gradient_index(color.x, color.y, color.z);
                    row.push(self.gradient[index]);
                }
                row
            })
            .collect();

        rows.join("")
    }

    #[allow(dead_code)]
    fn present_sequential(&self, texture: &Texture, width: usize, height: usize) -> String {
        let mut output = String::with_capacity(width * height + 10);

        for y in 0..height {
            for x in 0..width {
                let color = texture.fast_get(x % texture.width(), y % texture.height());
                let index = self.compute_gradient_index(color.x, color.y, color.z);
                output.push(self.gradient[index]);
            }
        }

        output
    }
}
