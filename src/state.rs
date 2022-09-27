use derivative::Derivative;
use macroquad::prelude::*;

use crate::compact;
use crate::grid::Grid;
use crate::undo::{ Undo, Action };
use crate::util::{ rgba_to_hex, hex_to_rgba };

#[derive(Derivative)]
#[derivative(Default)]
pub struct State {
    // ---------- [ Grid ] ----------
    /// Internal grid
    #[derivative(Debug = "ignore")]
    #[derivative(Default(value = "Grid::new(16, 16)"))]
    grid: Grid,
    /// A vector containing all cell's coordinates that have
    /// been modified at this frame.
    painted_cells: Vec<(usize, usize)>,

    /// The current primary color (left mouse)
    #[derivative(Default(value = "[255, 255, 255, 255]"))]
    primary_color: [u8; 4],
    /// The current secondary color (right mouse)
    #[derivative(Default(value = "[0, 0, 0, 255]"))]
    secondary_color: [u8; 4],
    /// Primary color's text input string
    primary_color_input: String,
    /// Secondary color's text input string
    secondary_color_input: String,

    // ---------- [ Undo ] ----------
    /// The undo stack.
    #[derivative(Debug = "ignore")]
    #[derivative(Default(value = "Undo::new()"))]
    undo: Undo,

    // ---------- [ UI ] ----------
    /// Zoom level
    #[derivative(Default(value = "24"))]
    zoom: i32,
    /// Is the mouse is hovering over the grid
    is_on_gui: bool,
    /// Grid offset (pan offset + middle offset + zoom level offset)
    grid_offset: (f32, f32),
    /// Grid pan offset relative to the center of the screen
    pan_offset: (f32, f32),
    /// Grid pan offset relative to the mouse position this frame
    pan_pos: (f32, f32),
}

impl State {
    pub fn new() -> Self { Self::default() }

    pub fn init(&mut self) {
        self.undo.push(Action::Clear, &self.grid);
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

    /// Update the state one frame
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
        let border_color = Color::from_rgba(68, 81, 94, 255);

        // Grid cells
        (0..self.grid.width).for_each(|x| {
            (0..self.grid.height).for_each(|y| {
                let cell_color = self.grid.get(x, y);
                let (x, y, w, h) = (
                    self.grid_offset.0 + x as f32 * self.zoom as f32, self.grid_offset.1 + y as f32 * self.zoom as f32,
                    self.zoom as f32, self.zoom as f32,
                );
                let (w_half, h_half) = (w / 2.0, h / 2.0);

                match cell_color[3] {
                    0..=254 => {
                        let light = Color::from_rgba(198, 202, 206, 255);
                        let dim = Color::from_rgba(161, 168, 174, 255);

                        // Draw transparent checkerboard pattern
                        draw_rectangle(x, y, w_half, h_half, light);
                        draw_rectangle(x, y + h_half, w_half, h_half, dim);
                        draw_rectangle(x + w_half, y, w_half, h_half, dim);
                        draw_rectangle(x + w_half, y + h_half, w_half, h_half, light);
                        draw_rectangle(x, y, w, h, compact!(cell_color));
                    }
                    255 => {
                        draw_rectangle(x, y, w, h, compact!(cell_color));
                    }
                }
            });
        });

        // Grid border lines
        draw_rectangle_lines(
            self.grid_offset.0, self.grid_offset.1,
            self.grid.width as f32 * self.zoom as f32, self.grid.height as f32 * self.zoom as f32,
            1.0,
            border_color,
        );

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

            let widget_style = egui::style::WidgetVisuals {
                bg_fill: egui::Color32::from_rgba_premultiplied(10, 12, 14, 192),
                bg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(34, 36, 38)),
                rounding: Default::default(),
                fg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(236, 237, 238)),
                expansion: Default::default(),
            };

            style.visuals.widgets.noninteractive = widget_style.clone();
            style.visuals.widgets.inactive = widget_style.clone();
            style.visuals.widgets.hovered = widget_style.clone();
            style.visuals.widgets.active = widget_style.clone();

            ctx.set_style(style);

            // Panels
            let grid_actions = egui::Window::new("Grid Actions").show(ctx, |ui| {
                ui.add(egui::Slider::new(&mut self.zoom, 4..=64).text("Zoom"));

                ui.label("Clear grid");
                if ui.button("Clear").clicked() {
                    self.undo.push(Action::Clear, &self.grid);
                    self.grid.clear();
                }
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

            let info = egui::TopBottomPanel::bottom("info").show(ctx, |ui| {
                ui.label(format!("FPS: {}", get_fps()));
            });

            // Check if the GUI is using pointer
            // so that it blocks the mouse from drawing
            [
                colors,
                grid_actions,
            ].iter().for_each(|panel| {
                if let Some(panel) = panel {
                    if panel.response.ctx.is_using_pointer() {
                        self.is_on_gui = true;
                    } else {
                        self.is_on_gui = false;
                    }
                }
            });
            // For component that isn't wrapped in Option
            [
                info
            ].iter().for_each(|panel| {
                if panel.response.ctx.is_using_pointer() {
                    self.is_on_gui = true;
                } else {
                    self.is_on_gui = false;
                }
            });
        });
    }

    /// Process user inputs
    fn input(&mut self) {
        if !self.is_on_gui {
            // On mouse down
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
            // On mouse up
            if is_mouse_button_released(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Right) {
                // Push undo action
                self.undo.push(
                    Action::Paint(
                        self.painted_cells.clone(),
                        if is_mouse_button_released(MouseButton::Left) {
                            self.primary_color
                        } else {
                            self.secondary_color
                        }),
                    &self.grid);
                // Clear the painted cells list
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

            // Undo and redo
            if is_key_pressed(KeyCode::Z)
            && is_key_down(KeyCode::LeftControl) {
                if is_key_down(KeyCode::LeftShift) {
                    self.undo.redo(&mut self.grid);
                } else {
                    self.undo.undo(&mut self.grid);
                }
            }
        }
    }
}
