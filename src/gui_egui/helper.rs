use crate::common::{Components, EguiComponent, Input, Ports, SignalValue, Simulator};
use crate::gui_egui::component_ui::input_selector;
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, SnapPriority};
use crate::gui_egui::EguiExtra;
use egui::{
    Align2, Area, Color32, Context, InnerResponse, Order, Pos2, Rect, Response, Sense,
    TextWrapMode, Ui, Vec2, Window,
};
use epaint::Shadow;

pub fn offset_reverse_helper_pos2(xy: Pos2, scale: f32, offset: Vec2) -> Pos2 {
    egui::Pos2 {
        x: (xy.x - offset.x) / scale,
        y: (xy.y - offset.y) / scale,
    }
}

pub fn offset_reverse_helper(xy: (f32, f32), scale: f32, offset: Vec2) -> Pos2 {
    egui::Pos2 {
        x: (xy.0 - offset.x) / scale,
        y: (xy.1 - offset.y) / scale,
    }
}

pub fn offset_helper_pos2(xy: Pos2, scale: f32, offset: Vec2) -> Pos2 {
    egui::Pos2 {
        x: xy.x * scale,
        y: xy.y * scale,
    } + offset
}

pub fn offset_helper(xy: (f32, f32), scale: f32, offset: Vec2) -> Pos2 {
    egui::Pos2 {
        x: xy.0 * scale,
        y: xy.1 * scale,
    } + offset
}

pub fn out_of_bounds(request: Rect, clip_rect: Rect) -> Rect {
    let mut rect = Rect::NOTHING;
    if request.max.x < clip_rect.min.x
        || request.max.y < clip_rect.min.y
        || request.min.x > clip_rect.max.x
        || request.min.y > clip_rect.max.y
    {
        return rect;
    }
    rect = request;
    if request.max.x > clip_rect.max.x {
        rect.max.x = clip_rect.max.x;
    }
    if request.max.y > clip_rect.max.y {
        rect.max.y = clip_rect.max.y;
    }
    if request.min.x < clip_rect.min.x {
        rect.min.x = clip_rect.min.x;
    }
    if request.min.y < clip_rect.min.y {
        rect.min.y = clip_rect.min.y;
    }
    rect
}

pub fn unique_component_name(id_ports: &[(crate::common::Id, Ports)], id: &str) -> String {
    let mut new_id: String = id.into();
    let mut contains_id = true;
    while contains_id {
        contains_id = false;
        for c in id_ports {
            let id = c.0.clone();
            if id == new_id {
                contains_id = true;
                // todo: make this fancier
                new_id.push('1');
                break;
            }
        }
    }
    new_id
}

pub fn id_ports_of_all_components(cs: &Components) -> Vec<(crate::common::Id, Ports)> {
    let mut v = vec![];
    for c in cs.iter() {
        v.push(c.get_id_ports());
    }
    v
}

pub fn id_ports_of_all_components_non_wires(cs: &Components) -> Vec<(crate::common::Id, Ports)> {
    let mut v = vec![];
    for c in cs.iter() {
        match c.snap_priority() {
            SnapPriority::Wire => (),
            _ => v.push(c.get_id_ports()),
        }
    }
    v
}

pub fn editor_mode_to_sense(editor_mode: EditorMode) -> Sense {
    match editor_mode {
        EditorMode::Wire => Sense::hover(),
        _ => Sense::all(),
    }
}

pub fn shadow_small_dark() -> Shadow {
    Shadow {
        offset: [5, 5],
        blur: 5,
        spread: 0,
        color: Color32::BLACK,
    }
}

pub fn component_area<R>(
    id: String,
    ctx: &Context,
    pos: impl Into<Pos2>,
    content: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    Area::new(egui::Id::from(id))
        .order(Order::Middle)
        .current_pos(pos)
        .movable(false)
        .enabled(true)
        .interactable(true)
        .sense(Sense::all())
        .pivot(Align2::CENTER_CENTER)
        .constrain(false)
        .show(ctx, content)
}

