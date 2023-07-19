use crate::common::{ComponentStore, EguiComponent, Input};
use crate::components::*;
use crate::gui_egui::{
    gui::Gui,
    helper::{offset_helper, unique_component_name},
    menu::Menu,
};
use eframe::{egui, Frame};
use egui::{Color32, Context, PointerButton, Pos2, Rect, Response, Style, Ui, Vec2};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

pub struct Editor {
    pub component_store: ComponentStore,
    pub scale: f32,
    pub pan: Vec2,
    pub offset: Vec2,
    pub clip_rect: Rect,
    pub side_panel_width: f32,
    pub ui_change: bool,
    pub library: ComponentStore,
    pub dummy_input: Input,
}

impl Editor {
    pub fn gui(cs: ComponentStore, _path: &PathBuf) -> Self {
        let dummy_input = Input::new("id", "field");
        Editor {
            component_store: cs,
            scale: 1f32,
            pan: Vec2::new(0f32, 0f32),
            offset: Vec2 { x: 0f32, y: 0f32 },
            clip_rect: Rect {
                min: Pos2 { x: 0f32, y: 0f32 },
                max: Pos2 {
                    x: 1000f32,
                    y: 1000f32,
                },
            },
            side_panel_width: 400f32,
            ui_change: true,
            library: ComponentStore {
                store: vec![
                    Rc::new(RefCell::new(Add::new(
                        "add".to_string(),
                        (0.0, 0.0),
                        dummy_input.clone(),
                        dummy_input.clone(),
                    ))),
                    Rc::new(RefCell::new(Constant {
                        id: "c".to_string(),
                        pos: (0.0, 0.0),
                        value: 0,
                    })),
                    Rc::new(RefCell::new(Wire {
                        id: "w".to_string(),
                        pos: (0.0, 0.0),
                        delta: (70.0, 0.0),
                        input: dummy_input.clone(),
                    })),
                    Rc::new(RefCell::new(Probe {
                        id: "p".to_string(),
                        pos: (0.0, 0.0),
                        input: dummy_input.clone(),
                    })),
                ],
            },
            dummy_input,
        }
    }

    pub fn update(ctx: &Context, frame: &mut Frame, gui: &mut Gui) {
        let frame = egui::Frame::none().fill(egui::Color32::WHITE);

        if Editor::gui_to_editor(gui).should_area_update(ctx) {
            egui::TopBottomPanel::top("topBarEditor").show(ctx, |ui| {
                Menu::new_editor(ui, gui);
            });
            Editor::library(ctx, gui);
            let top =
                egui::containers::panel::PanelState::load(ctx, egui::Id::from("topBarEditor"))
                    .unwrap();
            let side =
                egui::containers::panel::PanelState::load(ctx, egui::Id::from("leftLibrary"))
                    .unwrap();
            Editor::gui_to_editor(gui).offset = egui::Vec2 {
                x: side.rect.max.x,
                y: top.rect.max.y,
            };
            Editor::gui_to_editor(gui).clip_rect = egui::Rect {
                min: egui::Pos2 {
                    //x: 0f32,
                    x: Editor::gui_to_editor(gui).offset.to_pos2().x,
                    y: Editor::gui_to_editor(gui).offset.to_pos2().y,
                },
                max: egui::Pos2 {
                    x: f32::INFINITY,
                    y: f32::INFINITY,
                },
            };
            egui::Context::request_repaint(ctx);
        } else {
            egui::TopBottomPanel::top("topBarEditor").show(ctx, |ui| {
                Menu::new_editor(ui, gui);
            });
            Editor::library(ctx, gui);
            Editor::draw_area(ctx, gui, frame);
        }
    }

    fn should_area_update(&mut self, ctx: &egui::Context) -> bool {
        if self.ui_change {
            self.ui_change = false;
            true
        } else {
            (egui::containers::panel::PanelState::load(ctx, egui::Id::from("topBarEditor"))
                .unwrap()
                .rect
                .max
                .y
                - self.offset.y)
                .abs()
                > 0.1
                || (egui::containers::panel::PanelState::load(ctx, egui::Id::from("leftLibrary"))
                    .unwrap()
                    .rect
                    .max
                    .x
                    - self.offset.x)
                    .abs()
                    > 0.1
        }
    }

