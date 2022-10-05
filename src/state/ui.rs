use super::State;

use derivative::Derivative;
use macroquad::prelude::*;
use egui::{
    FontFamily::Proportional,
    FontId,
    TextStyle::*,
    widgets::color_picker,
};

#[derive(Derivative)]
#[derivative(Default)]
pub struct Ui {
    #[derivative(Default(value = "egui::Style::default()"))]
    pub style: egui::Style,
    pub width: f32,
}

impl State {
    pub fn init_ui(&mut self) {
        self.ui.style.text_styles = [
            (Heading, FontId::new(24.0, Proportional)),
            (Body, FontId::new(20.0, Proportional)),
            (Monospace, FontId::new(20.0, Proportional)),
            (Button, FontId::new(20.0, Proportional)),
            (Small, FontId::new(20.0, Proportional)),
        ].into();

        self.ui.style.spacing.item_spacing = [8.0, 8.0].into();
        self.ui.style.spacing.window_margin = egui::style::Margin::same(8.0);

        self.ui.width = 400.0;
    }

    pub fn render_ui(&mut self) { egui_macroquad::ui(|ctx| {
        ctx.set_style(self.ui.style.clone());

        let bottom = egui::TopBottomPanel::bottom("Status")
            .min_height(30.0).max_height(30.0).resizable(false)
            .show(ctx, |ui| { ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    ui.label(format!(
                        "{} FPS",
                        get_fps(),
                    ));
                });
            }) });

        let v: [u8; 4] = self.color.into();
        let mut c = egui::Color32::from_rgba_unmultiplied(v[0], v[1], v[2], v[3]);

        let side = egui::SidePanel::left("Menu")
            .min_width(self.ui.width).max_width(self.ui.width).resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.set_width(self.ui.width - 16.0);
                    // Color picker
                    color_picker::color_picker_color32(ui, &mut c, color_picker::Alpha::OnlyBlend);
                });

                ui.add(egui::widgets::Slider::new(&mut self.brush_size, 1..=128)
                    .text("Brush size"));
            });

        let v: [u8; 4] = c.to_srgba_unmultiplied();
        self.color = Color::from_rgba(v[0], v[1], v[2], v[3]);

        macro_rules! checks_request {
            ($ui: expr) => {
                if $ui.response.ctx.is_using_pointer()
                || $ui.response.ctx.is_pointer_over_area()
                || $ui.response.ctx.wants_keyboard_input()
                || $ui.response.ctx.wants_keyboard_input() {
                    self.is_ui_focused = true;
                } else {
                    self.is_ui_focused = false;
                }
            }
        }

        checks_request!(bottom);
        checks_request!(side);
    }); }
}
