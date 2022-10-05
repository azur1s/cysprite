use super::State;

use derivative::Derivative;
use macroquad::prelude::*;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Inputs {
    /// A vector containing all cells that have
    /// been modified in this frame.
    pub painted_cells: Vec<(u32, u32)>,
}

impl State {
    /// Handle all mouse inputs
    pub fn input_mouse(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();

            let bounds = self.tex_bounds();
            if !(x < bounds[0] || x > bounds[1] || y < bounds[2] || y > bounds[3]) {
                let x = (x - self.offsets.grid_offset.x) as i32 / self.offsets.zoom as i32;
                let y = (y - self.offsets.grid_offset.y) as i32 / self.offsets.zoom as i32;

                if !self.inputs.painted_cells.contains(&(x as u32, y as u32)) {
                    self.tex_paint_single(x, y, self.color);
                    self.inputs.painted_cells.push((x as u32, y as u32));
                    println!("Painted cell: {}, {}", x, y);
                }
            }
        }
    }
}