use crate::common::EguiComponent;
use crate::components::*;
use crate::gui_egui::editor::EditorMode;
use crate::gui_egui::{
    editor::Editor,
    helper::{id_ports_of_all_components, offset_reverse_helper_pos2, unique_component_name},
};
use egui::{Context, CursorIcon, LayerId, PointerButton, Pos2, Rect, Response, Ui, Vec2};
use std::rc::Rc;

pub struct InputMode {
    pub comp: Option<Rc<dyn EguiComponent>>,
    pub cursor_location: Pos2,
}

pub fn input_mode(ctx: &Context, e: &mut Editor, cpr: Response, layer_id: Option<LayerId>) {
    let layer_id = layer_id.unwrap();
    ctx.output_mut(|o| o.cursor_icon = CursorIcon::None);
    ctx.input_mut(|i| {
        e.im.cursor_location += i.pointer.delta();
    });
    let clip_rect = Rect {
        min: Pos2 {
            x: 0f32,
            y: e.offset.y,
        },
        max: Pos2 {
            x: f32::INFINITY,
            y: f32::INFINITY,
        },
    };
    let mut ui = Ui::new(
        ctx.to_owned(),
        layer_id,
        "input".into(),
        clip_rect,
        Rect::EVERYTHING,
    );
    e.im.comp.as_ref().unwrap().render(
        &mut ui,
        None,
        Vec2::new(e.im.cursor_location.x, e.im.cursor_location.y),
        e.scale,
        clip_rect,
        e.editor_mode,
    );

    if cpr.drag_started_by(PointerButton::Primary) {
        add_comp_to_editor(e);
    }
}

pub fn reset_input_mode(im: &mut InputMode) {
    im.comp = None;
    im.cursor_location = Pos2::ZERO;
}

pub fn show_library(e: &mut Editor, ui: &mut Ui) {
    let mut padding = Vec2 {
        x: e.offset.x / 2f32,
        y: e.offset.y + 10f32,
    };
    let clip_rect = Rect {
        min: Pos2 {
            x: 0f32,
            y: e.offset.y,
        },
        max: Pos2 {
            x: e.offset.x,
            y: f32::INFINITY,
        },
    };
    for c in e.library.store.iter() {
        let size = c.size();
        padding.y -= e.scale * size.min.y;
        let r_vec = c
            .render(ui, None, padding, e.scale, clip_rect, e.editor_mode)
            .unwrap();
        let rect = r_vec[0].rect;
        for resp in r_vec {
            // Create new component
            if resp.drag_started_by(PointerButton::Primary) {
                e.editor_mode = EditorMode::Input;
                e.im.comp = Some(c.clone());
                ui.input_mut(|i| {
                    let origin = i.pointer.press_origin().unwrap();
                    e.im.cursor_location = origin;
                });
            } else if resp.drag_started_by(PointerButton::Secondary) {
                reset_input_mode(&mut e.im);
            }
        }
        padding.y = rect.max.y + 10f32;
    }
}

// todo: This should really just copy the component that's in e.input_comp
pub fn add_comp_to_editor(e: &mut Editor) {
    let pos = offset_reverse_helper_pos2(e.im.cursor_location, e.scale, e.offset);
    let id_ports = id_ports_of_all_components(&e.components);
    let mut comp: Rc<dyn EguiComponent> =
        match e.im.comp.as_mut().unwrap().get_id_ports().0.as_str() {
            "c" => {
                let id = unique_component_name(&id_ports, "c");
                Rc::new(Constant::new(id.as_str(), (0.0, 0.0), 0))
            }
            "p" => {
                let id = unique_component_name(&id_ports, "p");
                Rc::new(Probe::new(id.as_str(), (0.0, 0.0), e.dummy_input.clone()))
            }
            "add" => {
                let id = unique_component_name(&id_ports, "add");
                Rc::new(Add::new(
                    id.as_str(),
                    (0.0, 0.0),
                    e.dummy_input.clone(),
                    e.dummy_input.clone(),
                ))
            }
            _ => todo!(),
        };
    Rc::<dyn EguiComponent>::get_mut(&mut comp)
        .unwrap()
        .set_pos((pos.x, pos.y));
    Rc::<dyn EguiComponent>::get_mut(&mut comp)
        .unwrap()
        .set_id_tmp();
    e.components.push(comp);
}
