use crate::common::{EguiComponent, Input, Ports, SignalUnsigned, Simulator};
use crate::components::Mux;
use crate::gui_egui::component_ui::{
    input_change_id, input_selector, input_selector_removeable, pos_drag_value, properties_window,
    rect_with_hover,
};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::offset_helper;
use egui::{Color32, PointerButton, Pos2, Rect, Response, Shape, Stroke, Ui, Vec2};

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
        // 41x(20*ports + 11)
        // middle: 21x ((20*ports + 10)/2+1)y (0 0)
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        let pa = self.m_in.len() as f32;

        // selector, here we can treat Signal better (see Vizia counterpart)
        let select: SignalUnsigned = match simulator {
            Some(s) => s.get_input_value(&self.select).try_into().unwrap(),
            None => 0,
        };

        // The shape
        ui.painter().add(Shape::closed_line(
            vec![
                oh((-20f32, pa * (-10f32) - 10f32), s, o),
                oh((0f32, pa * (-10f32) - 10f32), s, o),
                oh((20f32, pa * (-10f32) + 10f32), s, o),
                oh((20f32, pa * (10f32) - 10f32), s, o),
                oh((0f32, pa * (10f32) + 10f32), s, o),
                oh((-20f32, pa * (10f32) + 10f32), s, o),
            ],
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));
        // select line
        ui.painter().add(Shape::line_segment(
            [
                oh(
                    (-20f32, ((select as f32) * 20f32) - pa * 10f32 + 10f32),
                    s,
                    o,
                ),
                oh((20f32, 0f32), s, o),
            ],
            Stroke {
                width: scale,
                color: Color32::RED,
            },
        ));
        let rect = Rect {
            min: oh((-20f32, pa * (-10f32) - 10f32), s, o),
            max: oh((20f32, pa * 10f32 + 10f32), s, o),
        };
        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("Mux");
        });
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
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let mut delete = false;
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
        if resp.dragged_by(PointerButton::Primary) {
            let delta = resp.drag_delta() / scale;
            self.pos = (self.pos.0 + delta.x, self.pos.1 + delta.y);
        }

        if resp.drag_released_by(PointerButton::Primary)
            && resp.interact_pointer_pos().unwrap().x < offset.x
        {
            delete = true;
        }

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

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        let pa = self.m_in.len() as f32;
        let top = -pa * 10f32 - 10f32;
        let mut v = vec![(
            crate::components::MUX_SELECT_ID.to_string(),
            Pos2::new(-10f32, top) + own_pos,
        )];
        for i in 0..=self.m_in.len() - 1 {
            v.push((
                format!("{}{}", crate::components::MUX_TEMPLATE_ID, i),
                Pos2::new(-20f32, top + i as f32 * 20f32 + 20f32) + own_pos,
            ));
        }
        v.push((
            crate::components::MUX_OUT_ID.to_string(),
            Pos2::new(20f32, 0f32) + own_pos,
        ));
        v
    }

    fn top_padding(&self) -> f32 {
        self.m_in.len() as f32 * 10f32 + 5f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }
}
