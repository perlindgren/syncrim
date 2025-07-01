use crate::common::{EguiComponent, Ports, Simulator};
use crate::components::Wire;
use crate::gui_egui::component_ui::{input_change_id, input_selector, visualize_ports};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions, SnapPriority};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::{basic_on_hover, offset_helper, shadow_small_dark};
use egui::{
    Color32, CornerRadius, DragValue, Frame, Key, KeyboardShortcut, Margin, Modifiers,
    PointerButton, Pos2, Rect, Response, Sense, Shape, Stroke, StrokeKind, Ui, UiBuilder, Vec2,
    Window,
};

/// if the mouse cursor is less than this distance in points away from the wire display tooltip
/// Note points is often same as pixels, but some times differ with the points_per_pixels value in egui
const TOOLTIP_DISTANCE: f32 = 5.0;

/// Calculates the minimum distance the point is from our line going from start: Vec2 to end: Vec2
fn min_from_line(start: Vec2, end: Vec2, point: Vec2) -> f32 {
    // could probably use length_sq, but this don't need to be optimized
    let length: f32 = (end - start).length();
    // if length is zero, get length between start and point
    if length == 0f32 {
        return (start - point).length();
    };

    let dir_to_end: Vec2 = (end - start).normalized();
    let point_rel_to_start: Vec2 = point - start;
    // dot product,
    // a dot b = abs(a)*abs(b)*cos(theta)
    // if abs(a)=1 we can use this to determine how far along the line our point is
    let dist_along_line: f32 = dir_to_end.dot(point_rel_to_start);

    // if we are before our line start
    if dist_along_line < 0f32 {
        // distance to our start point
        (point - start).length() // return this value
    }
    // if our point is after the end of our line
    else if dist_along_line > length {
        // distance to our end point
        (point - end).length() // return this value
    }
    // our point is between the line
    else {
        // project vec a up on vec b
        // theta is the angel between our vectors
        // abs(a) * cos(theta) * b/abs(b)
        // we can se the resemblance to a dot product
        // abs(a) * abs(b) * cos(theta) * b/abs(b) # one to much abs(b)
        // abs(a) * abs(b) * cos(theta) * b/(abs(b))^2
        // a dot b * b/abs(b)^2
        // this is our point projected along our line (line starts at origin (0,0))
        // if abs(b)=1, aka normalized we can simplify the math
        // a dot b * b/1^2
        // a dot b * b
        // a dot b is already calculated in dist along line
        let proj_point_on_line: Vec2 = dist_along_line * dir_to_end;
        // lets use this to calculate the orthogonal vector from our line to our point
        (point_rel_to_start - proj_point_on_line).length() // return this value
    }
}
use log::trace;

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
        let is_active = simulator
            .as_ref()
            .map_or(false, |sim| sim.is_active(&self.input.id));

        trace!("render constant {}, active {}", self.id, is_active);

        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let offset_old = offset;
        let s = scale;
        let o = offset;
        let mut line_vec = vec![];
        for pos in self.pos.clone() {
            line_vec.push(oh(pos, s, o));
        }

        let mut hovered = false;
        let mut r: Vec<Response> = vec![];

        for val in line_vec.windows(2) {
            let first_pos = val[0];
            let last_pos = val[1];
            let rect = Rect::from_two_pos(first_pos, last_pos).expand(2.5);

            #[allow(clippy::single_match)]
            match editor_mode {
                EditorMode::Default => {
                    // why the fuck do i need this much code just to make sure its rendered at the correct layer
                    let resp = ui
                        .allocate_new_ui(
                            UiBuilder::new().layer_id(ui.layer_id()).max_rect(rect),
                            |ui| ui.allocate_exact_size(rect.size(), Sense::all()),
                        )
                        .inner
                        .1;

                    // log::debug!("{:?}", resp);
                    if resp.contains_pointer() {
                        ui.painter().rect_stroke(
                            resp.interact_rect,
                            CornerRadius::same(0),
                            Stroke {
                                width: scale,
                                color: Color32::RED,
                            },
                            StrokeKind::Inside,
                        );
                    }
                    r.push(resp);
                }
                _ => {}
            };

            if let Some(cursor) = ui.ctx().pointer_latest_pos() {
                if min_from_line(first_pos.to_vec2(), last_pos.to_vec2(), cursor.to_vec2())
                    < TOOLTIP_DISTANCE
                    && clip_rect.contains(cursor)
                    && !hovered
                {
                    hovered = true;
                    egui::containers::popup::show_tooltip_at(
                        ui.ctx(),
                        ui.layer_id(),
                        egui::Id::new(&self.id),
                        (first_pos + last_pos.to_vec2()) / 2.0,
                        |ui| basic_on_hover(ui, self, &simulator),
                    );
                }
            };
        }

        let sk = Stroke {
            width: if hovered { scale * 3.0 } else { scale },
            color: Color32::from_rgba_unmultiplied(
                self.color_rgba[0],
                self.color_rgba[1],
                self.color_rgba[2],
                self.color_rgba[3],
            ),
        };
        if is_active {
            ui.painter().add(Shape::line(line_vec.clone(), sk));
        } else {
            Shape::dashed_line(&line_vec, sk, 10.0, 2.0)
                .drain(..)
                .for_each(|s| {
                    ui.painter().add(s);
                });
        }

        match editor_mode {
            EditorMode::Simulator => (),
            _ => visualize_ports(ui, self.ports_location(), offset_old, scale, clip_rect),
        }

        Some(r)
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
                    inner_margin: Margin::same(10),
                    outer_margin: Margin::same(0),
                    corner_radius: CornerRadius::same(10),
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
                    let mut c: Color32 = Color32::from_rgba_unmultiplied(
                        self.color_rgba[0],
                        self.color_rgba[1],
                        self.color_rgba[2],
                        self.color_rgba[3],
                    );
                    ui.color_edit_button_srgba(&mut c);
                    self.color_rgba = c.to_array();

                    let mut i = 0;
                    let mut to_insert: Option<(usize, (f32, f32))> = None;
                    let mut first_item = true;
                    self.pos.retain_mut(|seg_pos| {
                        let mut delete = false;
                        ui.horizontal(|ui| {
                            ui.label(format!("Segment {}:", i));
                            ui.label("pos x");
                            ui.add(DragValue::new(&mut seg_pos.0).speed(0.5));
                            ui.label("pos y");
                            ui.add(DragValue::new(&mut seg_pos.1).speed(0.5));

                            if first_item {
                                first_item = false;
                            } else if ui.button("🗙").clicked() {
                                delete = true;
                            }
                            if ui.button("NEW").clicked() {
                                to_insert = Some((i, *seg_pos));
                            }
                        });
                        i += 1;
                        !delete
                    });
                    if let Some((i, pos)) = to_insert {
                        self.pos.insert(i, pos)
                    };

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
