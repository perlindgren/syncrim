use crate::common::{EguiComponent, Ports, SignalUnsigned, Simulator};
use crate::components::Wire;
use crate::gui_egui::component_ui::{
    input_change_id, input_selector, rect_with_hover, visualize_ports,
};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions, SnapPriority};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::{offset_helper, shadow_small_dark};
use egui::{
    Color32, DragValue, Frame, Key, KeyboardShortcut, Margin, Modifiers, PointerButton, Pos2, Rect,
    Response, Rounding, Shape, Stroke, Ui, Vec2, Window,
};

#[typetag::serde]
impl EguiComponent for Wire {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let offset_old = offset;
        let s = scale;
        let o = offset;
        let mut line_vec = vec![];
        for pos in self.pos.clone() {
            line_vec.push(oh(pos, s, o));
        }

        ui.painter().add(Shape::line(
            line_vec.clone(),
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));
        let mut r_vec = vec![];

        for (i, _) in line_vec[1..].iter().enumerate() {
            let (line_top, line_bottom) = if line_vec[i].x > line_vec[i + 1].x {
                (line_vec[i + 1].x, line_vec[i].x)
            } else {
                (line_vec[i].x, line_vec[i + 1].x)
            };
            let (line_left, line_right) = if line_vec[i].y > line_vec[i + 1].y {
                (line_vec[i + 1].y, line_vec[i].y)
            } else {
                (line_vec[i].y, line_vec[i + 1].y)
            };
            let rect = Rect {
                min: Pos2::new(line_top, line_left),
                max: Pos2::new(line_bottom, line_right),
            };

            let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
                ui.label(format!("Id: {}", self.id.clone()));
                if let Some(s) = &simulator {
                    ui.label({
                        let r: Result<SignalUnsigned, String> =
                            s.get_input_value(&self.input).try_into();
                        match r {
                            Ok(data) => format!("{:#x}", data),
                            _ => format!("{:?}", r),
                        }
                    });
                }
            });
            r_vec.push(r);
        }

        match editor_mode {
            EditorMode::Simulator => (),
            _ => visualize_ports(ui, self.ports_location(), offset_old, scale, clip_rect),
        }

        Some(r_vec)
    }

    fn render_editor(
        &mut self,
        ui: &mut Ui,
        context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        id_ports: &[(crate::common::Id, Ports)],
        _grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let mut delete = false;
        let r_vec = Wire::render(
            self,
            ui,
            context,
            simulator,
            offset,
            scale,
            clip_rect,
            editor_mode,
        )
        .unwrap();

        let mut properties_window_open = false;
        for (i, resp) in r_vec.iter().enumerate() {
            if resp.dragged_by(PointerButton::Primary) {
                if ui.ctx().input_mut(|i| {
                    i.consume_shortcut(&KeyboardShortcut {
                        modifiers: Modifiers {
                            alt: false,
                            ctrl: false,
                            shift: false,
                            mac_cmd: false,
                            command: false,
                        },
                        logical_key: Key::Delete,
                    })
                }) || ui.ctx().input_mut(|i| {
                    i.consume_shortcut(&KeyboardShortcut {
                        modifiers: Modifiers {
                            alt: false,
                            ctrl: false,
                            shift: false,
                            mac_cmd: false,
                            command: false,
                        },
                        logical_key: Key::X,
                    })
                }) {
                    delete = true;
                }
                let delta = resp.drag_delta() / scale;
                self.pos[i] = (self.pos[i].0 + delta.x, self.pos[i].1 + delta.y);
                self.pos[i + 1] = (self.pos[i + 1].0 + delta.x, self.pos[i + 1].1 + delta.y);
            }
            if resp.drag_stopped_by(PointerButton::Primary)
                && resp.interact_pointer_pos().unwrap().x < offset.x
            {
                delete = true;
            }
            properties_window_open |= resp.clicked_by(PointerButton::Secondary);
        }
        let mut clicked_dropdown = false;
        if properties_window_open || context.properties_window {
            let resp = Window::new(format!("Properties: {}", self.id))
                .frame(Frame {
                    inner_margin: Margin::same(10f32),
                    outer_margin: Margin::same(0f32),
                    rounding: Rounding::same(10f32),
                    shadow: shadow_small_dark(),
                    fill: ui.visuals().panel_fill,
                    stroke: ui.visuals().window_stroke,
                })
                .default_pos(Pos2 {
                    x: self.pos[0].0,
                    y: self.pos[0].1,
                })
                .show(ui.ctx(), |ui| {
                    input_change_id(ui, &mut context.id_tmp, &mut self.id, id_ports);
                    clicked_dropdown |= input_selector(
                        ui,
                        &mut self.input,
                        crate::components::WIRE_INPUT_ID.to_string(),
                        id_ports,
                        self.id.clone(),
                    );

                    let mut i = 0;
                    let mut first_item = true;
                    self.pos.retain_mut(|seg_pos| {
                        let mut delete = false;
                        ui.horizontal(|ui| {
                            ui.label(format!("Segment {}:", i));
                            ui.label("pos x");
                            ui.add(DragValue::new(&mut seg_pos.0));
                            ui.label("pos y");
                            ui.add(DragValue::new(&mut seg_pos.1));

                            if first_item {
                                first_item = false;
                            } else if ui.button("🗙").clicked() {
                                delete = true;
                            }
                        });
                        i += 1;
                        !delete
                    });

                    if ui.button("+ Add new segment").clicked() {
                        self.pos.push(*self.pos.last().unwrap());
                    }
                });
            if !context.properties_window {
                context.properties_window = true;
            } else if !clicked_dropdown && resp.unwrap().response.clicked_elsewhere() {
                context.properties_window = false;
            }
        }

        EditorRenderReturn {
            delete,
            resp: Some(r_vec),
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let mut vec: Vec<(crate::common::Id, Pos2)> = vec![];
        for (i, pos) in self.pos.iter().enumerate() {
            vec.push((
                format!("{}-{}", crate::components::WIRE_INPUT_ID, i),
                Pos2 { x: pos.0, y: pos.1 },
            ));
        }
        vec
    }

    fn snap_priority(&self) -> SnapPriority {
        SnapPriority::Wire
    }

    fn get_pos(&self) -> (f32, f32) {
        (0f32, 0f32)
    }
}
