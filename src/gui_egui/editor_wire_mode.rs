use crate::common::{Components, EguiComponent, Id, Input};
use crate::components::Wire;
use crate::gui_egui::editor::SnapPriority;
use crate::gui_egui::editor::{get_component, CloseToComponent, Editor};
use crate::gui_egui::helper::{
    id_ports_of_all_components, offset_helper, offset_reverse_helper, offset_reverse_helper_pos2,
    unique_component_name,
};
use egui::{Color32, Context, CursorIcon, LayerId, PointerButton, Pos2, Response, Shape, Stroke};
use std::rc::Rc;

pub struct WireMode {
    pub mode_ended: bool,
    pub last_pos: Option<Pos2>,
    pub input: Option<Input>,
    pub cursor_location: Pos2,
    pub start_comp_port: Option<CloseToComponent>,
    pub temp_positions: Vec<(f32, f32)>,
}

pub fn drag_started(ctx: &Context, e: &mut Editor, _cpr: Response) {
    e.wm.mode_ended = false;
    ctx.input_mut(|i| {
        let origin = i.pointer.press_origin().unwrap();
        e.wm.cursor_location = origin;

        let offset_cursor_scale = offset_reverse_helper_pos2(origin, e.scale, e.offset_and_pan);
        let (closest, _closest_wire) =
            clicked_close_to_input_output(offset_cursor_scale, &e.components);
        let closest_uw = closest.unwrap();

        // First click ALWAYS has to start at a port so we force it
        if e.wm.temp_positions.len() == 0 {
            // requires at least one component on the canvas
            let new_pos = closest_uw.pos;
            e.wm.start_comp_port = Some(closest_uw);
            let new_pos = offset_helper((new_pos.x, new_pos.y), e.scale, e.offset_and_pan);
            e.wm.temp_positions.push((new_pos.x, new_pos.y));

            // We clicked close to a port so this will be the last click done
        } else if e.wm.temp_positions.len() > 0 && closest_uw.dist <= 10.0f32 {
            // We should finish the component
            last_click(e, closest_uw);

            // Click somewhere not near a component
        } else {
            let mut wires =
                wire_split_into_two_vec(*e.wm.temp_positions.last().unwrap(), (origin.x, origin.y));
            e.wm.temp_positions.append(&mut wires)
        }
    });
}

pub fn last_click(e: &mut Editor, closest_uw: CloseToComponent) {
    //let in_c = e.wm.start_comp_port.unwrap();
    let in_c = std::mem::replace(&mut e.wm.start_comp_port, None).unwrap();
    let out_c = closest_uw;
    let (field_name, input, is_comp_start) =
        get_outputs_from_port(&in_c.port_id, &in_c.comp, &out_c.port_id, &out_c.comp);
    match input {
        Some(i) => {
            let id_ports = id_ports_of_all_components(&e.components);
            let id = unique_component_name(&id_ports, "w");
            let id = id.as_str();
            let mut pos_v: Vec<(f32, f32)> = vec![];

            for pos in &e.wm.temp_positions {
                let pos_2 = offset_reverse_helper(*pos, e.scale, e.offset_and_pan);
                pos_v.push((pos_2.x, pos_2.y));
            }

            let last_pos = *e.wm.temp_positions.last().unwrap();
            let last_pos = offset_reverse_helper(last_pos, e.scale, e.offset_and_pan);
            let mut v =
                wire_split_into_two_vec((last_pos.x, last_pos.y), (out_c.pos.x, out_c.pos.y));
            pos_v.append(&mut v);

            e.components.push(Rc::new(Wire::new(id, pos_v, i.clone())));

            // Now actually set the input of the wired component
            let comp = if is_comp_start { in_c } else { out_c };
            match get_component(&e.components, comp) {
                Some(c) => Rc::get_mut(&mut e.components[c])
                    .unwrap()
                    .set_id_port(field_name, i),
                None => (),
            }
        }
        None => {
            println!("Seems like you don't have an input at the start or end of the wire");
        }
    }
    reset_wire_mode(&mut e.wm);
}

