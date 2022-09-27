use macroquad::prelude::*;

use crate::grid::Grid;
use crate::util::{ rgba_to_hex, hex_to_rgba };

pub struct State {
    // Grid
    grid: Grid,
    painted_cells: Vec<(usize, usize)>,

    primary_color: [u8; 4],
    secondary_color: [u8; 4],
    primary_color_input: String,
    secondary_color_input: String,

    // UI
    zoom: i32,
    is_on_gui: bool,
    grid_offset: (f32, f32),
    pan_offset: (f32, f32),
    pan_pos: (f32, f32),
}

impl State {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(16, 16),
            painted_cells: vec![],

            primary_color: [255, 255, 255, 255],
            secondary_color: [0, 0, 0, 255],
            primary_color_input: String::new(),
            secondary_color_input: String::new(),

            zoom: 24,
            is_on_gui: false,
            grid_offset: (0.0, 0.0),
            pan_offset: (0.0, 0.0),
            pan_pos: (0.0, 0.0),
        }
    }

    /// Calculate grid boundaries.
    /// Return a tuple with minimum and maximum coordinates of X and Y.
    fn grid_bounds(&self) -> (f32, f32, f32, f32) {
        (
            // X min
            self.grid_offset.0,
            // X max
            self.grid_offset.0 + self.grid.width as f32 * self.zoom as f32,
            // Y min
            self.grid_offset.1,
            // Y max
            self.grid_offset.1 + self.grid.height as f32 * self.zoom as f32,
        )
    }

    /// Update the state one time step
    pub fn update(&mut self) {
        let middle_offset = (
            (screen_width() - self.grid.width as f32 * self.zoom as f32) / 2.0,
            (screen_height() - self.grid.height as f32 * self.zoom as f32) / 2.0,
        );
        self.grid_offset = (middle_offset.0 + self.pan_offset.0, middle_offset.1 + self.pan_offset.1);
        self.render();
        self.input();
    }

    /// Render the grid and the UI
    fn render(&mut self) {
        clear_background(Color::from_rgba(10, 12, 14, 255));
        let grid_color = Color::from_rgba(68, 81, 94, 255);

        // Drawing grid lines
        for x in 0..=self.grid.width {
            draw_line(
                // from
                self.grid_offset.0 + x as f32 * self.zoom as f32,
                self.grid_offset.1,
                // to
                self.grid_offset.0 + x as f32 * self.zoom as f32,
                self.grid_offset.1 + self.grid.height as f32 * self.zoom as f32,
                1.0, // width
                grid_color
            ); 
        }

        for y in 0..=self.grid.height {
            draw_line(
                // from
                self.grid_offset.0,
                self.grid_offset.1 + y as f32 * self.zoom as f32,
                // to
                self.grid_offset.0 + self.grid.width as f32 * self.zoom as f32,
                self.grid_offset.1 + y as f32 * self.zoom as f32,
                1.0, // width
                grid_color
            ); 
        }

        // Rendering grid cells
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let cell_color = self.grid.get(x, y);
                if cell_color[3] != 0 {
                    draw_rectangle(
                        self.grid_offset.0 + x as f32 * self.zoom as f32,
                        self.grid_offset.1 + y as f32 * self.zoom as f32,
                        self.zoom as f32,
                        self.zoom as f32,
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
                    self.grid.clear();
                }

                ui.label("Zoom");
                ui.add(egui::Slider::new(&mut self.zoom, 4..=64).text("Zoom"));
            });

            let colors = egui::Window::new("Colors").show(ctx, |ui| {
                // Macros for adding color inputs
                macro_rules! color_input {
                    ($ui: expr, $color: expr, $color_input: expr) => {
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

                ui.label("Primary");
                ui.horizontal(|ui| {
                    color_input!(ui, self.primary_color, self.primary_color_input);
                });
                ui.label("Secondary");
                ui.horizontal(|ui| {
                    color_input!(ui, self.secondary_color, self.secondary_color_input);
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
                        self.is_on_gui = true;
                    } else {
                        self.is_on_gui = false;
                    }
                }
            });
        });
    }

    /// Process user inputs
    fn input(&mut self) {
        if !self.is_on_gui {
            // Mouse handling
            if is_mouse_button_down(MouseButton::Left)
            || is_mouse_button_down(MouseButton::Right) {
                let (x, y) = mouse_position();

                // Bail out if mouse is outside of self.grid
                let bounds = self.grid_bounds();
                if !(x < bounds.0 || x > bounds.1 || y < bounds.2 || y > bounds.3) {
                    // Align center and convert to self.grid coordinates
                    let x = ((x - self.grid_offset.0) / self.zoom as f32) as usize;
                    let y = ((y - self.grid_offset.1) / self.zoom as f32) as usize;

                    // Don't draw if the cell have already been painted since mouse down
                    if !self.painted_cells.contains(&(x, y)) {
                        if is_mouse_button_down(MouseButton::Left) {
                            self.grid.set(x, y, self.primary_color);
                        } else if is_mouse_button_down(MouseButton::Right) {
                            self.grid.set(x, y, self.secondary_color);
                        }
                        self.painted_cells.push((x, y));
                        self.painted_cells.sort_unstable();
                        self.painted_cells.dedup();
                    }
                }
            }
            if is_mouse_button_released(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Right) {
                self.painted_cells.clear();
            }

            // Scroll handling
            self.zoom += mouse_wheel().1 as i32;
            self.zoom = self.zoom.clamp(4, 64);

            // Keyboard handling
            if is_key_pressed(KeyCode::Space) {
                self.pan_pos = mouse_position();
            }

            // Panning
            if is_key_down(KeyCode::Space) {
                let (x, y) = mouse_position();

                self.pan_offset.0 += (x - self.pan_pos.0) * 0.5;
                self.pan_offset.1 += (y - self.pan_pos.1) * 0.5;

                self.pan_pos = (x, y);
            }
        }
    }
}