/// This renders a basic component
/// Use Content to add label or other graphical info
/// and if desired implement a custom on hover function.
/// The default hover function displays component id and  in/out signals formatet as hex
///
/// # Arguments
/// - size: if size is (0f32,0f32) the component will be as large as its content
/// - content: Note the function don't size the components,
///   that is the responsibility of the content closure
/// - on_hover: if this is some this overides the on hover function and displays that instead
///
/// # Example
/// This renders a box with the size of 100 by 20, this is scaled with the passed scaled.
/// It is also moved according to the offset argument.
///
/// In the box the label "Jump Merge" is displayed.
///
/// And an possible default on hover label might be
///
/// id: jump_merge
///
/// merge_instr_addr_in <- reg:out (0x00000000)
///
/// merge_jump_addr_in <- c1:out (0x00000000)
///
/// out-> 0x00000000
///  
/// ```
/// # use std::any::Any;
/// # use egui::{Ui, Vec2, Rect, Response};
/// # use syncrim::common::{Ports, EguiComponent, Component, Simulator};
/// # use syncrim::gui_egui::{EguiExtra, editor::EditorMode};
/// # use serde::{Deserialize, Serialize};
/// # #[derive(Serialize, Deserialize, Clone)]
/// # struct JumpMerge {tmp: u32}
/// # impl Component for JumpMerge {
/// #   fn get_id_ports(&self) -> (std::string::String, Ports) { todo!() }
/// #   fn as_any(&self) -> &(dyn Any + 'static) { todo!() }
/// #   fn typetag_name(&self) -> &'static str { todo!() }
/// #   fn typetag_deserialize(&self) { todo!() }
/// # }
///
/// use syncrim::gui_egui::helper::basic_component_gui_with_on_hover;
///
/// # #[typetag::serde]
/// impl EguiComponent for JumpMerge {
///     fn render(
///         &self,
///         ui: &mut Ui,
///         _context: &mut EguiExtra,
///         simulator: Option<&mut Simulator>,
///         offset: Vec2,
///         scale: f32,
///         clip_rect: Rect,
///         _editor_mode: EditorMode,
///     ) -> Option<Vec<Response>> {
///         // size of the component
///         let width = 100f32;
///         let height: f32 = 20f32;
///         basic_component_gui_with_on_hover(
///             self,
///             ui.ctx(),
///             offset,
///             scale,
///             clip_rect,
///             |ui| {
///                 ui.label("Jump Merge");
///             },
///             |ui| {
///                 ui.label("i am hovered");
///             },
///         )
///     }
/// }
/// ```
pub fn basic_component_gui_with_on_hover(
    component: &dyn EguiComponent,
    ctx: &Context,
    offset: impl Into<Vec2>,
    scale: f32,
    clip_rect: Rect,
    content: impl FnOnce(&mut Ui),
    on_hover: impl FnOnce(&mut Ui),
) -> Option<Vec<Response>> {
    let offset: Vec2 = offset.into();

    let r = component_area(
        component.get_id_ports().0,
        ctx,
        Pos2::from(component.get_pos()) * scale + offset,
        |ui| {
            ui.set_clip_rect(clip_rect);

            ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);

            for (_text_style, font) in ui.style_mut().text_styles.iter_mut() {
                font.size *= scale;
            }
            ui.spacing_mut().button_padding *= scale;
            ui.spacing_mut().item_spacing *= scale;
            ui.spacing_mut().combo_height *= scale;
            ui.spacing_mut().combo_width *= scale;
            ui.spacing_mut().icon_width *= scale;
            ui.spacing_mut().icon_width_inner *= scale;
            ui.spacing_mut().icon_spacing *= scale;
            ui.spacing_mut().interact_size *= scale;

            let mut group = egui::containers::Frame::group(ui.style());
            group.inner_margin *= scale;
            group.corner_radius *= scale;
            // group.fill = Color32::LIGHT_RED; // Use this ween component background is implemented, probably when we implement dark mode
            group
                .show(ui, |ui| {
                    content(ui);
                })
                .response
        },
    )
    .response;

    r.clone().on_hover_ui(on_hover);
    Some(vec![r])
}

pub fn basic_component_gui(
    component: &dyn EguiComponent,
    simulator: &Option<&mut Simulator>,
    ctx: &Context,
    offset: impl Into<Vec2>,
    scale: f32,
    clip_rect: Rect,
    content: impl FnOnce(&mut Ui),
) -> Option<Vec<Response>> {
    basic_component_gui_with_on_hover(component, ctx, offset, scale, clip_rect, content, |ui| {
        basic_on_hover(ui, component, simulator)
    })
}

