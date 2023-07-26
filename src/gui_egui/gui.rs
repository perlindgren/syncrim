use crate::common::{ComponentStore, Components, Simulator};
use crate::gui_egui::editor::EditorMode;
use crate::gui_egui::{editor::Editor, keymap, keymap::Shortcuts, menu::Menu};
use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Gui {
    pub simulator: Option<Simulator>,
    pub path: PathBuf,
    // History, acts like a stack
    pub scale: f32,
    // When the ui elements change size
    pub ui_change: bool,
    pub offset: egui::Vec2,
    pub pan: egui::Vec2,
    pub clip_rect: egui::Rect,
    pub shortcuts: Shortcuts,
    pub pause: bool,
    pub editor: Option<Editor>,
    pub editor_use: bool,
    pub contexts: HashMap<crate::common::Id, EguiExtra>,
}

#[derive(Clone, Default, Debug)]
pub struct EguiExtra {
    pub properties_window: bool,
    pub id_tmp: String,
}

pub fn gui(cs: ComponentStore, path: &PathBuf) -> Result<(), eframe::Error> {
    let contexts = create_contexts(&cs.store);
    let simulator = Simulator::new(cs);
    let options = eframe::NativeOptions::default();
    let path = path.to_owned();
    simulator.save_dot(&path);
    println!("contexts: {:?}", contexts);

    let gui = Gui {
        path,
        simulator: Some(simulator),
        scale: 1.0f32,
        ui_change: true,
        offset: egui::Vec2 { x: 0f32, y: 0f32 },
        pan: egui::Vec2 { x: 0f32, y: 0f32 },
        clip_rect: egui::Rect::NOTHING,
        shortcuts: Shortcuts::new(),
        pause: true,
        editor: None,
        editor_use: false,
        contexts,
    };

    eframe::run_native("SyncRim", options, Box::new(|_cc| Box::new(gui)))
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.shortcuts.inputs(ctx, self);
        if self.editor_use {
            crate::gui_egui::editor::Editor::update(ctx, frame, self);
            return;
        }
        let frame = egui::Frame::none().fill(egui::Color32::WHITE);
        //let frame = egui::Frame::canvas(&(*ctx.style()).clone());

        // For getting the correct offset for our drawing we need to get the top bar
        // and side panel of the ui once before we draw
        if self.should_area_update(ctx) {
            // todo: Implement proper light and dark mode?
            // for testing light and dark mode
            //ctx.set_visuals(egui::Visuals::dark());
            //ctx.set_visuals(egui::Visuals::light());
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
            self.clip_rect = egui::Rect {
                min: self.offset.to_pos2(),
                max: egui::Pos2 {
                    x: f32::INFINITY,
                    y: f32::INFINITY,
                },
            };
            egui::Context::request_repaint(ctx);
        } else {
            self.top_bar(ctx);
            if self.simulator.is_some() {
                self.side_panel(ctx);
                self.draw_area(ctx, frame);
            }
        }
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
            let sim = self.simulator.as_mut().unwrap();
            ui.set_clip_rect(self.clip_rect);
            // Don't draw over the rest of the ui
            for c in &sim.ordered_components.clone() {
                let old_key = c.as_ref().get_id_ports().0;
                let mut context = self.contexts.remove(&old_key).unwrap();
                c.render(
                    ui,
                    &mut context,
                    Some(sim),
                    self.offset + self.pan,
                    self.scale,
                    self.clip_rect,
                    EditorMode::Default,
                );
                self.contexts.insert(context.id_tmp.clone(), context);
            }
        });
        let cpr = central_panel.response.interact(egui::Sense::drag());
        if cpr.dragged_by(egui::PointerButton::Middle) {
            self.pan += cpr.drag_delta();
        }
        if central_panel.response.hovered() {
            ctx.input_mut(|i| {
                if i.scroll_delta.y > 0f32 {
                    keymap::view_zoom_in_fn(self);
                } else if i.scroll_delta.y < 0f32 {
                    keymap::view_zoom_out_fn(self);
                }
            });
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

pub fn create_contexts(components: &Components) -> HashMap<crate::common::Id, EguiExtra> {
    let mut contexts = HashMap::new();
    for c in &components.clone() {
        let id = c.get_id_ports().0;
        contexts.insert(
            id.clone(),
            EguiExtra {
                properties_window: false,
                id_tmp: id,
            },
        );
    }
    contexts
}
