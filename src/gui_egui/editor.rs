use crate::common::{Components, EguiComponent, Id, Input};
use crate::components::*;
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::{
    editor_wire_mode::WireMode,
    gui::Gui,
    helper::{id_ports_of_all_components_non_wires, offset_helper},
    keymap,
    library::InputMode,
    menu::Menu,
};
use eframe::{egui, Frame};
use egui::{Color32, Context, LayerId, PointerButton, Pos2, Rect, Shape, Vec2};
use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
    path::Path,
    rc::Rc,
};

pub struct Editor {
    pub components: Components,
    pub scale: f32,
    pub pan: Vec2,
    pub offset: Vec2,
    pub offset_and_pan: Vec2,
    pub clip_rect: Rect,
    pub side_panel_width: f32,
    pub ui_change: bool,
    pub snap_distance: f32,
    pub grid: GridOptions,
    pub library: Components,
    pub dummy_input: Input,
    pub editor_mode: EditorMode,
    pub wm: WireMode,
    pub im: InputMode,
    pub contexts: HashMap<crate::common::Id, EguiExtra>,
}

#[derive(Clone)]
pub struct CloseToComponent {
    pub comp: Rc<dyn EguiComponent>,
    pub pos: Pos2,
    pub dist: f32,
    pub port_id: Id,
    pub potential_actual_input: Option<Input>,
}

#[derive(Debug, Clone, Copy)]
pub enum EditorMode {
    Simulator,
    Default,
    Wire,
    Input,
}

#[cfg(feature = "gui-egui")]
pub struct EditorRenderReturn {
    pub delete: bool,
    pub resp: Option<Vec<egui::Response>>,
}

// Specific structs for egui
#[cfg(feature = "gui-egui")]
pub enum SnapPriority {
    Default,
    Wire,
}

#[derive(Clone)]
pub struct GridOptions {
    pub enable: bool,
    pub size: f32,
    pub opacity: f32,
    pub snap_enable: bool,
    pub snap_distance: f32,
}
#[derive(Clone)]
pub struct Library(pub Components);
impl Default for Library {
    fn default() -> Library {
        let dummy_input = Input::new("id", "field");
        let library: Components = vec![
            Rc::new(Add {
                id: "add".to_string(),
                pos: (0.0, 0.0),
                a_in: dummy_input.clone(),
                b_in: dummy_input.clone(),
            }),
            Rc::new(Constant {
                id: "c".to_string(),
                pos: (0.0, 0.0),
                value: 0.into(),
            }),
            Rc::new(Probe {
                id: "p".to_string(),
                pos: (0.0, 0.0),
                input: dummy_input.clone(),
            }),
            Rc::new(ProbeEdit::new("pe", (0.0, 0.0))),
            Rc::new(Sext {
                id: "sext".to_string(),
                pos: (0.0, 0.0),
                sext_in: dummy_input.clone(),
                in_size: 16,
                out_size: 24,
            }),
            Rc::new(Mem {
                id: "mem".to_string(),
                pos: (0.0, 0.0),
                width: 100.0,
                height: 50.0,
                big_endian: true,
                data: dummy_input.clone(),
                addr: dummy_input.clone(),
                ctrl: dummy_input.clone(),
                size: dummy_input.clone(),
                sext: dummy_input.clone(),
                range: Range {
                    start: 0,
                    end: 0x20,
                },
                memory: Memory::new(BTreeMap::new()),
            }),
            Rc::new(Mux {
                id: "mux".to_string(),
                pos: (0.0, 0.0),
                select: dummy_input.clone(),
                m_in: vec![dummy_input.clone(), dummy_input.clone()],
            }),
            Rc::new(Register {
                id: "reg".to_string(),
                pos: (0.0, 0.0),
                r_in: dummy_input.clone(),
            }),
        ];
        Library(library)
    }
}
impl Editor {
    pub fn gui(components: Components, _path: &Path, library: &Library) -> Self {
        let dummy_input = Input::new("id", "field");
        let library: Components = library.clone().0;
        let library_contexts = crate::gui_egui::gui::create_contexts(&library);
        let mut e = Editor {
            components,
            scale: 1f32,
            pan: Vec2::new(0f32, 0f32),
            offset: Vec2::new(0f32, 0f32),
            offset_and_pan: Vec2::new(0f32, 0f32),
            clip_rect: Rect {
                min: Pos2 { x: 0f32, y: 0f32 },
                max: Pos2 {
                    x: 1000f32,
                    y: 1000f32,
                },
            },
            side_panel_width: 550f32,
            ui_change: true,
            snap_distance: 10f32,
            grid: GridOptions {
                enable: true,
                size: 20f32,
                opacity: 0.5f32,
                snap_enable: true,
                snap_distance: 20f32,
            },
            library,
            dummy_input,
            editor_mode: EditorMode::Default,
            wm: WireMode {
                mode_ended: true,
                last_pos: None,
                input: None,
                cursor_location: Pos2::ZERO,
                start_comp_port: None,
                temp_positions: vec![],
            },
            im: InputMode {
                comp: None,
                cursor_location: Pos2::ZERO,
                library_contexts,
            },
            contexts: HashMap::new(),
        };
        e.contexts = crate::gui_egui::gui::create_contexts(&e.components);
        e
    }