pub fn wire_mode(ctx: &Context, e: &mut Editor, cpr: Response, layer_id: Option<LayerId>) {
    ctx.output_mut(|o| o.cursor_icon = CursorIcon::Crosshair);

    if cpr.drag_started_by(PointerButton::Primary) {
        drag_started(ctx, e, cpr);
    } else {
        if cpr.drag_started_by(PointerButton::Secondary) {
            // place wire end
            // This should also occur when pressing an input/output after the first one
            reset_wire_mode(&mut e.wm);
        }

        if !e.wm.mode_ended {
            ctx.input_mut(|i| {
                e.wm.cursor_location += i.pointer.delta();
            });
            let offset_cursor_scale =
                offset_reverse_helper_pos2(e.wm.cursor_location, e.scale, e.offset_and_pan);
            let (closest, _closest_wire) =
                clicked_close_to_input_output(offset_cursor_scale, &e.components);

            let wire_shown_location = match closest {
                Some(c) => {
                    if c.dist <= 10.0f32 {
                        // We are close enough to move the shown wire to here instead
                        c.pos + e.offset_and_pan
                    } else {
                        e.wm.cursor_location
                    }
                }
                None => e.wm.cursor_location,
            };

            let v = wire_split_into_two_vec(
                *e.wm.temp_positions.last().unwrap(),
                (wire_shown_location.x, wire_shown_location.y),
            );
            let mut draw_vec: Vec<Pos2> = vec![]; // = s.wm.temp_positions.clone();
            for (posx, posy) in e.wm.temp_positions.clone().into_iter().chain(v.into_iter()) {
                draw_vec.push(Pos2::new(posx, posy))
            }

            ctx.layer_painter(layer_id.unwrap()).add(Shape::line(
                draw_vec,
                Stroke {
                    width: e.scale * 1.5f32,
                    color: Color32::BLACK,
                },
            ));
        }
    }
}

/// This will only occasionally split the wire into two parts in case needed (e.g. drawing a straight
/// line)
pub fn wire_split_into_two_vec(prev: (f32, f32), current: (f32, f32)) -> Vec<(f32, f32)> {
    let mut v = vec![];
    if f32::abs(current.0 - prev.0) < 0.1f32 && f32::abs(current.1 - prev.1) < 0.01f32 {
        v.push(current);
    } else {
        let (wire1, wire2) = wire_split_into_two(prev, current);
        v.push(wire1);
        v.push(wire2);
    }
    v
}

pub fn wire_split_into_two(prev: (f32, f32), current: (f32, f32)) -> ((f32, f32), (f32, f32)) {
    if f32::abs(prev.0 - current.0) < f32::abs(prev.1 - current.1) {
        ((current.0, prev.1), (current.0, current.1))
    } else {
        ((prev.0, current.1), (current.0, current.1))
    }
}

pub fn reset_wire_mode(wm: &mut WireMode) {
    wm.mode_ended = true;
    wm.last_pos = None;
    wm.input = None;
    wm.cursor_location = Pos2::ZERO;
    wm.start_comp_port = None;
    wm.temp_positions = vec![];
}

/// returns an input made from the output
pub fn get_outputs_from_port(
    port_id_start: &Id,
    comp_start: &Rc<dyn EguiComponent>,
    port_id_end: &Id,
    comp_end: &Rc<dyn EguiComponent>,
) -> (Id, Option<Input>, bool) {
    let (id, ports_start) = comp_start.get_id_ports();
    for port_id in ports_start.outputs {
        if port_id == *port_id_start {
            return (
                port_id_end.clone(),
                Some(Input { id, field: port_id }),
                false,
            );
        }
    }
    let (id, ports_end) = comp_end.get_id_ports();
    for port_id in ports_end.outputs {
        if port_id == *port_id_end {
            return (
                port_id_start.clone(),
                Some(Input { id, field: port_id }),
                true,
            );
        }
    }
    (String::new(), None, false)
}

/// returns (Component, Wire)
pub fn clicked_close_to_input_output(
    clicked_pos: Pos2,
    components: &Components,
) -> (Option<CloseToComponent>, Option<CloseToComponent>) {
    let mut closest: Option<CloseToComponent> = None;
    let mut closest_wire: Option<CloseToComponent> = None;
    for comp in components {
        let ports = comp.ports_location();
        let prio = comp.snap_priority();
        let clos: &mut Option<CloseToComponent> = match prio {
            SnapPriority::Wire => &mut closest_wire,
            _ => &mut closest,
        };
        for (port_id, pos) in ports {
            match clos.as_ref() {
                Some(c) => {
                    let dist = clicked_pos.distance(pos);
                    if dist < c.dist {
                        *clos = Some(CloseToComponent {
                            comp: comp.clone(),
                            pos,
                            dist,
                            port_id,
                        })
                    }
                }
                None => {
                    *clos = Some(CloseToComponent {
                        comp: comp.clone(),
                        pos,
                        dist: clicked_pos.distance(pos),
                        port_id,
                    })
                }
            };
        }
    }

    (closest, closest_wire)
}
