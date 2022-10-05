use super::State;
use macroquad::prelude::*;

impl State {
    pub fn tex_paint_single(&mut self, x: i32, y: i32, color: Color) {
        // Check if out of bounds (shouldn't happen, but just in case so it doesn't panic)
        if x < 0 || y < 0 || x as u32 >= self.tex_size.0 || y as u32 >= self.tex_size.1 {
            return;
        }
        self.tex.set_pixel(
            x as u32,
            y as u32,
            color,
        );
    }

    pub fn tex_render(&mut self) {
        // Transparent checkerboard background
        draw_texture_ex(
            self.transparent_bg,
            self.offsets.grid_offset.x,
            self.offsets.grid_offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    self.tex_size.0 as f32 * self.offsets.zoom as f32,
                    self.tex_size.1 as f32 * self.offsets.zoom as f32,
                )),
                ..Default::default()
            },
        );

        // The main texture
        let t = Texture2D::from_image(&self.tex);
        t.set_filter(FilterMode::Nearest);
        draw_texture_ex(
            t,
            self.offsets.grid_offset.x,
            self.offsets.grid_offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    self.tex_size.0 as f32 * self.offsets.zoom as f32,
                    self.tex_size.1 as f32 * self.offsets.zoom as f32,
                )),
                ..Default::default()
            },
        );

        draw_rectangle_lines(
            self.offsets.grid_offset.x,
            self.offsets.grid_offset.y,
            self.tex_size.0 as f32 * self.offsets.zoom as f32,
            self.tex_size.1 as f32 * self.offsets.zoom as f32,
            1.0,
            WHITE,
        );
    }
}
