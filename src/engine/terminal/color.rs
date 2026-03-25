use super::Terminal;
use crate::engine::render::Texture;
use anyhow::Result;
use crossterm::{cursor, execute, terminal};
use std::io::{self, Write};

pub struct ColorTerminal {
    width: usize,
    height: usize,
    numeric_cache: Vec<String>,
}

impl ColorTerminal {
    pub fn new() -> Result<Self> {
        let (cols, rows) = terminal::size()?;
        let numeric_cache: Vec<String> = (0..=255).map(|i| i.to_string()).collect();

        Ok(Self {
            width: cols as usize,
            height: rows as usize,
            numeric_cache,
        })
    }

    fn write_pixel(&self, buffer: &mut String, color_bg: (u8, u8, u8), color_fg: (u8, u8, u8)) {
        buffer.push_str("\x1b[48;2;");
        buffer.push_str(&self.numeric_cache[color_bg.0 as usize]);
        buffer.push(';');
        buffer.push_str(&self.numeric_cache[color_bg.1 as usize]);
        buffer.push(';');
        buffer.push_str(&self.numeric_cache[color_bg.2 as usize]);
        buffer.push('m');

        buffer.push_str("\x1b[38;2;");
        buffer.push_str(&self.numeric_cache[color_fg.0 as usize]);
        buffer.push(';');
        buffer.push_str(&self.numeric_cache[color_fg.1 as usize]);
        buffer.push(';');
        buffer.push_str(&self.numeric_cache[color_fg.2 as usize]);
        buffer.push('m');

        buffer.push('\u{2584}'); // ▄
    }
}

impl Terminal for ColorTerminal {
    fn size(&self) -> (usize, usize) {
        if let Ok((cols, rows)) = terminal::size() {
            (cols as usize, rows as usize * 2)
        } else {
            (self.width, self.height * 2)
        }
    }

    fn present(&mut self, texture: &Texture) -> Result<()> {
        let (width, height_doubled) = self.size();
        let height = height_doubled / 2;

        #[cfg(feature = "parallel")]
        let output = self.present_parallel(texture, width, height);

        #[cfg(not(feature = "parallel"))]
        let output = self.present_sequential(texture, width, height);

        let mut stdout = io::stdout();
        execute!(stdout, cursor::MoveTo(0, 0))?;
        write!(stdout, "{}\x1b[0m", output)?;
        stdout.flush()?;

        Ok(())
    }
}

impl ColorTerminal {
    #[cfg(feature = "parallel")]
    fn present_parallel(&self, texture: &Texture, width: usize, height: usize) -> String {
        use rayon::prelude::*;

        let rows: Vec<String> = (0..height)
            .into_par_iter()
            .map(|y| {
                let y_top = y * 2;
                let y_bottom = y * 2 + 1;
                let mut row_buf = String::with_capacity(width * 25);

                for x in 0..width {
                    let color_top = texture.fast_get(x % texture.width(), y_top % texture.height());
                    let color_bottom = texture.fast_get(x % texture.width(), y_bottom % texture.height());

                    let bg = (
                        (color_top.x * 255.0) as u8,
                        (color_top.y * 255.0) as u8,
                        (color_top.z * 255.0) as u8,
                    );
                    let fg = (
                        (color_bottom.x * 255.0) as u8,
                        (color_bottom.y * 255.0) as u8,
                        (color_bottom.z * 255.0) as u8,
                    );

                    self.write_pixel(&mut row_buf, bg, fg);
                }

                row_buf
            })
            .collect();

        rows.join("")
    }

    #[allow(dead_code)]
    fn present_sequential(&self, texture: &Texture, width: usize, height: usize) -> String {
        let mut output = String::with_capacity(width * height * 25);

        for y in 0..height {
            let y_top = y * 2;
            let y_bottom = y * 2 + 1;

            for x in 0..width {
                let color_top = texture.fast_get(x % texture.width(), y_top % texture.height());
                let color_bottom = texture.fast_get(x % texture.width(), y_bottom % texture.height());

                let bg = (
                    (color_top.x * 255.0) as u8,
                    (color_top.y * 255.0) as u8,
                    (color_top.z * 255.0) as u8,
                );
                let fg = (
                    (color_bottom.x * 255.0) as u8,
                    (color_bottom.y * 255.0) as u8,
                    (color_bottom.z * 255.0) as u8,
                );

                self.write_pixel(&mut output, bg, fg);
            }
        }

        output
    }
}
