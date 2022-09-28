#[derive(Clone)]
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

    // Set the cell color at the given coordinates.
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

    /// Clear all cells (set all cells to transparent)
    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = [0, 0, 0, 0];
        }
    }

    /// Replace with another grid
    pub fn replace(&mut self, grid: &Grid) {
        self.width = grid.width;
        self.height = grid.height;
        self.cells = grid.cells.clone();
    }
}
