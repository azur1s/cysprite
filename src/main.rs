use macroquad::prelude::*;

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<[u8; 4]>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![[0, 0, 0, 0]; width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> [u8; 4] {
        self.cells[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, color: [u8; 4]) {
        // If the coordinates is out of bounds or the color is transparent, do nothing
        if x >= self.width || y >= self.height || color[3] == 0 {
            return;
        }
        // If the color is full solid, then just simply set the color
        if color[3] == 255 {
            self.cells[y * self.width + x] = color;
            return;
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
                255,
            ].map(|x| x as u8);
            self.cells[y * self.width + x] = new_color;
        }
    }

    fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = [0, 0, 0, 0];
        }
    }
}

fn hex_to_rgba(s: &String) -> Option<[u8; 4]> {
    let s = s.trim_start_matches('#');

    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some([r, g, b, 255])
        },
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            let a = u8::from_str_radix(&s[6..8], 16).ok()?;
            Some([r, g, b, a])
        },
        _ => None,
    }
}

fn rgba_to_hex(color: [u8; 4]) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}", color[0], color[1], color[2], color[3])
}

#[macroquad::main("harcana - pixel art tool")]
async fn main() {
    let mut grid = Grid::new(16, 16);

    // A vector containing the coordinates of the pixels that have been modified at this frame
    // So you can't apply transparent color to a pixel over and over again
    let mut painted_this_frame: Vec<(usize, usize)> = vec![];

    // How many pixel per cell (could be interpreted as zoom level)
    let mut grid_cell_size = 24;

    let mut primary_color: [u8; 4] = [255, 255, 255, 255];
    let mut secondary_color: [u8; 4] = [255, 255, 255, 255];
    let mut primary_color_input = String::new();
    let mut secondary_color_input = String::new();

    // A bool for whether we are interacting with the GUI
    // so the mouse doesn't draw
    let mut on_gui = false;

    // Pan offset & initial position for panning
    let (mut x_pan_offset, mut y_pan_offset) = (0.0, 0.0);
    let (mut x_pan_pos, mut y_pan_pos) = (0.0, 0.0);

    loop {
        // Offset
        let x_center_offset = (screen_width() - grid.width as f32 * grid_cell_size as f32) / 2.0;
        let y_center_offset = (screen_height() - grid.height as f32 * grid_cell_size as f32) / 2.0;

        let x_offset = x_center_offset + x_pan_offset;
        let y_offset = y_center_offset + y_pan_offset;

        // -------------------- [ UI ] --------------------

        clear_background(Color::from_rgba(10, 12, 14, 255));
        let grid_color = Color::from_rgba(68, 81, 94, 255);

        // Drawing grid lines
        for x in 0..=grid.width {
            draw_line(
                // from
                x_offset + x as f32 * grid_cell_size as f32,
                y_offset,
                // to
                x_offset + x as f32 * grid_cell_size as f32,
                y_offset + grid.height as f32 * grid_cell_size as f32,
                1.0, // width
                grid_color
            ); 
        }

        for y in 0..=grid.height {
            draw_line(
                // from
                x_offset,
                y_offset + y as f32 * grid_cell_size as f32,
                // to
                x_offset + grid.width as f32 * grid_cell_size as f32,
                y_offset + y as f32 * grid_cell_size as f32,
                1.0, // width
                grid_color
            ); 
        }

        // Rendering grid cells
        for x in 0..grid.width {
            for y in 0..grid.height {
                let cell_color = grid.get(x, y);
                if cell_color[3] != 0 {
                    draw_rectangle(
                        x_offset + x as f32 * grid_cell_size as f32,
                        y_offset + y as f32 * grid_cell_size as f32,
                        grid_cell_size as f32,
                        grid_cell_size as f32,
                        Color::from_rgba(cell_color[0], cell_color[1], cell_color[2], cell_color[3]),
                    );
                }
            }
        }

        // Process UI
        egui_macroquad::ui(|ctx| {
            // Styling
            use egui::{
                FontFamily::Proportional,
                FontId
            };
            use egui::TextStyle::*;

            let mut style = (*ctx.style()).clone();
            style.text_styles = [
                (Heading, FontId::new(24.0, Proportional)),
                (Body, FontId::new(20.0, Proportional)),
                (Monospace, FontId::new(20.0, Proportional)),
                (Button, FontId::new(20.0, Proportional)),
                (Small, FontId::new(20.0, Proportional)),
            ].into();
            ctx.set_style(style);

            // Panels
            let grid_actions = egui::Window::new("Grid Actions").show(ctx, |ui| {
                ui.label("Clear grid");
                if ui.button("Clear").clicked() {
                    grid.clear();
                }

                ui.label("Zoom");
                ui.add(egui::Slider::new(&mut grid_cell_size, 4..=64).text("Zoom"));
            });

            let colors = egui::Window::new("Colors").show(ctx, |ui| {
                ui.label("Colors");

                // Macros for adding color inputs
                macro_rules! color_input {
                    ($ui: ident, $color: ident, $color_input: ident) => {
                        let mut picked_color = $color.clone();
                        // Color wheel
                        $ui.color_edit_button_srgba_unmultiplied(&mut picked_color);
                        // Hex input
                        $ui.add(egui::TextEdit::singleline(&mut $color_input)
                                .hint_text("#rrggbbaa")
                                .desired_width(100.0));

                        // If the color has changed by the color picker
                        if picked_color != $color {
                            $color = picked_color;
                            // Replace the hex input with the new color
                            $color_input = rgba_to_hex($color);
                        } else {
                            if let Some(color) = hex_to_rgba(&$color_input) {
                                $color = color;
                            }
                        }
                    }
                }

                ui.horizontal(|ui| {
                    color_input!(ui, primary_color, primary_color_input);
                });
                ui.horizontal(|ui| {
                    color_input!(ui, secondary_color, secondary_color_input);
                });
            });

            // Check if the GUI is using pointer
            // so that it blocks the mouse from drawing
            [
                colors,
                grid_actions
            ].iter().for_each(|panel| {
                if let Some(panel) = panel {
                    if panel.response.ctx.is_using_pointer() {
                        on_gui = true;
                    } else {
                        on_gui = false;
                    }
                }
            });
        });

        // -------------------- [ Inputs ] --------------------

        // Check if not interacting with GUI
        if !on_gui {
            // Mouse handling
            if is_mouse_button_down(MouseButton::Left)
            || is_mouse_button_down(MouseButton::Right) {
                let (x, y) = mouse_position();

                // Bail out if mouse is outside of grid
                if !(x > x_offset + grid.width as f32 * grid_cell_size as f32
                    || x < x_offset
                    || y > y_offset + grid.height as f32 * grid_cell_size as f32
                    || y < y_offset) {
                    // Align center and convert to grid coordinates
                    let x = ((x - x_offset) / grid_cell_size as f32) as usize;
                    let y = ((y - y_offset) / grid_cell_size as f32) as usize;

                    // Don't draw if the cell have already been painted since mouse down
                    if !painted_this_frame.contains(&(x, y)) {
                        if is_mouse_button_down(MouseButton::Left) {
                            grid.set(x, y, primary_color);
                        } else if is_mouse_button_down(MouseButton::Right) {
                            grid.set(x, y, secondary_color);
                        }
                        painted_this_frame.push((x, y));
                        painted_this_frame.sort_unstable();
                        painted_this_frame.dedup();
                    }
                }
            }
            if is_mouse_button_released(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Right) {
                painted_this_frame.clear();
            }

            // Scroll handling
            grid_cell_size += mouse_wheel().1 as i32;
            grid_cell_size = grid_cell_size.clamp(4, 64);

            // Keyboard handling
            if is_key_pressed(KeyCode::Space) {
                (x_pan_pos, y_pan_pos) = mouse_position();
            }

            // Panning
            if is_key_down(KeyCode::Space) {
                let (x, y) = mouse_position();

                x_pan_offset += (x - x_pan_pos) * 0.5;
                y_pan_offset += (y - y_pan_pos) * 0.5;

                (x_pan_pos, y_pan_pos) = (x, y);
            }
        }

        egui_macroquad::draw();
        next_frame().await
    }
}
