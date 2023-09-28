use syncrim::common::{Component, Condition, Id, Input, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct AntiqCtl {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) d_in: Input,
    pub(crate) addr: Input,
    pub(crate) we: Input,

    //MMIO range constants
    pub(crate) range_low: u32,
    pub(crate) range_high: u32,
}

#[typetag::serde]
impl Component for AntiqCtl {
    fn to_(&self) {
        trace!("register");
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Vector of inputs
                vec![&self.d_in, &self.addr, &self.we],
                OutputType::Combinatorial,
                vec!["push_ctl", "pop_ctl", "drop_ctl", "d_out"],
            ),
        )
    }

    // propagate input value to output
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input value
        //let value = simulator.get_input_value(&self.r_in);
        let we:u32 = simulator.get_input_value(&self.we).try_into().unwrap();
        let d_in:u32 = simulator.get_input_value(&self.d_in).try_into().unwrap();
        let addr:u32 = simulator.get_input_value(&self.addr).try_into().unwrap();
        //let sysclk:u32 = simulator.get_input_value(&self.sysclk).try_into().unwrap();
        let interrupt_id = d_in >> 24;   // shift out time data
        let time_in = d_in & 0x00FFFFFF; // mask out interrupt id
        let mut data_out = 0;
        if we == 1 && self.within_range(addr) {
            match addr - self.range_low{
                0 =>{//relative time maybe ignore this for now, needs signal to antiq to trigger
                  //adding sysclk
                       // data_out = (sysclk + time_in)<<8 | interrupt_id;
                        simulator.set_out_value(&self.id, "push_ctl", 1);
                    }
                4 =>{//absolute time
                        data_out = (time_in) << 8 | interrupt_id;
                        simulator.set_out_value(&self.id, "push_ctl", 1);
                    }
                8 =>{//drop input/push output
                        unimplemented!()
                    } 
                _=>{//do nothing, we've checked that addr is within range
                    }
            }
        }
        // set output
        simulator.set_out_value(&self.id, "d_out", data_out);
        Ok(())
    }
}

impl AntiqCtl {
    pub fn new(id: &str, pos: (f32, f32), d_in: Input, addr: Input, we: Input,start_of_range:u32) -> Self {
        AntiqCtl {
            id: id.to_string(),
            pos,
            d_in,
            addr,
            we,
            range_low: start_of_range,
            range_high: start_of_range + 4*3, //3 word wide registers
        }
    }

    fn within_range(&self, addr:u32)->bool{
        self.range_low <= addr && addr <= self.range_high
    }

   // pub fn rc_new(id: &str, pos: (f32, f32), r_in: Input) -> Rc<Self> {
   //     Rc::new(Register::new(id, pos, r_in))
   // }
}