    pub fn update(ctx: &Context, _frame: &mut Frame, gui: &mut Gui) {
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
            let e = Editor::gui_to_editor(gui);
            e.offset = egui::Vec2 {
                x: side.rect.max.x,
                y: top.rect.max.y,
            };
            e.offset_and_pan = e.pan + e.offset;
            e.clip_rect = egui::Rect {
                min: egui::Pos2 {
                    x: e.offset.to_pos2().x,
                    y: e.offset.to_pos2().y,
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
            if gui.editor_use {
                Editor::library(ctx, gui);
                Editor::draw_area(ctx, gui, frame);
            }
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
                        let e = Editor::gui_to_editor(gui);
                        crate::gui_egui::library::show_library(e, ui);
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
                let e = Editor::gui_to_editor(gui);
                ui.painter().add(Shape::line(
                    vec![
                        offset_helper((30f32, 0f32), e.scale, e.offset_and_pan),
                        offset_helper((0f32, 0f32), e.scale, e.offset_and_pan),
                        offset_helper((0f32, 30f32), e.scale, e.offset_and_pan),
                    ],
                    egui::Stroke {
                        width: e.scale,
                        color: Color32::BLACK,
                    },
                ));
                layer_id = Some(ui.layer_id());
            }

            // draw grid
            if Editor::gui_to_editor(gui).grid.enable {
                let e = Editor::gui_to_editor(gui);
                let screen_rect = ui.ctx().screen_rect();
                let grid_scale = e.grid.size * e.scale;
                let start = -(e.pan / e.grid.size / e.scale).floor();

                let end =
                    (Vec2::new(screen_rect.width(), screen_rect.height()) / e.scale / e.grid.size)
                        .ceil()
                        + start;

                for y in (start.y as i32)..(end.y as i32) {
                    ui.painter().hline(
                        0f32..=screen_rect.width(),
                        y as f32 * grid_scale + e.offset_and_pan.y,
                        egui::Stroke {
                            width: e.scale * 0.5f32,
                            color: egui::Color32::BLACK.gamma_multiply(e.grid.opacity),
                        },
                    );
                }
                for x in (start.x as i32)..(end.x as i32) {
                    ui.painter().vline(
                        x as f32 * grid_scale + e.offset_and_pan.x,
                        0f32..=screen_rect.height(),
                        egui::Stroke {
                            width: e.scale * 0.5f32,
                            color: egui::Color32::BLACK.gamma_multiply(e.grid.opacity),
                        },
                    );
                }
            }

            let e = Editor::gui_to_editor(gui);
            let id_ports = id_ports_of_all_components_non_wires(&e.components);
            // The reason we do this is because some of the input modes requires references to
            // components, but that makes us unable to get the mutable reference to it
            // (We can only get a mutable reference if only ONE reference to it exists)
            match e.editor_mode {
                EditorMode::Wire | EditorMode::Input => {
                    for c in &e.components {
                        let old_key = c.as_ref().get_id_ports().0;
                        //println!("{}, {:?}", old_key, e.contexts);
                        match e.contexts.remove(&old_key) {
                            Some(mut context) => {
                                c.render(
                                    ui,
                                    &mut context,
                                    None,
                                    e.offset + e.pan,
                                    e.scale,
                                    e.clip_rect,
                                    e.editor_mode,
                                );
                                e.contexts.insert(context.id_tmp.clone(), context);
                            }
                            _ => {
                                println!("could remove old key")
                            }
                        }
                    }
                }
                _ => e.components.retain_mut(|c| {
                    let old_key = c.as_ref().get_id_ports().0;
                    let mut context = e.contexts.remove(&old_key).unwrap();
                    let render_return = (*Rc::get_mut(c).unwrap()).render_editor(
                        ui,
                        &mut context,
                        None,
                        e.offset_and_pan,
                        e.scale,
                        e.clip_rect,
                        &id_ports,
                        &e.grid,
                        e.editor_mode,
                    );
                    // only reinsert if it's not getting deleted
                    if !render_return.delete {
                        e.contexts.insert(c.get_id_ports().0, context.clone());
                    }
                    !render_return.delete
                }),
            }
        });
        let e = Editor::gui_to_editor(gui);

        let cpr = central_panel.response.interact(egui::Sense::drag());
        if cpr.dragged_by(PointerButton::Middle) {
            e.pan += cpr.drag_delta();
            e.offset_and_pan = e.pan + e.offset;
        }
        match e.editor_mode {
            EditorMode::Wire => crate::gui_egui::editor_wire_mode::wire_mode(ctx, e, cpr, layer_id),
            EditorMode::Input => crate::gui_egui::library::input_mode(ctx, e, cpr, layer_id),
            EditorMode::Default | EditorMode::Simulator => {
                ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::Default)
            }
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

    fn gui_to_editor(gui: &mut Gui) -> &mut Editor {
        gui.editor.as_mut().unwrap()
    }
}

pub fn get_component(components: &Components, comp: CloseToComponent) -> Option<usize> {
    for (i, c) in components.iter().enumerate() {
        // doing std::ptr::eq doesn't work and this works so I'm going to keep it
        // even if clippy errors on it
        #[allow(clippy::vtable_address_comparisons)]
        if Rc::ptr_eq(c, &comp.comp) {
            drop(comp);
            return Some(i);
        }
    }
    None
}
