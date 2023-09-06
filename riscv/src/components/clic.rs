use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::{
    common::{Component, Condition, Id, Input, OutputType, Ports, Simulator},
    signal::SignalValue,
};

use priority_queue::PriorityQueue;
use std::{cell::RefCell, collections::HashMap};

#[derive(Serialize, Deserialize)]
pub struct CLIC {
    pub id: Id,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // configuration
    //pub big_endian: bool,

    // mmio ports
    pub data: Input,
    pub addr: Input,
    pub data_we: Input,

    //CSR ports
    pub csr_data: Input,
    pub csr_addr: Input,
    //1 = write, 2 = set, 3 = clear
    pub csr_ctl: Input,

    //MRET specific signal
    pub mret: Input,

    //PC input for MEPC update
    pub pc: Input,
    //interurpt lines
    // pub lines: Vec<Input>,

    //internal state
    pub csrstore: RefCell<HashMap<usize, usize>>, //address, val
    pub mmio: RefCell<HashMap<usize, MMIOEntry>>, //address, val
    pub queue: RefCell<PriorityQueue<u32, u8>>,   //prio, id's
}
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
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
        data: Input,
        addr: Input,
        data_we: Input,
        csr_data: Input,
        csr_addr: Input,
        //  lines: Vec<Input>,
        csr_ctl: Input,
        mret: Input,
        pc: Input,
    ) -> Self {
        CLIC {
            id: id,
            pos: pos,
            width: width,
            height: height,
            data: data,
            addr: addr,
            data_we: data_we,
            csr_data: csr_data,
            csr_addr: csr_addr,
            mret: mret,
            pc: pc,
            csrstore: {
                let mut csrstore = HashMap::new();
                csrstore.insert(0x300, 0); //mstatus
                csrstore.insert(0x305, 0b11); //mtvec, we only support vectored
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
            queue: RefCell::new(PriorityQueue::new()),
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
                    self.data.clone(),
                    self.addr.clone(),
                    self.data_we.clone(),
                    self.mret.clone(),
                    self.pc.clone(),
                ],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    "csr_data".into(),
                    "mmio_data".into(),
                    "mem_int_addr".into(),
                    "blu_int".into(),
                    "mret".into(),
                    "mepc".into(),
                ],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        //CSR IO Handling
        let csr_ctl: u32 = simulator
            .get_input_value(&self.csr_ctl)
            .try_into()
            .unwrap_or(0);
        let csr_addr: u32 = simulator
            .get_input_value(&self.csr_addr)
            .try_into()
            .unwrap_or(0);
        let mut csr_data: u32 = simulator
            .get_input_value(&self.csr_data)
            .try_into()
            .unwrap_or(0);
        //MMIO handling
        let addr: u32 = simulator
            .get_input_value(&self.addr)
            .try_into()
            .unwrap_or(0);
        let data: u32 = (simulator
            .get_input_value(&self.data)
            .try_into()
            .unwrap_or(0)
            >> (addr % 4) * 8)
            .into(); //we only allow aligned accesses, if bytewise memory op, shift the byte into place in a word.
        let mret: u32 = simulator
            .get_input_value(&self.mret)
            .try_into()
            .unwrap_or(0);
        let pc: u32 = simulator.get_input_value(&self.pc).try_into().unwrap_or(0);
        let mut val = 0;
        let mut blu_int = SignalValue::Uninitialized;
        let mut mmio_data = SignalValue::Uninitialized;
        let mut mem_int_addr = SignalValue::Uninitialized;
        let mut mret_sig: SignalValue = mret.into();
        let mut mepc = SignalValue::Uninitialized;
        if mret == 1 {
            let csrstore = self.csrstore.borrow();
            mepc = (*csrstore.get(&0x341).unwrap() as u32).into(); //infallible

            trace!("mret");
            simulator.set_out_value(&self.id, "mem_int_addr", mem_int_addr);
            simulator.set_out_value(&self.id, "blu_int", blu_int);
            simulator.set_out_value(&self.id, "csr_data", val as u32);
            simulator.set_out_value(&self.id, "mmio_data", mmio_data);
            simulator.set_out_value(&self.id, "mepc", mepc);
            simulator.set_out_value(&self.id, "mret", mret_sig);
            return Ok(());
        }
        trace!("ctl:{}, addr:{}, data:{}", csr_ctl, csr_addr, csr_data);
        match csr_ctl {
            0 => {}
            //write
            1 => {
                let mut csrstore = self.csrstore.borrow_mut();
                if csrstore.contains_key(&(csr_addr as usize)) {
                    if csr_addr == 0x305 {
                        //mtvec write
                        csr_data = csr_data | 0b11; //hardwire to vectored mode
                    }
                    val = csrstore.get(&(csr_addr as usize)).unwrap().clone();
                    csrstore.insert(csr_addr as usize, csr_data as usize);
                }
            }
            //set
            2 => {
                let mut csrstore = self.csrstore.borrow_mut();
                if csrstore.contains_key(&(csr_addr as usize)) {
                    if csr_addr == 0x305 {
                        //mtvec set
                        csr_data = csr_data | 0b11; //hardwire to vectored mode
                    }
                    val = csrstore.get(&(csr_addr as usize)).unwrap().clone();
                    csrstore.insert(csr_addr as usize, (csr_data as usize) | val);
                }
            }
            //clear
            3 => {
                let mut csrstore = self.csrstore.borrow_mut();
                if csrstore.contains_key(&(csr_addr as usize)) {
                    if csr_addr == 0x305 {
                        //mtvec clear
                        csr_data = csr_data | !0b11; //hardwire to vectored mode
                    }
                    val = csrstore.get(&(csr_addr as usize)).unwrap().clone();
                    csrstore.insert(csr_addr as usize, (!(csr_data as usize)) & val);
                }
            }
            _ => {}
        }
        trace!("preprune addr:{}", addr);
        let addr = addr - (addr % 4);
        trace!("clic mmio addr:{}", addr);
        let mut queue = self.queue.borrow_mut();
        let we: u32 = simulator.get_input_value(&self.data_we).try_into().unwrap();
        trace!("past we get");
        if (0x1000 <= addr) && (addr <= 0x5000) {
            //if within our mmio range
            if we == 2 {
                trace!("clic mmio write");
                let mut mmio = self.mmio.borrow_mut();
                let mmio_entry: MMIOEntry = data.into();
                if mmio_entry.clicintie == 1 && mmio_entry.clicintip == 1 {
                    //enqueue self if pending status and enable status are 1, this changes prio dynamically with prio change also.
                    trace!("interrupt enqueued");
                    queue.push(((addr - 0x1000) / 4), mmio_entry.clicintctl);
                }
                if mmio_entry.clicintie != 1 || mmio_entry.clicintip != 1 {
                    //dequeue self if pending or enabled status is 0
                    trace!("interrupt dequeued");
                    queue.remove(&((addr - 0x1000) / 4));
                }
                mmio.insert(addr as usize, data.into());
                trace!("write: {:08x} addr: {:08x}", data, addr);
            } else {
                mmio_data = SignalValue::Data(<MMIOEntry as Into<u32>>::into(
                    *self.mmio.borrow().get(&(addr as usize)).unwrap(),
                ));
            }
        };
        trace!("past mmio");
        //Interrupt dispatch
        let mut csrstore = self.csrstore.borrow_mut();
        let mstatus = csrstore.get(&0x300).unwrap().clone();
        let mintthresh = csrstore.get(&0x347).unwrap().clone();
        let mtvec = csrstore.get(&0x305).unwrap().clone();
        if mstatus & 8 == 8 {
            //if MIE is set
            if !queue.is_empty() {
                let interrupt = queue.peek().unwrap(); //peek highest prio interrupt
                if *interrupt.1 as usize > mintthresh {
                    let interrupt = queue.pop().unwrap(); //if above threshold, pop it
                                                          //now dispatch
                                                          //make memory output contents of mtvec + id*4 to branch mux
                                                          //set interrupt signal on branch control
                    csrstore.insert(0x300, mstatus & !0x8); //clear interrupt enable
                    csrstore.insert(0x341, pc as usize);
                    mem_int_addr = SignalValue::Data((mtvec as u32 + (interrupt.0) * 4) & !0b11);

                    blu_int = SignalValue::Data(1);
                    trace!(
                        "interrupt dispatched id:{} prio:{}",
                        interrupt.0,
                        interrupt.1
                    );
                }
            }
        }
        for entry in csrstore.clone().into_iter() {
            trace!("{:08x}:{:08x}", entry.0, entry.1);
        }
        trace!("CSR OUT:{:08x}", val);
        simulator.set_out_value(&self.id, "mem_int_addr", mem_int_addr);
        simulator.set_out_value(&self.id, "blu_int", blu_int);
        simulator.set_out_value(&self.id, "csr_data", val as u32);
        simulator.set_out_value(&self.id, "mmio_data", mmio_data);
        simulator.set_out_value(&self.id, "mepc", mepc);
        simulator.set_out_value(&self.id, "mret", mret_sig);
        Ok(())
    }
}
