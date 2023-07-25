use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct SZExt {
    pub id: String,
    pub pos: (f32, f32),

    pub data_i: Input,
    pub sel_i:Input,

}

#[typetag::serde()]
impl Component for SZExt {
    fn to_(&self) {
        println!("s_z_ext");
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.data_i.clone(),self.sel_i.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    "out".into(),
                ],
            },
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) {
        //data is zero extended as default since its a 32 bit signal 
        let mut data:u32 = simulator.get_input_val(&self.data_i).try_into().unwrap();
        let sel:u32 = simulator.get_input_val(&self.sel_i).try_into().unwrap();
        //println!("SZEDATA:{:x}", data);
        match sel{
            0=>{
                //println!("Sign extending");
                if data>>11 == 1{
                    let mask:u32 = 0xFFFFF000;
                    data = data|mask;
                    //println!("sign was one, data:{:x}", data);
                }
            }
            1=>{}
            _=>{panic!("Invalid sel on SZExt:{}",sel)}
        }
        simulator.set_out_val(&self.id, "out", data);
    }
}
