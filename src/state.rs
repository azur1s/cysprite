use derivative::Derivative;
use macroquad::prelude::*;
use egui::{
    FontFamily::Proportional,
    FontId
};
use egui::TextStyle::*;

use crate::compact;
use crate::util::{ rgba_to_hex, hex_to_rgba };

use crate::grid::Grid;
use crate::undo::{ Undo, Action };
use crate::status_message::StatusMessage;

#[derive(PartialEq)]
enum FileType {
    Ron,
    Png,
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct State {
    // ---------- [ Grid ] ----------
    /// Internal grid
    #[derivative(Debug = "ignore")]
    #[derivative(Default(value = "Grid::new(16, 16)"))]
    grid: Grid,
    /// Grid size (doesn't mutate the grid directly)
    #[derivative(Default(value = "(16, 16)"))]
    grid_size: (usize, usize),
    /// File path
    file_path: String,
    /// File type
    #[derivative(Default(value = "FileType::Ron"))]
    file_type: FileType,
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
    /// The egui's style
    #[derivative(Default(value = "egui::Style::default()"))]
    style: egui::Style,
    /// Zoom level
    #[derivative(Default(value = "32"))]
    zoom: i32,
    /// Is the mouse is hovering over the grid
    is_on_gui: bool,
    /// Grid offset (pan offset + middle offset + zoom level offset)
    grid_offset: (f32, f32),
    /// Grid pan offset relative to the center of the screen
    pan_offset: (f32, f32),
    /// Grid pan offset relative to the mouse position this frame
    pan_pos: (f32, f32),
    /// Status message
    #[derivative(Default(value = "StatusMessage::new()"))]
    status: StatusMessage,
    /// Frame per second display
    #[derivative(Default(value = "0"))]
    fps: u32,
    last_fps_update: f64,
}

impl State {
    pub fn new() -> Self { Self::default() }

    pub fn init(&mut self) {
        self.style.text_styles = [
            (Heading, FontId::new(24.0, Proportional)),
            (Body, FontId::new(20.0, Proportional)),
            (Monospace, FontId::new(20.0, Proportional)),
            (Button, FontId::new(20.0, Proportional)),
            (Small, FontId::new(20.0, Proportional)),
        ].into();

        self.style.spacing.item_spacing = [8.0, 8.0].into();
        self.style.spacing.window_margin = egui::style::Margin::same(8.0);

        self.status.set("Welcome to Harcana!", 5.0);
    }

    /// Calculate grid boundaries.
    /// Return a tuple with minimum and maximum coordinates of X and Y.
    fn grid_bounds(&self) -> (f32, f32, f32, f32) {(
        // X min
        self.grid_offset.0,
        // X max
        self.grid_offset.0 + self.grid.width as f32 * self.zoom as f32,
        // Y min
        self.grid_offset.1,
        // Y max
        self.grid_offset.1 + self.grid.height as f32 * self.zoom as f32,
    )}

    /// Update the state one frame
    pub fn update(&mut self) {
        let middle_offset = (
            (screen_width() - self.grid.width as f32 * self.zoom as f32) / 2.0,
            (screen_height() - self.grid.height as f32 * self.zoom as f32) / 2.0,
        );
        self.grid_offset = (middle_offset.0 + self.pan_offset.0, middle_offset.1 + self.pan_offset.1);

        // Update Fps every second
        if get_time() - self.last_fps_update > 1.0 {
            self.fps = get_fps() as u32;
            self.last_fps_update = get_time();
        }

        self.status.update(get_frame_time());

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
                    // Draw checkerboard pattern for translucent cells
                    0..=254 => {
                        let light = Color::from_rgba(198, 202, 206, 255);
                        let dim = Color::from_rgba(161, 168, 174, 255);

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
            ctx.set_style(self.style.clone());

            // Panels
            fn make_window(
                title: &str, ctx: &egui::Context, content: impl FnOnce(&mut egui::Ui)
            ) -> Option<egui::InnerResponse<Option<()>>> {
                egui::Window::new(title)
                    .resizable(false)
                    .collapsible(false)
                    .show(ctx, content)
            }

            let file_actions = make_window("File", ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Save as:");
                    ui.add(egui::TextEdit::singleline(&mut self.file_path)
                           .desired_width(100.0));
                });

                ui.horizontal(|ui| {
                    ui.label("File type");
                    ui.selectable_value(&mut self.file_type, FileType::Ron, "Ron");
                    ui.selectable_value(&mut self.file_type, FileType::Png, "Png");
                });

                if ui.button("Save").clicked() {
                    match self.file_type {
                        FileType::Ron => {
                            match self.grid.save_as_ron(&self.file_path) {
                                Ok(_) => self.status.set("Saved as .ron", 5.0),
                                Err(e) => self.status.set(&format!("Error: {}", e), 5.0),
                            }
                        }
                        FileType::Png => {
                            match self.grid.save_as_png(&self.file_path) {
                                Ok(_) => self.status.set("Saved as .png", 5.0),
                                Err(e) => self.status.set(&format!("Error: {}", e), 5.0),
                            }
                        }
                    }
                }
            });

