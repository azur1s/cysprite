use std::fs;

use serde::{ Serialize, Deserialize };
use ron::ser::{ to_string };

#[derive(Clone, Deserialize, Serialize)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<[u8; 4]>,
}

#[macro_export]
macro_rules! compact {
    ($c: expr) => { Color::from_rgba($c[0], $c[1], $c[2], $c[3]) };
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![[0, 0, 0, 0]; width * height],
        }
    }

    /// Get the cell color at the given coordinates.
    pub fn get(&self, x: usize, y: usize) -> [u8; 4] {
        self.cells[y * self.width + x]
    }

    /// Set the cell color at the given coordinates.
    pub fn set(&mut self, x: usize, y: usize, color: [u8; 4]) {
        // If the coordinates is out of bounds or the color is transparent, do nothing
        if x >= self.width || y >= self.height || color[3] == 0 {
            return;
        }
        // If the color is full solid, then just simply set the color
        if color[3] == 255 {
            self.cells[y * self.width + x] = color;
        } else {
            let old_color = self.get(x, y);
            let new_color = [
                (old_color[0] as u16 * (255 - color[3]) as u16
                    + color[0] as u16 * color[3] as u16)
                    / 255,
                (old_color[1] as u16 * (255 - color[3]) as u16
                    + color[1] as u16 * color[3] as u16)
                    / 255,
                (old_color[2] as u16 * (255 - color[3]) as u16
                    + color[2] as u16 * color[3] as u16)
                    / 255,
                (old_color[3].saturating_add(color[3])) as u16,
            ].map(|x| x as u8);
            self.cells[y * self.width + x] = new_color;
        }
    }

    /// Erase the cell at the given coordinates.
    pub fn erase(&mut self, x: usize, y: usize) {
        self.cells[y * self.width + x] = [0, 0, 0, 0];
    }

    /// Clear all cells (set all cells to transparent).
    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = [0, 0, 0, 0];
        }
    }

    /// Replace with another grid.
    pub fn replace(&mut self, grid: &Grid) {
        self.width = grid.width;
        self.height = grid.height;
        self.cells = grid.cells.clone();
    }

    /// Save file as RON format.
    pub fn save_as_ron(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let ron = to_string(self)?;
        fs::write(&path, ron)?;
        Ok(())
    }

    /// Load RON file into a grid.
    pub fn load_ron(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let ron = fs::read_to_string(path)?;
        let grid: Grid = ron::from_str(&ron)?;
        Ok(grid)
    }

    /// Save file as PNG format.
    pub fn save_as_png(&self, path: &str) -> Result<(), png::EncodingError> {
        let ref mut w = std::io::BufWriter::new(fs::File::create(path)?);
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);

        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        let data: Vec<u8> = self.cells.iter().flat_map(|c| c.iter()).cloned().collect();
        writer.write_image_data(&data)
    }
}

