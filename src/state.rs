use derivative::Derivative;
use macroquad::prelude::*;

mod ui;
mod tex;
mod offsets;
mod inputs;

/// The main program struct.
#[derive(Derivative)]
#[derivative(Default)]
pub struct State {
    /// The texture that is being drawn to.
    #[derivative(Default(value = "Image::empty()"))]
    tex: Image,
    /// Current texture size (doesn't mutate the grid directly,
    /// rather it is used to calculate the grid offset & used to
    /// store inputs for resizing texture).
    #[derivative(Default(value = "(16, 16)"))]
    tex_size: (u32, u32),
    /// The current color for painting.
    #[derivative(Default(value = "Color::from_rgba(255, 255, 255, 255)"))]
    color: Color,

    /// Offsets and zoom.
    offsets: offsets::Offsets,
    /// Mouse and keyboard inputs.
    inputs: inputs::Inputs,
    /// User interface (egui)
    ui: ui::Ui,
    is_ui_focused: bool,
}

impl State {
    pub fn new() -> State {
        let mut s = State {
            tex: Image::gen_image_color(
                16,
                16,
                Color::from_rgba(0, 0, 0, 0),
            ),
            ..Default::default()
        };
        s.init_ui();
        s
    }

    pub fn update(&mut self) {
        clear_background(Color::from_rgba(23, 23, 23, 255));

        self.update_grid_offset();
        self.tex_render();
        self.render_ui();
        self.input();
    }
}
