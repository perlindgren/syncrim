use crate::common::{ComponentStore, Simulator};
use crate::gui_egui::{keymap::Shortcuts, menu::Menu};
use eframe::egui;
use std::path::PathBuf;

pub struct Gui {
    pub simulator: Simulator,
    pub path: PathBuf,
    // History, acts like a stack
    pub history: Vec<Vec<u32>>,
    pub scale: f32,
    pub clock: usize,
    // When the ui elements change size
    pub ui_change: bool,
    pub offset: egui::Vec2,
    pub pan: egui::Vec2,
    pub shortcuts: Shortcuts,
    pub pause: bool,
}

pub fn gui(cs: &ComponentStore, path: &PathBuf) -> Result<(), eframe::Error> {
    let mut clock = 0;
    let simulator = Simulator::new(cs, &mut clock);
    let options = eframe::NativeOptions::default();
    let path = path.to_owned();
    simulator.save_dot(&path);
    let gui = Gui {
        clock,
        path,
        simulator,
        history: vec![],
        scale: 1.0f32,
        ui_change: true,
        offset: egui::Vec2 { x: 0f32, y: 0f32 },
        pan: egui::Vec2 { x: 0f32, y: 0f32 },
        shortcuts: Shortcuts::new(),
        pause: true,
    };
    eframe::run_native("SyncRim", options, Box::new(|_cc| Box::new(gui)))
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.shortcuts.inputs(ctx, self);
        let frame = egui::Frame::none().fill(egui::Color32::WHITE);

        // For getting the correct offset for our drawing we need to get the top bar
        // and side panel of the ui once before we draw
        if self.should_area_update(ctx) {
            self.top_bar(ctx);
            self.side_panel(ctx);
            let top =
                egui::containers::panel::PanelState::load(ctx, egui::Id::from("topBar")).unwrap();
            let side =
                egui::containers::panel::PanelState::load(ctx, egui::Id::from("leftGui")).unwrap();
            self.offset = egui::Vec2 {
                x: side.rect.max.x,
                y: top.rect.max.y,
            };
            egui::Context::request_repaint(ctx);
            return;
        }

        /*
        println!(
            "size: y: {}, x: {}",
            egui::containers::panel::PanelState::load(ctx, egui::Id::from("topBar"))
                .unwrap()
                .rect
                .max
                .y,
            egui::containers::panel::PanelState::load(ctx, egui::Id::from("leftGui"))
                .unwrap()
                .rect
                .max
                .x
        );
        */

        self.draw_area(ctx, frame);
        self.top_bar(ctx);
        self.side_panel(ctx);
    }
}

impl Gui {
    fn should_area_update(&mut self, ctx: &egui::Context) -> bool {
        if self.ui_change {
            self.ui_change = false;
            true
        } else {
            (egui::containers::panel::PanelState::load(ctx, egui::Id::from("topBar"))
                .unwrap()
                .rect
                .max
                .y
                - self.offset.y)
                .abs()
                > 0.1
                || (egui::containers::panel::PanelState::load(ctx, egui::Id::from("leftGui"))
                    .unwrap()
                    .rect
                    .max
                    .x
                    - self.offset.x)
                    .abs()
                    > 0.1
        }
    }

    fn draw_area(&mut self, ctx: &egui::Context, frame: egui::Frame) {
        let central_panel = egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            for c in &self.simulator.ordered_components {
                c.render(
                    ui,
                    self.simulator.clone(),
                    self.offset + self.pan,
                    self.scale,
                );
            }
        });
        let cpr = central_panel.response.interact(egui::Sense::drag());
        if cpr.dragged_by(egui::PointerButton::Middle) {
            self.pan += cpr.drag_delta();
        }
    }

    fn side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("leftGui").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("0x00000004\n0x00000008\n".repeat(100));
                    ui.label("100000\n20000\n".repeat(100));
                });
            });
        });
    }

    fn top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("topBar").show(ctx, |ui| Menu::new(ui, self));
    }
}
