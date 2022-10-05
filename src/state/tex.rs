use super::State;
use macroquad::prelude::*;

impl State {
    pub fn tex_paint_single(&mut self, x: i32, y: i32, color: Color) {
        // Ignore if out of bounds
        if x < 0 || y < 0 || x as u32 >= self.tex_size.0 || y as u32 >= self.tex_size.1 {
            return;
        }
        self.tex.set_pixel(x as u32, y as u32, color);
    }

    pub fn tex_paint_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        for x in x..x + w {
            for y in y..y + h {
                self.tex_paint_single(x, y, color);
            }
        }
    }

    pub fn tex_paint_square(&mut self, x: i32, y: i32, size: i32, color: Color) {
        if size == 1 {
            self.tex_paint_single(x, y, color);
        } else {
            // Offset so that the center of the square is at the given coordinates
            if size % 2 == 0 {
                self.tex_paint_rect(x - size / 2, y - size / 2, size, size, color);
            } else {
                self.tex_paint_rect(x - size / 2, y - size / 2, size + 1, size + 1, color);
            }
        }
    }

    pub fn tex_paint_circle(&mut self, _x: i32, _y: i32, _r: i32, _color: Color) {
        // TODO: I can not figure it out :(
    }

    pub fn tex_render(&mut self) {
        // Transparent background
        draw_rectangle(
            self.offsets.grid_offset.x,
            self.offsets.grid_offset.y,
            self.tex_size.0 as f32 * self.offsets.zoom as f32,
            self.tex_size.1 as f32 * self.offsets.zoom as f32,
            Color::from_rgba(128, 128, 128, 255),
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
            4.0,
            Color::from_rgba(237, 237, 237, 255),
        );
    }
}
