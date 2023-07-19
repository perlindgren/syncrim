use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, Output, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct Sext {
    pub id: String,
    pub pos: (f32, f32),

    pub data_i: Input,

}

#[typetag::serde()]
impl Component for Sext {
    fn to_(&self) {
        println!("Sext");
    }
    fn to_string(&self)->String{"".to_string()}
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
    fn evaluate(&self, simulator: &mut Simulator) {
        //data is zero extended as default since its a 32 bit signal 
        let mut data = simulator.get_input_val(&self.data_i);
        //println!("SEDATA:{:b}", data);
        //println!("Sign extending");
        if(data>>20 == 1){
            let mask:u32 = 0b11111111111<<21;
            data = data|mask;
            //println!("sign was one, data:{:b}", data);
        }
        simulator.set_out_val(&self.id, "out", data);
    }
}
