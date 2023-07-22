use crate::common::{Components, EguiComponent, Id, Input, SnapPriority};
use crate::components::Wire;
use crate::gui_egui::editor::{CloseToComponent, Editor};
use crate::gui_egui::helper::{
    offset_helper, offset_reverse_helper, offset_reverse_helper_pos2, unique_component_name,
};
use egui::{Color32, Context, CursorIcon, LayerId, PointerButton, Pos2, Response, Shape, Stroke};
use std::{cell::RefCell, rc::Rc};

pub fn drag_started(ctx: &Context, e: &mut Editor, cpr: Response) {
    // todo: Snap functionality should also apply here
    e.wire_mode_ended = false;
    ctx.input_mut(|i| {
        let origin = i.pointer.press_origin().unwrap();
        e.wire_cursor_location = origin;

        let offset_cursor_scale = offset_reverse_helper_pos2(origin, e.scale, e.offset);
        let (closest, closest_wire) =
            clicked_close_to_input_output(offset_cursor_scale, &e.component_store.store);
        let closest_uw = closest.clone().unwrap();

        // First click ALWAYS has to start at a port so we force it
        if e.wire_temp_positions.len() == 0 {
            // requires at least one component on the canvas
            e.wire_start_comp_port = closest;
            let new_pos = closest_uw.pos;
            let new_pos = offset_helper((new_pos.x, new_pos.y), e.scale, e.offset_and_pan);
            e.wire_temp_positions.push((new_pos.x, new_pos.y));
            // We clicked close to a port so this will be the last click done
        } else if e.wire_temp_positions.len() > 0 && closest_uw.dist <= 10.0f32 {
            // We should finish the component
            last_click(e, closest_uw);
            // Click somewhere not near a component
        } else {
            let (wire1, wire2) =
                wire_split_into_two(*e.wire_temp_positions.last().unwrap(), (origin.x, origin.y));
            e.wire_temp_positions.push(wire1);
            e.wire_temp_positions.push(wire2);
            //e.wire_temp_positions.push((origin.x, origin.y));
        }
    });
}

pub fn last_click(e: &mut Editor, closest_uw: CloseToComponent) {
    let in_c = e.wire_start_comp_port.as_ref().unwrap();
    let out_c = closest_uw;
    let input = get_input_from_port(&in_c.port, &in_c.comp, &out_c.port, &out_c.comp);
    match input {
        Some(i) => {
            println!("Creating wire component");
            let id = unique_component_name(&e.component_store.store, "w");
            let mut pos_v: Vec<(f32, f32)> = vec![];

            for pos in &e.wire_temp_positions {
                let pos_2 = offset_reverse_helper(*pos, e.scale, e.offset_and_pan);
                pos_v.push((pos_2.x, pos_2.y));
            }

            let last_pos = *e.wire_temp_positions.last().unwrap();
            let last_pos = offset_reverse_helper(last_pos, e.scale, e.offset_and_pan);
            let mut v =
                wire_split_into_two_vec((last_pos.x, last_pos.y), (out_c.pos.x, out_c.pos.y));
            pos_v.append(&mut v);

            e.component_store
                .store
                .push(Rc::new(RefCell::new(Wire::new(id, pos_v, i))));
        }
        None => {
            println!("Seems like you don't have an input at the start or end of the wire");
        }
    }
    reset_wire_mode(e);
}

pub fn wire_mode(ctx: &Context, e: &mut Editor, cpr: Response, layer_id: Option<LayerId>) {
    ctx.output_mut(|o| o.cursor_icon = CursorIcon::Crosshair);

    if cpr.drag_started_by(PointerButton::Primary) {
        drag_started(ctx, e, cpr);
    } else {
        if cpr.drag_started_by(PointerButton::Secondary) {
            // place wire end
            // This should also occur when pressing an input/output after the first one
            reset_wire_mode(e);
        }

        if !e.wire_mode_ended {
            ctx.input_mut(|i| {
                e.wire_cursor_location += i.pointer.delta();
            });
            let offset_cursor_scale =
                offset_reverse_helper_pos2(e.wire_cursor_location, e.scale, e.offset_and_pan);
            let (closest, closest_wire) =
                clicked_close_to_input_output(offset_cursor_scale, &e.component_store.store);

            let wire_shown_location = match closest {
                Some(c) => {
                    if c.dist <= 10.0f32 {
                        // We are close enough to move the shown wire to here instead
                        c.pos + e.offset_and_pan
                    } else {
                        e.wire_cursor_location
                    }
                }
                None => e.wire_cursor_location,
            };

            let v = wire_split_into_two_vec(
                *e.wire_temp_positions.last().unwrap(),
                (wire_shown_location.x, wire_shown_location.y),
            );
            let mut draw_vec: Vec<Pos2> = vec![]; // = s.wire_temp_positions.clone();
            for (posx, posy) in e
                .wire_temp_positions
                .clone()
                .into_iter()
                .chain(v.into_iter())
            {
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
    if f32::abs(prev.0 - current.0) > f32::abs(prev.1 - current.1) {
        ((current.0, prev.1), (current.0, current.1))
    } else {
        ((prev.0, current.1), (current.0, current.1))
    }
}

pub fn reset_wire_mode(e: &mut Editor) {
    e.wire_mode_ended = true;
    e.wire_last_pos = None;
    e.wire_input = None;
    e.wire_cursor_location = Pos2::ZERO;
    e.wire_start_comp_port = None;
    e.wire_temp_positions = vec![];
}

pub fn get_input_from_port(
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

/// returns (Component, Wire)
pub fn clicked_close_to_input_output(
    clicked_pos: Pos2,
    components: &Components,
) -> (Option<CloseToComponent>, Option<CloseToComponent>) {
    let mut closest: Option<CloseToComponent> = None;
    let mut closest_wire: Option<CloseToComponent> = None;
    for comp in components {
        let ports = comp.borrow_mut().ports_location();
        let prio = comp.borrow_mut().snap_priority();
        let clos: &mut Option<CloseToComponent> = match prio {
            SnapPriority::Wire => &mut closest_wire,
            _ => &mut closest,
        };
        for (port, pos) in ports {
            match clos.as_ref() {
                Some(c) => {
                    let dist = clicked_pos.distance(pos);
                    if dist < c.dist {
                        *clos = Some(CloseToComponent {
                            comp: comp.clone(),
                            pos,
                            dist,
                            port,
                        })
                    }
                }
                None => {
                    *clos = Some(CloseToComponent {
                        comp: comp.clone(),
                        pos,
                        dist: clicked_pos.distance(pos),
                        port,
                    })
                }
            };
        }
    }

    (closest, closest_wire)
}
