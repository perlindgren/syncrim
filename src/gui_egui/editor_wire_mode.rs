use crate::common::{Components, Id, Input};
use crate::components::Wire;
use crate::gui_egui::editor::{get_component, CloseToComponent, Editor, SnapPriority};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::{
    id_ports_of_all_components, offset_helper, offset_reverse_helper, offset_reverse_helper_pos2,
    unique_component_name,
};
use egui::{
    Color32, Context, CursorIcon, LayerId, PointerButton, Pos2, Rect, Response, Shape, Stroke, Vec2,
};
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
    ctx.input_mut(|i| {
        let origin = i.pointer.press_origin().unwrap();
        e.wm.cursor_location = origin;

        let offset_cursor_scale = offset_reverse_helper_pos2(origin, e.scale, e.offset_and_pan);
        let (closest, closest_wire) =
            clicked_close_to_input_output(offset_cursor_scale, &e.components);

        match get_closest_component_non_wire_prio(closest, closest_wire, 10f32) {
            Some(closest_uw) => {
                if e.wm.temp_positions.is_empty() {
                    // New Component
                    e.wm.mode_ended = false;
                    let new_pos = closest_uw.pos;
                    e.wm.start_comp_port = Some(closest_uw);
                    let new_pos = offset_helper((new_pos.x, new_pos.y), e.scale, e.offset_and_pan);
                    e.wm.temp_positions.push((new_pos.x, new_pos.y));
                } else {
                    // Continue existing component
                    last_click(e, closest_uw);
                }
            }
            None => {
                if !e.wm.temp_positions.is_empty() {
                    let mut wires = wire_split_into_two_vec(
                        *e.wm.temp_positions.last().unwrap(),
                        (origin.x, origin.y),
                    );
                    e.wm.temp_positions.append(&mut wires)
                }
            }
        }
    });
}

pub fn last_click(e: &mut Editor, closest_uw: CloseToComponent) {
    let in_c = e.wm.start_comp_port.take().unwrap();
    let out_c = closest_uw;
    let (field_name, input_and_bool) = get_outputs_from_port(&in_c, &out_c);
    match input_and_bool {
        Some((i, is_input_in_comp_start)) => {
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

            // Now actually set the input of the wired component
            #[allow(clippy::vtable_address_comparisons)]
            if !Rc::ptr_eq(&in_c.comp, &out_c.comp) {
                let comp = if is_input_in_comp_start { out_c } else { in_c };
                println!("comp: {:?}", comp.port_id);
                e.components.push(Rc::new(Wire {
                    id: id.to_string(),
                    pos: pos_v,
                    input: i.clone(),
                }));
                e.contexts.insert(
                    id.to_string(),
                    EguiExtra {
                        properties_window: false,
                        id_tmp: id.to_string(),
                        size_rect: Rect::NAN,
                    },
                );

                if let Some(c) = get_component(&e.components, comp) {
                    println!("setting id_port");

                    Rc::get_mut(&mut e.components[c])
                        .unwrap()
                        .set_id_port(field_name, i);
                }
            } else {
                println!("You cannot connect a wire to itself");
            }
        }
        None => {
            println!("Seems like you don't exactly have one input at the start or end of the wire");
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
            // abort wire mode
            reset_wire_mode(&mut e.wm);
        }

        if !e.wm.mode_ended {
            ctx.input_mut(|i| {
                e.wm.cursor_location += i.pointer.delta();
            });
            let offset_cursor_scale =
                offset_reverse_helper_pos2(e.wm.cursor_location, e.scale, e.offset_and_pan);
            let (closest, closest_wire) =
                clicked_close_to_input_output(offset_cursor_scale, &e.components);

            // Here we prioritize component ports over wires in a 10f32 range
            let wire_shown_location = get_location_of_port_or_wire_inside_radius(
                closest,
                closest_wire,
                10f32,
                e.wm.cursor_location,
                e.offset_and_pan,
            );

            let v = wire_split_into_two_vec(
                *e.wm.temp_positions.last().unwrap(),
                (wire_shown_location.x, wire_shown_location.y),
            );
            let mut draw_vec: Vec<Pos2> = vec![];
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
        } else {
            // todo: Marker that you are potientially close to connecting to something to start a wire
            // here, like a circle or similar over the port
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
    comp_start: &CloseToComponent,
    comp_end: &CloseToComponent,
) -> (Id, Option<(Input, bool)>) {
    let out_start: Option<Input> = match &comp_start.potential_actual_input {
        Some(i) => Some(i.clone()),
        None => match comp_start.potential_actual_input.clone() {
            Some(s) => Some(s),
            None => {
                let mut o = None;
                let (id, ports_start) = comp_start.comp.get_id_ports();
                for port_id in ports_start.outputs {
                    if port_id == *comp_start.port_id {
                        o = Some(Input {
                            id: id.clone(),
                            field: port_id,
                        });
                    }
                }
                o
            }
        },
    };
    let out_end: Option<Input> = match &comp_end.potential_actual_input {
        Some(i) => Some(i.clone()),
        None => match comp_end.potential_actual_input.clone() {
            Some(s) => Some(s),
            None => {
                let mut o = None;
                let (id, ports_end) = comp_end.comp.get_id_ports();
                for port_id in ports_end.outputs {
                    if port_id == *comp_end.port_id {
                        o = Some(Input {
                            id: id.clone(),
                            field: port_id,
                        });
                    }
                }
                o
            }
        },
    };
    match out_start {
        Some(s) => match out_end {
            Some(_) => (String::new(), None),
            None => (comp_end.port_id.clone(), Some((s, true))),
        },
        None => match out_end {
            Some(e) => (comp_start.port_id.clone(), Some((e, false))),
            None => (String::new(), None),
        },
    }
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
            let dist = clicked_pos.distance(pos);
            let potential_actual_input = match comp.snap_priority() {
                SnapPriority::Wire => Some(comp.get_id_ports().1.inputs[0].input.clone()),
                _ => None,
            };
            match clos.as_ref() {
                Some(c) => {
                    if dist < c.dist {
                        *clos = Some(CloseToComponent {
                            comp: comp.clone(),
                            pos,
                            dist,
                            port_id,
                            potential_actual_input,
                        })
                    }
                }
                None => {
                    *clos = Some(CloseToComponent {
                        comp: comp.clone(),
                        pos,
                        dist,
                        port_id,
                        potential_actual_input,
                    })
                }
            };
        }
    }

    (closest, closest_wire)
}

pub fn get_closest_component_non_wire_prio(
    port: Option<CloseToComponent>,
    wire: Option<CloseToComponent>,
    distance: f32,
) -> Option<CloseToComponent> {
    fn is_wire_in_range(wire: Option<CloseToComponent>, distance: f32) -> Option<CloseToComponent> {
        match wire {
            Some(w) => {
                if w.dist <= distance {
                    Some(w)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    match port {
        Some(p) => {
            if p.dist <= distance {
                Some(p)
            } else {
                is_wire_in_range(wire, distance)
            }
        }
        None => None,
    }
}

pub fn get_location_of_port_or_wire_inside_radius(
    port: Option<CloseToComponent>,
    wire: Option<CloseToComponent>,
    distance: f32,
    cursor_location: Pos2,
    offset: Vec2,
) -> Pos2 {
    match get_closest_component_non_wire_prio(port, wire, distance) {
        Some(c) => c.pos + offset,
        None => cursor_location,
    }
}
