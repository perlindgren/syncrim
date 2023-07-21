use crate::common::{ComponentStore, Components, EguiComponent, Id, Input};
use crate::components::*;
use crate::gui_egui::{
    gui::Gui,
    helper::{offset_helper, offset_reverse_helper_pos2, unique_component_name},
    keymap,
    menu::Menu,
};
use eframe::{egui, Frame};
use egui::{
    Color32, Context, LayerId, PointerButton, Pos2, Rect, Response, Shape, Stroke, Style, Ui, Vec2,
};
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
    pub wire_mode: bool,
    pub wire_mode_ended: bool,
    pub wire_last_pos: Option<Pos2>,
    pub wire_input: Option<Input>,
    pub wire_cursor_location: Pos2,
    pub wire_start_comp_port: Option<CloseToComponent>,
    pub wire_end_comp_port: Option<CloseToComponent>,
    pub wire_temp_positions: Vec<(f32, f32)>,
}

#[derive(Clone)]
pub struct CloseToComponent {
    //comp: Ref<EguiComponent>,
    pub comp: Rc<RefCell<dyn EguiComponent>>,
    pub pos: Pos2,
    pub dist: f32,
    pub port: Id,
}
// todo: enum for input mode, wire, component, none

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
                    Rc::new(RefCell::new(Constant::new("c".to_string(), (0.0, 0.0), 0))),
                    Rc::new(RefCell::new(Wire::new(
                        "w".to_string(),
                        vec![(0.0, 0.0), (70.0, 0.0)],
                        dummy_input.clone(),
                    ))),
                    Rc::new(RefCell::new(Probe::new(
                        "p".to_string(),
                        (0.0, 0.0),
                        dummy_input.clone(),
                    ))),
                ],
            },
            dummy_input,
            wire_mode: false,
            wire_mode_ended: true,
            //wire_current: None,
            wire_last_pos: None,
            wire_input: None,
            wire_cursor_location: Pos2::ZERO,
            wire_start_comp_port: None,
            wire_end_comp_port: None,
            wire_temp_positions: vec![],
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
                            let r_vec = c
                                .borrow_mut()
                                .render(ui, None, padding, s.scale, clip_rect)
                                .unwrap();
                            let rect = r_vec[0].rect.clone();
                            for resp in r_vec {
                                // Create new component
                                if resp.drag_started_by(PointerButton::Primary) {
                                    let _resp = match c.borrow_mut().get_id_ports().0.as_str() {
                                        // todo: Make this a lot better and not hardcoded
                                        "c" => {
                                            let id = unique_component_name(
                                                &s.component_store.store,
                                                "c",
                                            );
                                            let comp: Rc<RefCell<dyn EguiComponent>> = Rc::new(
                                                RefCell::new(Constant::new(id, (0.0, 0.0), 0)),
                                            );
                                            let resp = comp
                                                .borrow_mut()
                                                .render(ui, None, padding, s.scale, clip_rect);
                                            s.component_store.store.push(comp);
                                            resp
                                        }
                                        "w" => {
                                            let id = unique_component_name(
                                                &s.component_store.store,
                                                "w",
                                            );
                                            let comp: Rc<RefCell<dyn EguiComponent>> =
                                                Rc::new(RefCell::new(Wire::new(
                                                    id,
                                                    vec![(0.0, 0.0), (70.0, 0.0)],
                                                    s.dummy_input.clone(),
                                                )));
                                            let resp = comp
                                                .borrow_mut()
                                                .render(ui, None, padding, s.scale, clip_rect);
                                            s.component_store.store.push(comp);
                                            resp
                                        }
                                        "p" => {
                                            let id = unique_component_name(
                                                &s.component_store.store,
                                                "p",
                                            );
                                            let comp: Rc<RefCell<dyn EguiComponent>> =
                                                Rc::new(RefCell::new(Probe::new(
                                                    id,
                                                    (0.0, 0.0),
                                                    s.dummy_input.clone(),
                                                )));
                                            let resp = comp
                                                .borrow_mut()
                                                .render(ui, None, padding, s.scale, clip_rect);
                                            s.component_store.store.push(comp);
                                            resp
                                        }
                                        "add" | _ => {
                                            let id = unique_component_name(
                                                &s.component_store.store,
                                                "add",
                                            );
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
                            }
                            padding.y = rect.max.y + 10f32;
                        }
                    });
                });
            });
        //
    }

    fn draw_area(ctx: &Context, gui: &mut Gui, frame: egui::Frame) {
        let mut layer_id: Option<LayerId> = None;
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
                layer_id = Some(ui.layer_id());
            }

            let tcs = gui.simulator.ordered_components.clone();
            let s = Editor::gui_to_editor(gui);
            if s.wire_mode {
                for c in &s.component_store.store {
                    c.borrow_mut()
                        .render(ui, None, s.offset + s.pan, s.scale, s.clip_rect);
                }
            } else {
                s.component_store.store.retain(|c| {
                    let delete = c
                        .borrow_mut()
                        .render_editor(ui, None, s.offset + s.pan, s.scale, s.clip_rect, &tcs)
                        .delete;
                    !delete
                });
            }
        });
        let s = Editor::gui_to_editor(gui);

        let cpr = central_panel.response.interact(egui::Sense::drag());
        if cpr.dragged_by(PointerButton::Middle) {
            s.pan += cpr.drag_delta();
        }
        if s.wire_mode {
            ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::Crosshair);

            if cpr.drag_started_by(PointerButton::Primary) {
                // todo: Snap functionality should also apply here
                s.wire_mode_ended = false;
                ctx.input_mut(|i| {
                    let origin = i.pointer.press_origin().unwrap();
                    s.wire_cursor_location = origin;

                    let offset_cursor_scale = offset_reverse_helper_pos2(origin, s.scale, s.offset);
                    let closest = Editor::clicked_close_to_input_output(
                        offset_cursor_scale,
                        &s.component_store.store,
                    );
                    let closest_uw = closest.unwrap();

                    if s.wire_temp_positions.len() == 0 {
                        // requires at least one component on the canvas
                        let new_pos = closest_uw.pos;
                        let new_pos = offset_helper((new_pos.x, new_pos.y), s.scale, s.offset);
                        s.wire_temp_positions.push((new_pos.x, new_pos.y));
                    } else if s.wire_temp_positions.len() > 0 && closest_uw.dist <= 10.0f32 {
                        // We should finish the component
                        let in_c = s.wire_start_comp_port.as_ref().unwrap();
                        let out_c = s.wire_end_comp_port.as_ref().unwrap();
                        let input = Editor::get_input_from_port(
                            &in_c.port,
                            &in_c.comp,
                            &out_c.port,
                            &out_c.comp,
                        );
                        match input {
                            Some(i) => {
                                let id = unique_component_name(&s.component_store.store, "w");
                                s.component_store.
                                    store.push(Rc::new(RefCell::new(Wire::new(id, s.wire_temp_positions.clone(), i))));
                            },
                            None => {
                                println!("Seems like you don't have an input at the start or end of the wire");
                            }
                        }
                    } else {
                        s.wire_temp_positions.push((origin.x, origin.y));
                    }
                });
            } else {
                if cpr.drag_started_by(PointerButton::Secondary) {
                    // place wire end
                    // This should also occur when pressing an input/output after the first one
                    s.reset_wire_mode();
                }

                if !s.wire_mode_ended {
                    ctx.input_mut(|i| {
                        s.wire_cursor_location += i.pointer.delta();
                    });
                    let offset_cursor_scale =
                        offset_reverse_helper_pos2(s.wire_cursor_location, s.scale, s.offset);
                    let closest = Editor::clicked_close_to_input_output(
                        offset_cursor_scale,
                        &s.component_store.store,
                    );

                    let wire_shown_location = match closest {
                        Some(c) => {
                            if c.dist <= 10.0f32 {
                                // We are close enough to move the shown wire to here instead
                                println!("Closest: {:?} {}", c.pos, c.dist);
                                c.pos + s.offset
                            } else {
                                s.wire_cursor_location
                            }
                        }
                        None => s.wire_cursor_location,
                    };

                    let mut draw_vec: Vec<Pos2> = vec![]; // = s.wire_temp_positions.clone();
                    for (posx, posy) in &s.wire_temp_positions {
                        draw_vec.push(Pos2::new(*posx, *posy))
                    }
                    draw_vec.push(wire_shown_location);
                    ctx.layer_painter(layer_id.unwrap()).add(Shape::line(
                        draw_vec,
                        Stroke {
                            width: s.scale * 1.5f32,
                            color: Color32::BLACK,
                        },
                    ));
                }
            }
        } else {
            ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::Default);
        }
        if central_panel.response.hovered() {
            ctx.input_mut(|i| {
                if i.scroll_delta.y > 0f32 {
                    keymap::view_zoom_in_fn(gui);
                } else if i.scroll_delta.y < 0f32 {
                    keymap::view_zoom_out_fn(gui);
                }
            });
        }
    }

    fn clicked_close_to_input_output(
        clicked_pos: Pos2,
        components: &Components,
    ) -> Option<CloseToComponent> {
        let mut closest: Option<CloseToComponent> = None;
        for comp in components {
            let ports = comp.borrow_mut().ports_location();
            for (port, pos) in ports {
                match closest.as_ref() {
                    Some(c) => {
                        let dist = clicked_pos.distance(pos);
                        if dist < c.dist {
                            closest = Some(CloseToComponent {
                                comp: comp.clone(),
                                pos,
                                dist,
                                port,
                            })
                        }
                    }
                    None => {
                        closest = Some(CloseToComponent {
                            comp: comp.clone(),
                            pos,
                            dist: clicked_pos.distance(pos),
                            port,
                        })
                    }
                };
            }
        }

        closest
    }

    fn get_input_from_port(
        id_start: &Id,
        comp_start: &Rc<RefCell<dyn EguiComponent>>,
        id_end: &Id,
        comp_end: &Rc<RefCell<dyn EguiComponent>>,
    ) -> Option<Input> {
        let (_, ports_start) = comp_start.borrow().get_id_ports();
        for input in ports_start.inputs {
            if input.id == *id_start {
                return Some(input);
            }
        }
        let (_, ports_end) = comp_end.borrow().get_id_ports();
        for input in ports_end.inputs {
            if input.id == *id_end {
                return Some(input);
            }
        }
        None
    }

    pub fn reset_wire_mode(&mut self) {
        self.wire_mode_ended = true;
        self.wire_last_pos = None;
        self.wire_input = None;
        self.wire_cursor_location = Pos2::ZERO;
        self.wire_start_comp_port = None;
        self.wire_end_comp_port = None;
        self.wire_temp_positions = vec![];
    }

    fn gui_to_editor(gui: &mut Gui) -> &mut Editor {
        gui.editor.as_mut().unwrap()
    }
}