/// example
/// r.on_hover(|ui| {
///     basic_on_hover(ui,self,simulator)
/// })
pub fn basic_on_hover(
    ui: &mut Ui,
    component: &dyn EguiComponent,
    simulator: &Option<&mut Simulator>,
) {
    ui.set_max_width(200.0);
    ui.label(format!("id: {}", component.get_id_ports().0));
    if let Some(sim) = simulator {
        ui.separator();
        for port in component.get_id_ports().1.inputs {
            ui.label(format!(
                "{} <- {}:{} ({})",
                port.port_id,
                port.input.id,
                port.input.field,
                match sim.get_input_value(&port.input) {
                    SignalValue::Uninitialized => "Uninitialized".to_string(),
                    SignalValue::Unknown => "Unknown".to_string(),
                    SignalValue::DontCare => "don't care".to_string(),
                    SignalValue::Data(v) => format!("{:#010x}", v),
                },
            ));
        }
        ui.separator();
        for port_id in component.get_id_ports().1.outputs {
            ui.label(format!(
                "{} -> {}",
                port_id,
                match sim.get_input_value(&Input::new(&component.get_id_ports().0, &port_id)) {
                    SignalValue::Uninitialized => "Uninitialized".to_string(),
                    SignalValue::Unknown => "Unknown".to_string(),
                    SignalValue::DontCare => "Don't care".to_string(),
                    SignalValue::Data(v) => format!("{:#010x}", v),
                },
            ));
        }
    };
}

/// basic editor popup
/// TODO test me and write documentation
pub fn basic_editor_popup(
    component: &mut dyn EguiComponent,
    ui: &mut Ui,
    context: &mut EguiExtra,
    id_ports: &[(crate::common::Id, Ports)],
    activator_resp: Response,
    extra_properties_window_stuff: impl FnOnce(&mut Ui), // TODO rename to something better
) -> EditorRenderReturn {
    // if we clicked the activator response and sould open a properties window
    let properties_window = activator_resp.clicked_by(egui::PointerButton::Secondary);
    // shul component be deleted
    let mut delete = false;

    // either clicked or windows was open last frame (from egui extra)
    if properties_window || context.properties_window {
        // dont close properties when dropdown is active
        let mut clicked_dropdown = false;

        // crete a window for our properties
        let resp = Window::new(format!("Properties: {}", component.get_id_ports().0))
            .frame(
                egui::containers::Frame::new()
                    .inner_margin(10)
                    .corner_radius(10)
                    .shadow(shadow_small_dark())
                    .fill(ui.visuals().panel_fill)
                    .stroke(ui.visuals().window_stroke),
            )
            .default_pos(Pos2::from(component.get_pos()))
            .show(ui.ctx(), |ui| {
                // TODO make it possible to change id
                // currently this is not possible as EguiComponent don't implements a way to do this

                // show user properties window stuff
                extra_properties_window_stuff(ui);

                // Change position
                let (mut x, mut y) = component.get_pos();
                ui.horizontal(|ui| {
                    ui.label("pos x");
                    ui.add(egui::DragValue::new(&mut x).speed(0.5));
                    ui.label("pos y");
                    ui.add(egui::DragValue::new(&mut y).speed(0.5));
                });
                component.set_pos((x, y));

                // get inputs to the component and lop over them to create a select input
                for port in component.get_id_ports().1.inputs {
                    // for this port create input selector and up date our port
                    ui.horizontal(|ui| {
                        let mut new_input = port.input.clone();
                        clicked_dropdown |= input_selector(
                            ui,
                            &mut new_input,
                            port.port_id.clone(),
                            id_ports,
                            component.get_id_ports().0,
                        );
                        component.set_id_port(port.port_id.clone(), new_input);
                    });
                }

                delete = ui.button("Delete Component").clicked()
            });

        // if context haven't set properties window on this obj
        // set it to true so it stays up next frame
        if !context.properties_window {
            context.properties_window = true;
        // im scared of this unwrap, can we grantee that the window is open?
        } else if !clicked_dropdown && resp.unwrap().response.clicked_elsewhere() {
            context.properties_window = false;
        };
    }
    EditorRenderReturn {
        delete,
        resp: Some(vec![activator_resp]),
    }
}
