use egui::{ViewportBuilder, ViewportId, Pos2};

#[derive(Clone, Debug)]
pub struct GuiOptions {
    // This is added/subtracted to/from the view scale when zoomed.
    pub view_scaling_val: f32,
    pub window_visible: bool,
}

impl Default for GuiOptions {
    fn default() -> GuiOptions {
        GuiOptions {
            view_scaling_val: 0.03,
            window_visible: false,
        }
    }
}


impl GuiOptions {
    // Naming would make more sense if it was a GuiOptionsWindow being rendered, then again i see
    // no point in adding more structs for the sake of naming (maybe the point will become apparent
    // down the line).
    pub fn render(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            ViewportId::from_hash_of("Preferences"),
            ViewportBuilder {
                title: Some("Preferences".to_string()),
                position: Some(Pos2::new(ctx.screen_rect().max.x/2.0, ctx.screen_rect().max.y/2.0)),
                inner_size: Some((500.0, 200.0).into()),
                ..ViewportBuilder::default()
            },
            |ctx, _class| {
                if ctx.input(|i| i.viewport().close_requested()) {
                    self.window_visible = false
                }
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Zoom Scaling");
                    let response = ui.add(egui::Slider::new(&mut self.view_scaling_val, 0.0..=0.1));
                    response.on_hover_text("Adjusts the step size by which zooming zooms.");
                });
        });
    }
}