            let grid_actions = make_window("Grid", ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Zoom");
                    ui.add(egui::DragValue::new(&mut self.zoom)
                        .speed(1.0)
                        .clamp_range(4..=64));
                });

                ui.horizontal(|ui| {
                    ui.label("Size");
                    ui.add(egui::DragValue::new(&mut self.grid_size.0)
                        .speed(1.0)
                        .clamp_range(1..=4096));
                    ui.add(egui::DragValue::new(&mut self.grid_size.1)
                        .speed(1.0)
                        .clamp_range(1..=4096));
                    if ui.button("Resize").clicked() {
                        self.undo.push(
                            Action::Resize(self.grid_size.0, self.grid_size.1),
                            &self.grid,
                        );
                        self.grid.resize(self.grid_size.0, self.grid_size.1);
                    }
                });

                if ui.button("Clear Grid").clicked() {
                    self.undo.push(Action::Clear, &self.grid);
                    self.grid.clear();
                }
            });

            let colors = make_window("Colors", ctx, |ui| {
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

            let status = egui::TopBottomPanel::bottom("info").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("{}", self.status.get()));
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.label(format!(
                            "Fps: {}, Mem: {:.1} KB",
                            self.fps,
                            procinfo::pid::statm_self().unwrap().size as f32 / 1024.0
                        ));
                    });
                });
            });

            macro_rules! checks_request {
                ($ui: expr) => {
                    if $ui.response.ctx.is_using_pointer()
                    || $ui.response.ctx.is_pointer_over_area()
                    || $ui.response.ctx.wants_keyboard_input()
                    || $ui.response.ctx.wants_keyboard_input() {
                        self.is_on_gui = true;
                    } else {
                        self.is_on_gui = false;
                    }
                }
            }

            // Check if the GUI is requesting input
            // so that it blocks the mouse from drawing
            [
                file_actions,
                grid_actions,
                colors,
            ].iter().for_each(|panel| {
                if let Some(panel) = panel {
                    checks_request!(panel);
                }
            });
            // For component that isn't wrapped in Option
            [
                status,
            ].iter().for_each(|panel| {
                checks_request!(panel);
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

            // Panning (Space)
            if is_key_pressed(KeyCode::Space) {
                self.pan_pos = mouse_position();
            }

            if is_key_down(KeyCode::Space) {
                let (x, y) = mouse_position();

                self.pan_offset.0 += (x - self.pan_pos.0) * 0.5;
                self.pan_offset.1 += (y - self.pan_pos.1) * 0.5;

                self.pan_pos = (x, y);
            }

            // Undo and redo (Ctrl + Z and Ctrl + Shift + Z)
            if is_key_pressed(KeyCode::Z)
            && is_key_down(KeyCode::LeftControl) {
                if is_key_down(KeyCode::LeftShift) {
                    self.status.set(
                        if let Some(act) = self.undo.redo(&mut self.grid) {
                            format!("Redo {}", act)
                        } else {
                            "Nothing more to redo".to_string()
                        }.as_str(),
                        3.0);
                } else {
                    self.status.set(
                        if let Some(act) = self.undo.undo(&mut self.grid) {
                            format!("Undo {}", act)
                        } else {
                            "Nothing more to undo".to_string()
                        }.as_str(),
                        3.0);
                }
            }

            // Zoom (Equal and Minus)
            if is_key_pressed(KeyCode::Equal) {
                self.zoom += 4;
            }
            if is_key_pressed(KeyCode::Minus) {
                self.zoom -= 4;
            }
        }
    }
}
