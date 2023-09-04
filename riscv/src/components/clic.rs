use log::trace;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Condition, Id, Input, OutputType, Ports, Signal, Simulator};

use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::{cell::RefCell, collections::HashMap, convert::TryFrom};

#[derive(Serialize, Deserialize)]
pub struct CLIC {
    pub id: Id,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // configuration
    //pub big_endian: bool,

    // mmio ports
   // pub data: Input,
   // pub addr: Input,

    //CSR ports
    pub csr_data: Input,
    pub csr_addr: Input,
    //1 = write, 2 = set, 3 = clear
    pub csr_ctl: Input,

    //interurpt lines
   // pub lines: Vec<Input>,

    //internal state
    pub csrstore: RefCell<HashMap<usize, usize>>, //address, val
    pub mmio: RefCell<HashMap<usize, MMIOEntry>>, //address, val
    pub queue: RefCell<BTreeMap<usize, HashSet<usize>>>, //prio, id's
}
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct MMIOEntry {
    clicintip: u8,
    clicintie: u8,
    clicintattr: u8,
    clicintctl: u8,
}

impl From<u32> for MMIOEntry {
    fn from(val: u32) -> Self {
        MMIOEntry {
            clicintip: (val & 0b11111111) as u8,
            clicintie: ((val >> 8) & 0b11111111) as u8,
            clicintattr: ((val >> 16) & 0b11111111) as u8,
            clicintctl: ((val >> 24) & 0b11111111) as u8,
        }
    }
}

impl Into<u32> for MMIOEntry {
    fn into(self) -> u32 {
        (self.clicintip as u32
            | ((self.clicintie as u32) << 8) 
            | ((self.clicintattr as u32) << 16)
            | ((self.clicintctl as u32) << 24))
    }
}
impl CLIC {
    pub fn new(
        id: Id,
        pos: (f32, f32),
        width: f32,
        height: f32,
      //  data: Input,
      //  addr: Input,
        csr_data: Input,
        csr_addr: Input,
      //  lines: Vec<Input>,
        csr_ctl: Input,
    ) -> Self {
        CLIC {
            id: id,
            pos: pos,
            width: width,
            height: height,
          //  data: data,
          //  addr: addr,
            csr_data: csr_data,
            csr_addr: csr_addr,
            csrstore: {
                let mut csrstore = HashMap::new();
                csrstore.insert(0x300, 0); //mstatus
                csrstore.insert(0x305, 0); //mtvec
                csrstore.insert(0x307, 0); //mtvt
                csrstore.insert(0x340, 0); //mscratch
                csrstore.insert(0x341, 0); //mepc
                csrstore.insert(0x342, 0); //mcause
                csrstore.insert(0x343, 0); //mtval
                csrstore.insert(0x345, 0); //mnxti
                csrstore.insert(0xFB1, 0); //mintstatus
                csrstore.insert(0x347, 0); //mintthresh
                csrstore.insert(0x348, 0); //mscratchcsw
                csrstore.insert(0x349, 0); //mscratchcswl
                RefCell::new(csrstore)
            },
            mmio: {
                let mut mmio = HashMap::new();
                for i in (0x1000..0x5000).step_by(4) {
                    mmio.insert(
                        i,
                        MMIOEntry {
                            clicintip: 0,
                            clicintie: 0,
                            clicintattr: 0,
                            clicintctl: 0,
                        },
                    );
                }
                RefCell::new(mmio)
            },
            queue: RefCell::new(BTreeMap::new()),
           // lines: lines,
            csr_ctl: csr_ctl,
        }
    }
}

#[typetag::serde()]
impl Component for CLIC {
    fn to_(&self) {
        println!("CLIC");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![
                    self.csr_addr.clone(),
                    self.csr_ctl.clone(),
                    self.csr_data.clone(),

                ],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    "csr_data".into(),
                ],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
     //   let data: u32 = simulator.get_input_value(&self.data).try_into().unwrap();
     //   let addr: u32 = simulator.get_input_value(&self.addr).try_into().unwrap();
    //    let data: u32 = (data >> (addr % 4) * 8).into(); //we only allow aligned accesses, if bytewise memory op, shift the byte into place in a word.
    //    let entry = addr - (addr % 4);
    //    let old_data:u32 = <MMIOEntry as Into<u32>>::into(*self
            // .mmio
            // .borrow()
            // .get(&(entry as usize))
            // .unwrap_or(&(0u32.into())));
        let csr_ctl:u32 = simulator.get_input_value(&self.csr_ctl).try_into().unwrap_or(0);
        let csr_addr:u32 = simulator.get_input_value(&self.csr_addr).try_into().unwrap_or(0);
        let csr_data:u32 = simulator.get_input_value(&self.csr_data).try_into().unwrap_or(0);
        let mut val = 0;
        trace!("ctl:{}, addr:{}, data:{}", csr_ctl, csr_addr, csr_data);
        match csr_ctl{
            0=>{}
            //write
            1=>{
                let mut csrstore = self.csrstore.borrow_mut();
                if csrstore.contains_key(&(csr_addr as usize)){
                    val = csrstore.get(&(csr_addr as usize)).unwrap().clone();
                    csrstore.insert(csr_addr as usize, csr_data as usize);
                }
            }
            //set
            2=>{
                let mut csrstore = self.csrstore.borrow_mut();
                if csrstore.contains_key(&(csr_addr as usize)){
                    val = csrstore.get(&(csr_addr as usize)).unwrap().clone();
                    csrstore.insert(csr_addr as usize, (csr_data as usize)|val);
                }
            }
            //clear
            3=>{
                let mut csrstore = self.csrstore.borrow_mut();
                if csrstore.contains_key(&(csr_addr as usize)){
                    val = csrstore.get(&(csr_addr as usize)).unwrap().clone();
                    csrstore.insert(csr_addr as usize, (!(csr_data as usize))&val);
                }
            }
            _=>{}
        }
        for entry in self.csrstore.borrow().clone().into_iter(){
            trace!("{:08x}:{:08x}", entry.0, entry.1);
        }
        trace!("CSR OUT:{:08x}", val);
        simulator.set_out_value(&self.id, "csr_data", val as u32);
        Ok(())
    }
}
