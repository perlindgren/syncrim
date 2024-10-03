use crate::common::{EguiComponent, Input, Ports, SignalUnsigned, Simulator};
use crate::components::{Mux, MUX_OUT_ID};
use crate::gui_egui::component_ui::{
    drag_logic, input_change_id, input_selector, input_selector_removeable, pos_drag_value,
    properties_window, rect_with_hover, visualize_ports,
};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::offset_helper;
use egui::{Color32, Pos2, Rect, Response, Shape, Stroke, Ui, Vec2};
use log::trace;

#[typetag::serde]
impl EguiComponent for Mux {
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
            .map_or(false, |sim| sim.is_active(&self.id));

        trace!("render mux {}, active {}", self.id, is_active);

        // 41x(20*ports + 11)
        // middle: 21x ((20*ports + 10)/2+1)y (0 0)
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let offset_old = offset;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        let pa = self.m_in.len() as f32;

        // selector, here we can treat Signal better (see Vizia counterpart)
        let select: SignalUnsigned = match simulator {
            Some(s) => s.get_input_value(&self.select).try_into().unwrap_or(0),
            None => 0,
        };
        let mut shape: Vec<(f32, f32)> = vec![
            (-20f32, pa * (-10f32) - 10f32),
            (0f32, pa * (-10f32) - 10f32),
            (10f32, pa * (-10f32) + 10f32),
            (10f32, pa * (10f32) - 10f32),
            (0f32, pa * (10f32) + 10f32),
            (-20f32, pa * (10f32) + 10f32),
        ];
        for (x, y) in shape.iter_mut() {
            *x *= self.scale;
            *y *= self.scale;
        }

        // The shape
        ui.painter().add(Shape::closed_line(
            shape.clone().iter().map(|point| oh(*point, s, o)).collect(),
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));

        // select line
        ui.painter().add(Shape::line_segment(
            [
                oh(
                    (
                        -20f32 * self.scale,
                        (((select as f32) * 20f32) - pa * 10f32 + 10f32) * self.scale,
                    ),
                    s,
                    o,
                ),
                oh((10f32 * self.scale, 0f32 * self.scale), s, o),
            ],
            Stroke {
                width: scale,
                color: if is_active {
                    Color32::RED
                } else {
                    Color32::BLACK
                },
            },
        ));

        let rect = Rect {
            min: oh(
                (-20f32 * self.scale, (pa * (-10f32) - 10f32) * self.scale),
                s,
                o,
            ),
            max: oh(
                (10f32 * self.scale, (pa * 10f32 + 10f32) * self.scale),
                s,
                o,
            ),
        };
        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("Mux");
        });
        match editor_mode {
            EditorMode::Simulator => (),
            _ => visualize_ports(ui, self.ports_location(), offset_old, scale, clip_rect),
        }
        Some(vec![r])
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
        grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let r_vec = Mux::render(
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
        let resp = &r_vec[0];
        let delete = drag_logic(
            ui.ctx(),
            resp,
            &mut self.pos,
            &mut context.pos_tmp,
            scale,
            offset,
            grid,
        );

        properties_window(
            ui,
            self.id.clone(),
            resp,
            &mut context.properties_window,
            |ui| {
                let mut clicked_dropdown = false;
                input_change_id(ui, &mut context.id_tmp, &mut self.id, id_ports);
                pos_drag_value(ui, &mut self.pos);
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.select,
                    crate::components::MUX_SELECT_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                let mut i = 0;
                //for i in 0..=self.m_in.len() - 1 {
                self.m_in.retain_mut(|inp| {
                    let (clicked, delete) = input_selector_removeable(
                        ui,
                        inp,
                        format!("{}{}", crate::components::MUX_TEMPLATE_ID, i),
                        id_ports,
                        self.id.clone(),
                        i != 0,
                    );
                    i += 1;
                    clicked_dropdown |= clicked;
                    !delete
                });
                if ui.button("+ Add new input").clicked() {
                    self.m_in.push(Input {
                        id: "id".to_string(),
                        field: "field".to_string(),
                    });
                }
                clicked_dropdown
            },
        );

        EditorRenderReturn {
            delete,
            resp: Some(r_vec),
        }
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();
        if let Some(i) = self.m_in.iter().position(|item| item == &id) {
            Some(loc[i + 1])
        } else if id == self.select {
            Some(loc[0])
        } else if id == Input::new(&self.id, MUX_OUT_ID) {
            Some(*loc.last().unwrap())
        } else {
            None
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        let pa = self.m_in.len() as f32;
        let top = -pa * 10f32 - 10f32;
        let mut v = vec![(
            crate::components::MUX_SELECT_ID.to_string(),
            Pos2::new(-10f32, top) * self.scale + own_pos,
        )];
        for i in 0..=self.m_in.len() - 1 {
            v.push((
                format!("{}{}", crate::components::MUX_TEMPLATE_ID, i),
                Pos2::new(-20f32, top + i as f32 * 20f32 + 20f32) * self.scale + own_pos,
            ));
        }
        v.push((
            crate::components::MUX_OUT_ID.to_string(),
            Pos2::new(10f32, 0f32) * self.scale + own_pos,
        ));
        v
    }

    fn top_padding(&self) -> f32 {
        self.m_in.len() as f32 * 10f32 + 5f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
