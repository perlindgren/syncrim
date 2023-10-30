use crate::common::{ComponentStore, Components, Simulator};
use crate::gui_egui::editor::EditorMode;
use crate::gui_egui::{
    editor::{Editor, Library},
    keymap,
    keymap::Shortcuts,
    menu::Menu,
};
use eframe::{egui, Frame};
use egui::{
    containers, CentralPanel, Color32, Context, PointerButton, Pos2, Rect, ScrollArea, Sense,
    SidePanel, TopBottomPanel, Vec2,
};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Gui {
    pub simulator: Option<Simulator>,
    pub path: PathBuf,
    // History, acts like a stack
    pub scale: f32,
    // When the ui elements change size
    pub ui_change: bool,
    pub offset: Vec2,
    pub pan: Vec2,
    pub clip_rect: Rect,
    pub shortcuts: Shortcuts,
    pub pause: bool,
    pub editor: Option<Editor>,
    pub editor_use: bool,
    pub contexts: HashMap<crate::common::Id, EguiExtra>,
    pub library: Library,
}

#[derive(Clone, Debug)]
pub struct EguiExtra {
    pub properties_window: bool,
    pub size_rect: Rect,
    pub id_tmp: String,
    pub pos_tmp: Pos2,
}

pub fn gui(cs: ComponentStore, path: &PathBuf, library: Library) -> Result<(), eframe::Error> {
    let contexts = create_contexts(&cs.store);
    let simulator = Simulator::new(cs).unwrap();
    let options = eframe::NativeOptions::default();
    let path = path.to_owned();
    simulator.save_dot(&path);

    let gui = Gui {
        path,
        simulator: Some(simulator),
        scale: 1.0f32,
        ui_change: true,
        offset: Vec2 { x: 0f32, y: 0f32 },
        pan: Vec2 { x: 0f32, y: 0f32 },
        clip_rect: Rect::NOTHING,
        shortcuts: Shortcuts::new(),
        pause: true,
        editor: None,
        editor_use: false,
        contexts,
        library,
    };

    eframe::run_native("SyncRim", options, Box::new(|_cc| Box::new(gui)))
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.shortcuts.inputs(ctx, self);
        if self.editor_use {
            crate::gui_egui::editor::Editor::update(ctx, frame, self);
            return;
        }
        let frame = egui::Frame::none().fill(Color32::WHITE);
        //let frame = egui::Frame::canvas(&(*ctx.style()).clone());

        // For getting the correct offset for our drawing we need to get the top bar
        // and side panel of the ui once before we draw
        if self.should_area_update(ctx) {
            // todo: Implement proper light and dark mode?
            // for testing light and dark mode
            //ctx.set_visuals(Visuals::dark());
            //ctx.set_visuals(Visuals::light());
            self.top_bar(ctx);
            //self.side_panel(ctx);
            let top = containers::panel::PanelState::load(ctx, egui::Id::from("topBar")).unwrap();
            // let side = containers::panel::PanelState::load(ctx, egui::Id::from("leftGui")).unwrap();
            self.offset = Vec2 {
                x: 0.0,
                y: top.rect.max.y,
            };
            self.clip_rect = Rect {
                min: self.offset.to_pos2(),
                max: Pos2 {
                    x: f32::INFINITY,
                    y: f32::INFINITY,
                },
            };
            Context::request_repaint(ctx);
        } else {
            self.top_bar(ctx);
            if self.simulator.is_some() {
                // self.side_panel(ctx);
                self.draw_area(ctx, frame);
                if self.simulator.as_ref().unwrap().running {
                    self.simulator.as_mut().unwrap().clock();
                    ctx.request_repaint();
                }
            }
        }
    }
    fn post_rendering(&mut self, _window_size_px: [u32; 2], frame: &Frame) {}
}

impl Gui {
    fn should_area_update(&mut self, ctx: &Context) -> bool {
        if self.ui_change {
            self.ui_change = false;
            true
        } else {
            /* (containers::panel::PanelState::load(ctx, egui::Id::from("topBar"))
            .unwrap()
            .rect
            .max
            .y
            - self.offset.y)
            .abs()
            > 0.1
            || (containers::panel::PanelState::load(ctx, egui::Id::from("leftGui"))
                .unwrap()
                .rect
                .max
                .x
                - self.offset.x)
                .abs()
                > 0.1 */
            false
        }
    }

    fn draw_area(&mut self, ctx: &Context, frame: egui::Frame) {
        let central_panel = CentralPanel::default().frame(frame).show(ctx, |ui| {
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
                    EditorMode::Simulator,
                );
                self.contexts.insert(context.id_tmp.clone(), context);
            }
        });
        let cpr = central_panel.response.interact(Sense::drag());
        if cpr.dragged_by(PointerButton::Middle) {
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
        if self.simulator.as_ref().unwrap().running {
            self.simulator.as_mut().unwrap().clock();
            ctx.request_repaint();
        }
    }

    fn side_panel(&mut self, ctx: &Context) {
        SidePanel::left("leftGui").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("0x00000004\n0x00000008\n".repeat(1000));
                    ui.label("100000\n20000\n".repeat(1000));
                });
            });
        });
    }

    fn side_panel_inst(&mut self, ctx: &Context, data: u32) {}

    fn top_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("topBar").show(ctx, |ui| Menu::new(ui, self));
    }
}

pub fn create_contexts(components: &Components) -> HashMap<crate::common::Id, EguiExtra> {
    let mut contexts = HashMap::new();
    for c in &components.clone() {
        let id = c.get_id_ports().0;
        let pos = c.get_pos();
        contexts.insert(
            id.clone(),
            EguiExtra {
                properties_window: false,
                size_rect: Rect::NAN,
                id_tmp: id,
                pos_tmp: Pos2::new(pos.0, pos.1),
            },
        );
    }
    contexts
}
