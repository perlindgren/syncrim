use crate::components::CLIC;
use egui::{Color32, Pos2, Rect, Response, Rounding, Shape, Stroke, Ui, Vec2};
use syncrim::common::{EguiComponent, Ports, Simulator};
use syncrim::gui_egui::component_ui::{
    drag_logic, input_change_id, input_selector, pos_drag_value, properties_window,
    rect_with_hover, visualize_ports,
};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::offset_helper;

#[typetag::serde]
impl EguiComponent for CLIC {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        // 21x41
        // middle: 11x 21y (0 0)
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let offset_old = offset;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;

        // The shape
        let rect = Rect {
            min: oh((-self.width / 2f32, -self.height / 2f32), s, o),
            max: oh((self.width / 2f32, self.height / 2f32), s, o),
        };
        ui.painter().add(Shape::rect_stroke(
            rect,
            Rounding::none(),
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));

        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("CLIC");
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
        let r_vec = CLIC::render(
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
                pub const CLIC_CSR_ADDR_ID: &str = "csr_addr";
                pub const CLIC_CSR_CTL_ID: &str = "csr_ctl";
                pub const CLIC_CSR_DATA_ID: &str = "csr_data";
                pub const CLIC_DATA_ID: &str = "data";
                pub const CLIC_ADDR_ID: &str = "addr";
                pub const CLIC_DATA_WE_ID: &str = "data_we";
                pub const CLIC_MRET_ID: &str = "mret";
                pub const CLIC_PC_ID: &str = "pc";
                pub const CLIC_DATA_SIZE_ID: &str = "size";

                pub const CLIC_CSR_DATA_OUT_ID: &str = "csr_data_o";
                pub const CLIC_MMIO_DATA_OUT_ID: &str = "mmio_data_o";
                pub const CLIC_MEM_INT_ADDR_ID: &str = "mem_int_addr";
                pub const CLIC_BLU_INT_ID: &str = "blu_int";
                pub const CLIC_MRET_OUT_ID: &str = "mret_out";
                pub const CLIC_MEPC_OUT_ID: &str = "mepc_out";
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.csr_addr,
                    crate::components::CLIC_CSR_ADDR_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.csr_ctl,
                    crate::components::CLIC_CSR_CTL_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.csr_data,
                    crate::components::CLIC_CSR_DATA_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.data,
                    crate::components::CLIC_DATA_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.addr,
                    crate::components::CLIC_ADDR_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.data_we,
                    crate::components::CLIC_DATA_WE_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.mret,
                    crate::components::CLIC_MRET_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.data_size,
                    crate::components::CLIC_DATA_SIZE_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.pc,
                    crate::components::CLIC_PC_ID.to_string(),
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

    fn ports_location(&self) -> Vec<(syncrim::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![
            (
                crate::components::CLIC_CSR_DATA_OUT_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 4f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::CLIC_MMIO_DATA_OUT_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 3f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::CLIC_DATA_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 1f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::CLIC_CSR_DATA_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 2f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),

        ]
    }

    fn top_padding(&self) -> f32 {
        self.height / 4f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
