use crate::components::{MipsIO, IO_DATA_OUT_ID, IO_INTERRUPT_OUT_ID};
use egui::{
    pos2, Button, Color32, Event, Pos2, Rect, Response, RichText, Ui, Vec2, ViewportBuilder,
    ViewportId,
};
use syncrim::common::{EguiComponent, Id, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::{
    basic_component_gui_with_on_hover, basic_editor_popup, basic_on_hover,
};

const WIDTH: f32 = 45.0;
const HEIGHT: f32 = 40.0;

// this will break if the simulator is asynchronous
// as we cant assume cycle is the same as when this function was invoked
// when we are writing data
// to fix this we would need to pass along simulator
// lock mips_io.data  get simulator cycle
// this can also fail, hmm
// we need a chanel, that io reads during its clock so that data is clearly related to that evaluation
// why am i thinking about this
// its not relevant yet, if ever
fn render_window(mips_io: &MipsIO, ui: &mut Ui, cycle: usize) {
    ui.ctx().show_viewport_immediate(
        ViewportId::from_hash_of(&mips_io.id),
        ViewportBuilder::default().with_title("IO component"),
        |ctx, _class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut data = mips_io.data.borrow_mut();

                // for debug purpose
                // show when data will be available
                ui.monospace(format!(
                    "input data:\n{}",
                    data.key_buff_write_history
                        .iter()
                        .fold((0usize, String::new()), |mut acc, (cycle, count)| {
                            let written = data.key_buff.get(acc.0..acc.0 + count).unwrap();
                            acc.1 = format!(
                                "{} cycle {}: {}\n",
                                acc.1,
                                cycle + 1,
                                String::from_utf8_lossy(written).escape_default()
                            );
                            acc.0 += count;
                            acc
                        })
                        .1
                ));
                let future_data_exist = data.key_buff_write_history.iter().any(|(c, _)| *c > cycle);
                let upcoming_data_exist =
                    data.key_buff_write_history.iter().any(|(c, _)| *c == cycle);

                if future_data_exist {
                    ui.label(
                        RichText::new(
                            "Future data exist input is disabled. To enable clear future data",
                        )
                        .color(Color32::RED),
                    );
                    let btn = Button::new("Clear future data");
                    if ui.add(btn).clicked() {
                        data.key_buff_write_history = data
                            .key_buff_write_history
                            .iter()
                            .filter(|(c, _)| *c <= cycle)
                            .map(|i| i.clone())
                            .collect();
                        let end =
                            data.end_pos + data.key_buff_write_history.last().unwrap_or(&(0, 0)).1;
                        data.key_buff.truncate(end);
                    }
                } else if upcoming_data_exist {
                    if ui.button("clear upcoming data").clicked() {
                        data.key_buff_write_history = data
                            .key_buff_write_history
                            .iter()
                            .filter(|(c, _)| *c < cycle)
                            .map(|i| i.clone())
                            .collect();
                        let end = data.end_pos;
                        data.key_buff.truncate(end);
                    }
                }

                ui.separator();
                // render our outbuf as a string
                // TODO care about carriage return, backspace and other advance utf8/ "ascii" codes
                ui.monospace(String::from_utf8_lossy(&data.out_buff));
                if !future_data_exist {
                    let mut write_count = 0;

                    // get events and loop over them
                    // events such as keys and text input
                    for text_in in ui.input(|ev| {
                        ev.events
                            .iter()
                            .filter_map(|e| match e {
                                Event::Text(s) => Some(s.clone()),
                                Event::Key { .. } => None, // TODO handle special keys, enter, arrows, backspace. maybe as ansi codes https://en.wikipedia.org/wiki/ANSI_escape_code#Terminal_input_sequences
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                    }) {
                        // for each event

                        // write data to our key buff
                        for b in text_in.as_bytes() {
                            write_count += 1;
                            data.key_buff.push(*b);
                        }
                    }

                    // update how many bytes were written this cycle
                    if let Some(count) = data
                        .key_buff_write_history
                        .last_mut()
                        .and_then(|(c, count)| if *c == cycle { Some(count) } else { None })
                    {
                        *count += write_count
                    } else {
                        // if we dont have an entry on how much we have written this cycle create one
                        // only if we acutely did write something
                        if write_count > 0 {
                            data.key_buff_write_history.push((cycle, write_count));
                        }
                    };
                }
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                *mips_io.gui_show.borrow_mut() = false;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            };
        },
    );
}

#[typetag::serde]
impl EguiComponent for MipsIO {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        basic_component_gui_with_on_hover(
            self,
            ui.ctx(),
            offset,
            scale,
            clip_rect,
            |ui| {
                ui.set_height(HEIGHT * scale);
                ui.set_width(WIDTH * scale);

                // because borrow of gui show happens later
                let show = *self.gui_show.borrow();

                ui.label("I/O");
                ui.toggle_value(
                    &mut self.gui_show.borrow_mut(),
                    if show { "Close" } else { "Show" },
                );
                if *self.gui_show.borrow() {
                    render_window(self, ui, simulator.as_ref().map_or(0, |sim| sim.cycle));
                }
            },
            |ui| {
                let data = self.data.borrow();
                let next_byte = data.key_buff.get(data.read_pos);
                ui.label(format!(
                    "Flags {:#04b}\nNext data {}",
                    data.input_control,
                    if let Some(byte) = next_byte {
                        format!("{:#02x} ({})", byte, String::from_utf8_lossy(&[*byte]))
                    } else {
                        "none".to_string()
                    }
                ));
                ui.separator();
                basic_on_hover(ui, self, &simulator)
            },
        )
    }

    fn render_editor(
        &mut self,
        ui: &mut egui::Ui,
        context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
        id_ports: &[(Id, Ports)],
        _grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let res = self
            .render(
                ui,
                context,
                simulator,
                offset,
                scale,
                clip_rect,
                editor_mode,
            )
            .unwrap()
            .remove(0); // no panic since we know basic_component_gui returns Some([area_response])
        basic_editor_popup(self, ui, context, id_ports, res, |_| {})
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    fn top_padding(&self) -> f32 {
        20f32
    }

    fn ports_location(&self) -> Vec<(syncrim::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        // margin of component
        const M: f32 = 6.0;
        vec![
            (
                crate::components::IO_REGISTER_SELECT_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, -4.0) + own_pos,
            ),
            (
                crate::components::IO_DATA_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, 4.0) + own_pos,
            ),
            (
                crate::components::IO_WRITE_ENABLE_IN.to_string(),
                pos2(-4.0, -HEIGHT / 2.0 - M) + own_pos,
            ),
            (
                crate::components::IO_READ_ENABLE_IN.to_string(),
                pos2(4.0, -HEIGHT / 2.0 - M) + own_pos,
            ),
            (
                crate::components::IO_DATA_OUT_ID.to_string(),
                pos2(WIDTH / 2.0 + M, -4.0) + own_pos,
            ),
            (
                crate::components::IO_INTERRUPT_OUT_ID.to_string(),
                pos2(WIDTH / 2.0 + M, 4.0) + own_pos,
            ),
        ]
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();

        if id == self.address_in {
            Some(loc[0])
        } else if id == self.data_in {
            Some(loc[1])
        } else if id == self.we_in {
            Some(loc[2])
        } else if id == self.re_in {
            Some(loc[3])
        } else if id == Input::new(&self.id, IO_DATA_OUT_ID) {
            Some(loc[4])
        } else if id == Input::new(&self.id, IO_INTERRUPT_OUT_ID) {
            Some(loc[5])
        } else {
            None
        }
    }
}