    // Clicking library items will create a clone of them and insert them into the component store
    fn library(ctx: &Context, gui: &mut Gui) {
        egui::SidePanel::left("leftLibrary")
            .default_width(gui.editor.as_mut().unwrap().side_panel_width)
            .frame(egui::Frame::side_top_panel(&(*ctx.style()).clone()).fill(Color32::WHITE))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let s = Editor::gui_to_editor(gui);
                        let mut padding = Vec2 {
                            x: s.offset.x / 2f32,
                            y: s.offset.y + 10f32,
                        };
                        let clip_rect = Rect {
                            min: Pos2 {
                                x: 0f32,
                                y: s.offset.y,
                            },
                            max: Pos2 {
                                x: s.offset.x,
                                y: f32::INFINITY,
                            },
                        };
                        for c in s.library.store.iter() {
                            let size = c.borrow_mut().size();
                            padding.y = padding.y - s.scale * size.min.y;
                            let resp = c
                                .borrow_mut()
                                .render(ui, None, padding, s.scale, clip_rect)
                                .unwrap();

                            // Create new component
                            if resp.drag_started_by(PointerButton::Primary) {
                                let _resp = match c.borrow_mut().get_id_ports().0.as_str() {
                                    // todo: Make this a lot better and not hardcoded
                                    "c" => {
                                        let id = unique_component_name(&s.component_store, "c");
                                        let comp: Rc<RefCell<dyn EguiComponent>> =
                                            Rc::new(RefCell::new(Constant {
                                                id,
                                                pos: (0.0, 0.0),
                                                value: 0,
                                            }));
                                        let resp = comp
                                            .borrow_mut()
                                            .render(ui, None, padding, s.scale, clip_rect);
                                        s.component_store.store.push(comp);
                                        resp
                                    }
                                    "w" => {
                                        let id = unique_component_name(&s.component_store, "w");
                                        let comp: Rc<RefCell<dyn EguiComponent>> =
                                            Rc::new(RefCell::new(Wire {
                                                id,
                                                pos: (0.0, 0.0),
                                                delta: (70.0, 0.0),
                                                input: s.dummy_input.clone(),
                                            }));
                                        let resp = comp
                                            .borrow_mut()
                                            .render(ui, None, padding, s.scale, clip_rect);
                                        s.component_store.store.push(comp);
                                        resp
                                    }
                                    "p" => {
                                        let id = unique_component_name(&s.component_store, "p");
                                        let comp: Rc<RefCell<dyn EguiComponent>> =
                                            Rc::new(RefCell::new(Probe {
                                                id,
                                                pos: (0.0, 0.0),
                                                input: s.dummy_input.clone(),
                                            }));
                                        let resp = comp
                                            .borrow_mut()
                                            .render(ui, None, padding, s.scale, clip_rect);
                                        s.component_store.store.push(comp);
                                        resp
                                    }
                                    "add" | _ => {
                                        let id = unique_component_name(&s.component_store, "add");
                                        let comp: Rc<RefCell<dyn EguiComponent>> =
                                            Rc::new(RefCell::new(Add::new(
                                                id,
                                                (0.0, 0.0),
                                                s.dummy_input.clone(),
                                                s.dummy_input.clone(),
                                            )));
                                        let resp = comp
                                            .borrow_mut()
                                            .render(ui, None, padding, s.scale, clip_rect);
                                        s.component_store.store.push(comp);
                                        resp
                                    }
                                };
                                // todo: use resp to make it draggable instantly
                                // I'm unsure if this is actually possible
                                // since it's not possible to create events/set currently
                                // dragged entity
                            }
                            padding.y = resp.rect.max.y + 10f32;
                        }
                    });
                });
            });
        //
    }

    fn draw_area(ctx: &Context, gui: &mut Gui, frame: egui::Frame) {
        let central_panel = egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.set_clip_rect(Editor::gui_to_editor(gui).clip_rect);

            // draw a marker to show 0,0
            {
                let s = Editor::gui_to_editor(gui);
                ui.painter().add(egui::Shape::line(
                    vec![
                        offset_helper((30f32, 0f32), s.scale, s.offset + s.pan),
                        offset_helper((0f32, 0f32), s.scale, s.offset + s.pan),
                        offset_helper((0f32, 30f32), s.scale, s.offset + s.pan),
                    ],
                    egui::Stroke {
                        width: s.scale,
                        color: egui::Color32::BLACK,
                    },
                ));
            }

            let xd = gui.simulator.ordered_components.clone();
            let s = Editor::gui_to_editor(gui);
            s.component_store.store.retain(|c| {
                let delete = c
                    .borrow_mut()
                    .render_editor(ui, None, s.offset + s.pan, s.scale, s.clip_rect, &xd)
                    .delete;
                !delete
            });
        });

        let cpr = central_panel.response.interact(egui::Sense::drag());
        if cpr.dragged_by(egui::PointerButton::Middle) {
            Editor::gui_to_editor(gui).pan += cpr.drag_delta();
        }
    }

    fn gui_to_editor(gui: &mut Gui) -> &mut Editor {
        gui.editor.as_mut().unwrap()
    }
}
