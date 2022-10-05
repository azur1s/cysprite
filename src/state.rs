use derivative::Derivative;
use macroquad::prelude::*;

mod ui;
mod tex;
mod offsets;
mod inputs;

pub const TRANSPARENT_TEX: &[u8; 580] = include_bytes!("../transparent.png");

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

    #[derivative(Default(value = "Texture2D::empty()"))]
    transparent_bg: Texture2D,

    /// Offsets and zoom.
    offsets: offsets::Offsets,
    /// Mouse and keyboard inputs.
    inputs: inputs::Inputs,
    /// User interface (egui)
    ui: ui::Ui,
}

impl State {
    pub fn new() -> State {
        let s = State {
            tex: Image::gen_image_color(
                16,
                16,
                Color::from_rgba(0, 0, 0, 0),
            ),
            transparent_bg: Texture2D::from_file_with_format(TRANSPARENT_TEX, Some(ImageFormat::Png)),
            ..Default::default()
        };
        s.transparent_bg.set_filter(FilterMode::Nearest);
        s
    }

    pub fn update(&mut self) {
        clear_background(BLACK);

        self.update_grid_offset();
        self.tex_render();
        self.input();
    }
}
