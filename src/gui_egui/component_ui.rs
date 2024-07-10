use crate::common::{Input, Ports};
use crate::gui_egui::editor::{EditorMode, GridOptions};
use crate::gui_egui::editor_wire_mode::get_grid_snap;
use crate::gui_egui::helper::{
    editor_mode_to_sense, offset_helper, out_of_bounds, unique_component_name,
};
use egui::{
    containers, Color32, ComboBox, Context, DragValue, Frame, Key, KeyboardShortcut, Margin,
    Modifiers, PointerButton, Pos2, Rect, Response, Rounding, Shape, Stroke, Ui, Vec2, Window,
};
use epaint::CircleShape;

use super::helper;

pub fn rect_with_hover<P>(
    rect: Rect,
    clip_rect: Rect,
    editor_mode: EditorMode,
    ui: &mut Ui,
    id: String,
    f: P,
) -> Response
where
    P: Fn(&mut Ui),
{
    let rect = out_of_bounds(rect, clip_rect);
    let r = ui.allocate_rect(rect, editor_mode_to_sense(editor_mode));

    if r.hovered() && !r.dragged() {
        containers::popup::show_tooltip_for(
            ui.ctx(),
            ui.layer_id(),
            egui::Id::new(id),
            &rect,
            |ui| {
                f(ui);
            },
        );
    }
    r
}

pub fn properties_window<P>(
    ui: &mut Ui,
    id: String,
    resp: &Response,
    properties_window: &mut bool,
    mut f: P,
) where
    P: FnMut(&mut Ui) -> bool,
{
    let mut clicked_dropdown = false;
    if *properties_window {
        let resp = Window::new(format!("Properties: {}", id))
            .frame(Frame {
                inner_margin: Margin::same(10f32),
                outer_margin: Margin::same(0f32),
                rounding: Rounding::same(10f32),
                shadow: helper::shadow_small_dark(),
                fill: ui.visuals().panel_fill,
                stroke: ui.visuals().window_stroke,
            })
            .default_pos(Pos2 {
                x: (resp.rect.min.x + resp.rect.max.x) / 2f32,
                y: (resp.rect.min.y + resp.rect.max.y) / 2f32,
            })
            .show(ui.ctx(), |ui| {
                clicked_dropdown = f(ui);
            });
        if !clicked_dropdown && resp.unwrap().response.clicked_elsewhere() {
            *properties_window = false;
        }
    }
    if resp.clicked_by(PointerButton::Secondary) {
        // Open properties window
        *properties_window = true;
    }
}

/// This function add a horizontal ui section which displays
/// the given pos and allows for modification
pub fn pos_drag_value(ui: &mut Ui, pos: &mut (f32, f32)) {
    ui.horizontal(|ui| {
        ui.label("pos x");
        ui.add(DragValue::new(&mut pos.0));
        ui.label("pos y");
        ui.add(DragValue::new(&mut pos.1));
    });
}

pub fn input_selector_removeable(
    ui: &mut Ui,
    input: &mut Input,
    port_name: crate::common::Id,
    id_ports: &[(crate::common::Id, Ports)],
    own_id: crate::common::Id,
    removable: bool,
) -> (bool, bool) {
    let mut port_id = input.id.clone();
    let mut port_field = input.field.clone();
    let label_port_id = format!("{}.id", port_name.clone());
    let text_port_id = port_id.to_string();
    let label_port_field = format!("{}.field", port_name.clone());
    let text_port_field = port_field.to_string();
    let mut should_be_removed = false;
    ui.horizontal(|ui| {
        ComboBox::from_label(label_port_id)
            .selected_text(text_port_id)
            .show_ui(ui, |ui| {
                for c in id_ports {
                    let id = c.0.clone();
                    if id == own_id {
                        continue;
                    }
                    ui.selectable_value(&mut port_id, id.clone(), id);
                }
            });
        ComboBox::from_label(label_port_field)
            .selected_text(text_port_field)
            .show_ui(ui, |ui| {
                for c in id_ports {
                    let id = c.0.clone();
                    if id != port_id {
                        continue;
                    }
                    let fields = c.1.outputs.clone();
                    for field in fields {
                        ui.selectable_value(&mut port_field, field.clone(), field);
                    }
                }
            });
        if removable && ui.button("🗙").clicked() {
            should_be_removed = true;
        }
    });
    let clicked_dropdown = input.id != port_id || input.field != port_field;

    input.id = port_id;
    input.field = port_field;
    (clicked_dropdown, should_be_removed)
}

pub fn input_selector(
    ui: &mut Ui,
    input: &mut Input,
    port_name: crate::common::Id,
    id_ports: &[(crate::common::Id, Ports)],
    own_id: crate::common::Id,
) -> bool {
    input_selector_removeable(ui, input, port_name, id_ports, own_id, false).0
}

pub fn input_change_id(
    ui: &mut Ui,
    id_tmp: &mut String,
    id: &mut String,
    id_ports: &[(crate::common::Id, Ports)],
) {
    ui.horizontal(|ui| {
        let id_label = ui.label("Id: ");
        let r = ui
            .text_edit_singleline(&mut *id_tmp)
            .labelled_by(id_label.id);
        if (r.lost_focus() || r.clicked_elsewhere()) && *id_tmp != *id {
            *id = unique_component_name(id_ports, (*id_tmp).as_str());
        }
    });
}

pub fn visualize_ports(
    ui: &mut Ui,
    ports: Vec<(crate::common::Id, Pos2)>,
    offset: Vec2,
    scale: f32,
    clip_rect: Rect,
) {
    for (id, pos) in ports {
        let pos = offset_helper((pos.x, pos.y), scale, offset);
        let scalev2 = Vec2 {
            x: scale * 2f32,
            y: scale * 2f32,
        };
        let circle = Shape::Circle(CircleShape {
            center: pos,
            radius: scale * 3f32,
            fill: Color32::TRANSPARENT,
            stroke: Stroke {
                width: 1f32 * scale,
                color: Color32::BLUE,
            },
        });
        ui.painter().add(circle);
        let rect = Rect {
            min: pos - scalev2,
            max: pos + scalev2,
        };
        rect_with_hover(rect, clip_rect, EditorMode::Wire, ui, id.clone(), |ui| {
            ui.label(format!("Port id: {}", id));
        });
    }
}

/// Returns if the dragged object should be deleted
pub fn drag_logic(
    ctx: &Context,
    resp: &Response,
    pos: &mut (f32, f32),
    tmp_pos: &mut Pos2,
    scale: f32,
    offset: Vec2,
    grid: &GridOptions,
) -> bool {
    let mut delete = false;
    if resp.dragged_by(PointerButton::Primary) {
        let mod_none = Modifiers {
            alt: false,
            ctrl: false,
            shift: false,
            mac_cmd: false,
            command: false,
        };

        if ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: mod_none,
                logical_key: Key::Delete,
            })
        }) || ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: mod_none,
                logical_key: Key::X,
            })
        }) {
            delete = true;
        }
        let delta = resp.drag_delta() / scale;
        *tmp_pos += delta;
        if grid.enable && grid.snap_enable {
            match get_grid_snap(grid.snap_distance, *tmp_pos, grid.size) {
                Some(p) => *pos = (p.x, p.y),
                None => *pos = (pos.0 + delta.x, pos.1 + delta.y),
            }
        } else {
            *pos = (pos.0 + delta.x, pos.1 + delta.y);
        }
    }
    if resp.drag_stopped_by(PointerButton::Primary)
        && resp.interact_pointer_pos().unwrap().x < offset.x
    {
        delete = true;
    }
    delete
}
