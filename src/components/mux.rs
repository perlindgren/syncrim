use crate::common::{
    Component, Id, Input, InputPort, OutputType, Ports, Signal, SignalUnsigned, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
pub const MUX_SELECT_ID: &str = "select";
pub const MUX_TEMPLATE_ID: &str = "in";
pub const MUX_OUT_ID: &str = "out";

#[derive(Serialize, Deserialize)]
pub struct Mux {
    pub id: Id,
    pub pos: (f32, f32),
    pub select: Input,
    pub m_in: Vec<Input>,
}

#[typetag::serde]
impl Component for Mux {
    fn to_(&self) {
        trace!("mux");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        let mut inputs: Vec<InputPort> = Vec::with_capacity(self.m_in.len() + 1);
        inputs.push(InputPort {
            port_id: MUX_SELECT_ID.to_string(),
            input: self.select.clone(),
        });
        for (i, item) in self.m_in.iter().enumerate() {
            inputs.push(InputPort {
                port_id: format!("{}{}", MUX_TEMPLATE_ID, i),
                input: item.clone(),
            });
        }

        (
            self.id.clone(),
            Ports {
                inputs,
                out_type: OutputType::Combinatorial,
                outputs: vec![MUX_OUT_ID.to_string()],
            },
        )
    }

    // propagate selected input value to output
    fn clock(&self, simulator: &mut Simulator) {
        // get input value
        let select: SignalUnsigned = simulator.get_input_val(&self.select).try_into().unwrap();
        let select = select as usize;
        trace!("select {}", select);
        let value = if select < self.m_in.len() {
            simulator.get_input_val(&self.m_in[select])
        } else {
            Signal::Unknown
        };

        // set output
        simulator.set_out_val(&self.id, "out", value);
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        let target_port_id = target_port_id.as_str();
        if target_port_id == MUX_SELECT_ID {
            self.select = new_input;
            return;
        }
        for i in 0..=self.m_in.len() - 1 {
            if target_port_id == format!("{}{}", MUX_TEMPLATE_ID, i) {
                self.m_in[i] = new_input;
                return;
            }
        }
    }
}
