use crate::components::{
    alu_op, ALU, ALU_A_IN_ID, ALU_B_IN_ID, ALU_OP_IN_ID, ALU_OUT_ID, ALU_OVERFLOW_OUT_ID,
};
use egui::{
    pos2, vec2, Align2, Area, Color32, Order, Pos2, Rect, Response, RichText, Shape, Stroke,
    TextWrapMode, Ui, Vec2,
};
use syncrim::common::{EguiComponent, Input, Ports, Simulator};
use syncrim::gui_egui::component_ui::{
    drag_logic, input_change_id, input_selector, pos_drag_value, properties_window,
    rect_with_hover, visualize_ports,
};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::offset_helper;
use syncrim::signal::Id;

#[typetag::serde]
impl EguiComponent for ALU {
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
        // 41x81
        // middle: 21x 41y (0 0)
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let offset_old = offset;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        // The shape
        ui.painter().add(Shape::closed_line(
            vec![
                oh((-20f32, -40f32), s, o),
                oh((0f32, -40f32), s, o),
                oh((20f32, -20f32), s, o),
                oh((20f32, 20f32), s, o),
                oh((0f32, 40f32), s, o),
                oh((-20f32, 40f32), s, o),
                oh((-20f32, 20f32), s, o),
                oh((-10f32, 0f32), s, o),
                oh((-20f32, -20f32), s, o),
            ],
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));

        let rect = Rect {
            min: oh((-20f32, -40f32), s, o),
            max: oh((20f32, 40f32), s, o),
        };
        let op: String = if let Some(s) = simulator {
            match TryInto::<u32>::try_into(s.get_input_value(&self.op_in)).unwrap() {
                alu_op::ADD => "ADD",
                alu_op::ADDU => "ADDU",
                alu_op::SUB => "SUB",
                alu_op::SUBU => "SUBU",
                alu_op::AND => "AND",
                alu_op::OR => "OR",
                alu_op::XOR => "XOR",
                alu_op::NOR => "NOR",
                alu_op::SLT => "SLT",
                alu_op::SLTU => "SLTU",
                alu_op::SLL => "SLL",
                alu_op::SRL => "SRL",
                alu_op::SRA => "SRA",
                alu_op::LUI => "LUI",
                _ => "UNDEF",
            }
            .to_string()
        } else {
            "no sim".to_string()
        };

        let _area = Area::new(egui::Id::from(self.id.to_string()))
            .order(Order::Middle)
            .current_pos(offset.to_pos2() + Vec2::new(5.0, 0.0) * scale)
            .movable(false)
            .enabled(true)
            .interactable(false)
            .pivot(Align2::CENTER_CENTER)
            .constrain(false)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(clip_rect);
                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                ui.label(RichText::new(format!("ALU\n{}", op)).size(scale * 12f32))
            });
        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("Adder");
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
        id_ports: &[(syncrim::common::Id, Ports)],
        grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let r_vec = ALU::render(
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
                    &mut self.a_in,
                    crate::components::ALU_A_IN_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.b_in,
                    crate::components::ALU_B_IN_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.op_in,
                    crate::components::ALU_OP_IN_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown
            },
        );
        EditorRenderReturn {
            delete,
            resp: Some(r_vec),
        }
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        if id == self.a_in {
            Some((Pos2::from(self.pos) + vec2(-20.0, -30.0)).into())
        } else if id == self.b_in {
            Some((Pos2::from(self.pos) + vec2(-20.0, 30.0)).into())
        } else if id == Input::new(&self.id, ALU_OUT_ID) {
            Some((Pos2::from(self.pos) + vec2(20.0, 0.0)).into())
        } else if id == Input::new(&self.id, ALU_OVERFLOW_OUT_ID) {
            Some((Pos2::from(self.pos) + vec2(20.0, 5.0)).into())
        } else if id == self.op_in {
            Some((Pos2::from(self.pos) + vec2(-10.0, -40.0)).into())
        } else {
            None
        }
    }

    fn ports_location(&self) -> Vec<(Id, egui::Pos2)> {
        let own_pos: Vec2 = self.pos.into();
        vec![
            (ALU_A_IN_ID.to_string(), pos2(-20.0, -30.0) + own_pos),
            (ALU_B_IN_ID.to_string(), pos2(-20.0, 30.0) + own_pos),
            (ALU_OP_IN_ID.to_string(), pos2(-10.0, -40.0) + own_pos),
            (ALU_OUT_ID.to_string(), pos2(20.0, 0.0) + own_pos),
            (ALU_OVERFLOW_OUT_ID.to_string(), pos2(-20.0, 5.0) + own_pos),
        ]
    }

    fn top_padding(&self) -> f32 {
        20f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
