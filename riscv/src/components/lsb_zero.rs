use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct LSBZero {
    pub id: String,
    pub pos: (f32, f32),

    pub data_i: Input,

}

#[typetag::serde()]
impl Component for LSBZero {
    fn to_(&self) {
        println!("LSBZero");
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.data_i.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    "out".into(),
                ],
            },
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) {
        let mut data:u32 = simulator.get_input_val(&self.data_i).try_into().unwrap();
        let mask:u32 = !0b1;
        //println!("STRIPPER  IN:0b{:032b}", data);
        //println!("STRIPPERMASK:0b{:032b}", mask);
        data &= mask;
        //println!("STRIPPER OUT:0b{:032b}", data);
        simulator.set_out_val(&self.id, "out", data);
    }
}
