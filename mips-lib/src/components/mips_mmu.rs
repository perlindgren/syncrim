// use std::fmt::Alignment;
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator,
};

pub const MMU_ADDRESS_IN_ID: &str = "mmu_address_signal_in";

pub const MMU_COMPONENT_SELECT_OUT_ID: &str = "component_select_out";
pub const MMU_ADDRESS_OUT_ID: &str = "address_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct MipsMmu {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) address_in: Input,
}

#[typetag::serde]
impl Component for MipsMmu {
    fn to_(&self) {
        trace!("Mips_mmu");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: MMU_ADDRESS_IN_ID.to_string(),
                        input: self.address_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![MMU_ADDRESS_OUT_ID, MMU_COMPONENT_SELECT_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            MMU_ADDRESS_IN_ID => self.address_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, _simulator: &mut Simulator) -> Result<(), Condition> {
        warn!("TODO make mmu send out select signal and modified address");
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MipsMmu {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        address_in: Input,
    ) -> Self {
        MipsMmu {
            id: id.to_string(),
            pos,
            address_in,
        }
    }
}
