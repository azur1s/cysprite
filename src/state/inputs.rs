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
    /// Handle all inputs
    pub fn input(&mut self) {
        // ----- Mouse section -----

        // Painting
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();

            let bounds = self.tex_bounds();
            if !(x < bounds[0] || x > bounds[1] || y < bounds[2] || y > bounds[3]) {
                let x = (x - self.offsets.grid_offset.x) as i32 / self.offsets.zoom as i32;
                let y = (y - self.offsets.grid_offset.y) as i32 / self.offsets.zoom as i32;

                if !self.inputs.painted_cells.contains(&(x as u32, y as u32)) {
                    self.tex_paint_single(x, y, self.color);
                    self.inputs.painted_cells.push((x as u32, y as u32));
                }
            }
        }

        // Panning
        if is_mouse_button_pressed(MouseButton::Right) {
            let (x, y) = mouse_position();
            self.offsets.pan_pos = vec2(x, y);
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (x, y) = mouse_position();

            self.offsets.pan_offset.x += (x - self.offsets.pan_pos.x) * 0.5;
            self.offsets.pan_offset.y += (y - self.offsets.pan_pos.y) * 0.5;

            self.offsets.pan_pos = vec2(x, y);
        }

        // Zooming
        self.offsets.zoom += mouse_wheel().1 as i8;
        self.offsets.zoom = self.offsets.zoom.clamp(4, 64);

        // ----- Keyboard section -----

        // Reset zoom and pan
        if is_key_pressed(KeyCode::F) {
            self.offsets.zoom = 32;
            self.offsets.pan_offset = vec2(0.0, 0.0);
        }
    }
}