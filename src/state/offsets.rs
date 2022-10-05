use super::State;

use derivative::Derivative;
use macroquad::prelude::*;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Offsets {
    /// Zoom level.
    #[derivative(Default(value = "32"))]
    pub zoom: i8,
    /// The pan offset relative to the center of the screen.
    pub pan_offset: Vec2,
    /// Grid pan offset relative to the mouse position this frame.
    pub pan_pos: Vec2,
    /// Total grid offsets.
    pub grid_offset: Vec2,
}

impl State {
    /// Update total grid offsets
    pub fn update_grid_offset(&mut self) {
        let middle_offset = (
            (screen_width() - self.tex_size.0 as f32 * self.offsets.zoom as f32) / 2.0,
            (screen_height() - self.tex_size.1 as f32 * self.offsets.zoom as f32) / 2.0,
        );
        self.offsets.grid_offset = vec2(
            middle_offset.0 + self.offsets.pan_offset.x,
            middle_offset.1 + self.offsets.pan_offset.y,
        );
    }

    /// Calculate texture boundary.
    /// Return an array containing minimum and maximum position of the texture.
    pub fn tex_bounds(&self) -> [f32; 4] {
        [
        // X min
        self.offsets.grid_offset.x,
        // X max
        (self.offsets.grid_offset.x + self.tex_size.0 as f32 * self.offsets.zoom as f32) - 1.0,
        // Y min
        self.offsets.grid_offset.y,
        // Y max
        (self.offsets.grid_offset.y + self.tex_size.1 as f32 * self.offsets.zoom as f32) - 1.0,
        ]
    }
}