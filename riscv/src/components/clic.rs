use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::common::InputPort;
use syncrim::{
    common::{Component, Condition, Id, Input, OutputType, Ports, Simulator},
    signal::{SignalSigned, SignalUnsigned, SignalValue},
};

use priority_queue::PriorityQueue;
use std::{cell::RefCell, collections::HashMap};
pub const CLIC_CSR_ADDR_ID: &str = "csr_addr";
pub const CLIC_CSR_CTL_ID: &str = "csr_ctl";
pub const CLIC_CSR_DATA_ID: &str = "csr_data";
pub const CLIC_DATA_ID: &str = "data";
pub const CLIC_ADDR_ID: &str = "addr";
pub const CLIC_DATA_WE_ID: &str = "data_we";
pub const CLIC_MRET_ID: &str = "mret";
pub const CLIC_PC_ID: &str = "pc";
pub const CLIC_PC_NEXT_ID: &str = "pc_next";
pub const CLIC_DATA_SIZE_ID: &str = "size";
pub const CLIC_CSR_DATA_OUT_ID: &str = "csr_data_o";
pub const CLIC_MMIO_DATA_OUT_ID: &str = "mmio_data_o";
pub const CLIC_MEM_INT_ADDR_ID: &str = "mem_int_addr";
pub const CLIC_INTERRUPT_ID: &str = "interrupt";
pub const CLIC_WRITE_RA_ENABLE_ID: &str = "write_ra_enable";
pub const CLIC_PC_ADDR_OUT_ID: &str = "pc_addr_out";
pub const CLIC_MEPC_OUT_ID: &str = "mepc_out";
// pub const CLIC_REG_FILE_WRITE_ID: &str = "reg_file_write";
pub const CLIC_STACK_DEPTH_OUT_ID: &str = "stack_depth_out";

#[derive(Serialize, Deserialize)]
struct CLICOp {
    pub mmio_op: Option<([u32; 2], u32)>,
    pub csr_op: Option<Vec<(usize, u32)>>,
    //op removed = true, op added = false
    pub queue_op: Vec<(u32, u8, bool)>,
}

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
    pub data_size: Input,

    //CSR ports
    pub csr_data: Input,
    pub csr_addr: Input,
    //1 = write, 2 = set, 3 = clear
    pub csr_ctl: Input,

    //MRET specific signal
    pub mret: Input,

    //PC input for MEPC update
    pub pc: Input,
    pub pc_next: Input,

    //internal state
    #[serde(skip)]
    pub csrstore: RefCell<HashMap<usize, usize>>, //address, val
    #[serde(skip)]
    pub mmio: RefCell<HashMap<usize, u8>>, //address, val
    #[serde(skip)]
    pub queue: RefCell<PriorityQueue<u32, u8>>, //prio, id's
    #[serde(skip)]
    pub clic_stack: RefCell<Vec<(u32, u32)>>,
    // #[serde(skip)]
    // pub stack_depth: RefCell<u32>, //current register stack depth
    history: RefCell<Vec<CLICOp>>,
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

impl From<MMIOEntry> for u32 {
    fn from(val: MMIOEntry) -> u32 {
        val.clicintip as u32
            | ((val.clicintie as u32) << 8)
            | ((val.clicintattr as u32) << 16)
            | ((val.clicintctl as u32) << 24)
    }
}
impl CLIC {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Id,
        pos: (f32, f32),
        width: f32,
        height: f32,
        data: Input,
        addr: Input,
        data_we: Input,
        data_size: Input,
        csr_data: Input,
        csr_addr: Input,
        //  lines: Vec<Input>,
        csr_ctl: Input,
        mret: Input,
        pc: Input,
        pc_next: Input,
    ) -> Self {
        CLIC {
            id,
            pos,
            width,
            height,
            data,
            addr,
            data_we,
            data_size,
            csr_data,
            csr_addr,
            mret,
            pc,
            pc_next,
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
                csrstore.insert(0xF14, 0); //mhartid
                csrstore.insert(0x350, 0); //stack_depth

                RefCell::new(csrstore)
            },
            mmio: {
                let mut mmio = HashMap::new();
                for i in 0x1000..0x5000 {
                    mmio.insert(i, 0);
                }
                RefCell::new(mmio)
            },
            queue: RefCell::new(PriorityQueue::new()),
            // lines: lines,
            csr_ctl,
            clic_stack: RefCell::new(Vec::new()),
            history: RefCell::new(vec![]),
            // stack_depth: 0.into(),
        }
    }
}

#[typetag::serde()]
impl Component for CLIC {
    fn reset(&self) {
        self.csrstore.swap({
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
            csrstore.insert(0xF14, 0); //mhartid
            csrstore.insert(0x350, 0); //stack_depth, vanilla clic config
            csrstore.insert(0x351, 0); //super mtvec
            &RefCell::new(csrstore)
        });
        self.mmio.swap({
            let mut mmio = HashMap::new();
            for i in 0x1000..0x5000 {
                mmio.insert(i, 0);
            }
            &RefCell::new(mmio)
        });
        self.queue.swap(&RefCell::new(PriorityQueue::new()));
        self.history.swap(&RefCell::new(vec![]));
        self.clic_stack.swap(&RefCell::new(Vec::new()));
    }

    fn to_(&self) {
        println!("CLIC");
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            CLIC_CSR_ADDR_ID => self.csr_addr = new_input,
            CLIC_CSR_CTL_ID => self.csr_ctl = new_input,
            CLIC_CSR_DATA_ID => self.csr_data = new_input,
            CLIC_DATA_ID => self.data = new_input,
            CLIC_ADDR_ID => self.addr = new_input,
            CLIC_DATA_WE_ID => self.data_we = new_input,
            CLIC_MRET_ID => self.mret = new_input,
            CLIC_PC_ID => self.pc = new_input,
            CLIC_DATA_SIZE_ID => self.data_size = new_input,
            _ => (),
        }
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: CLIC_CSR_ADDR_ID.to_string(),
                        input: self.csr_addr.clone(),
                    },
                    &InputPort {
                        port_id: CLIC_CSR_CTL_ID.to_string(),
                        input: self.csr_ctl.clone(),
                    },
                    &InputPort {
                        port_id: CLIC_CSR_DATA_ID.to_string(),
                        input: self.csr_data.clone(),
                    },
                    &InputPort {
                        port_id: CLIC_DATA_ID.to_string(),
                        input: self.data.clone(),
                    },
                    &InputPort {
                        port_id: CLIC_ADDR_ID.to_string(),
                        input: self.addr.clone(),
                    },
                    &InputPort {
                        port_id: CLIC_DATA_WE_ID.to_string(),
                        input: self.data_we.clone(),
                    },
                    &InputPort {
                        port_id: CLIC_MRET_ID.to_string(),
                        input: self.mret.clone(),
                    },
                    &InputPort {
                        port_id: CLIC_PC_ID.to_string(),
                        input: self.pc.clone(),
                    },
                    &InputPort {
                        port_id: CLIC_PC_NEXT_ID.to_string(),
                        input: self.pc_next.clone(),
                    },
                    &InputPort {
                        port_id: CLIC_DATA_SIZE_ID.to_string(),
                        input: self.data_size.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![
                    // CLIC_REG_FILE_WRITE_ID,
                    CLIC_CSR_DATA_OUT_ID,
                    CLIC_MMIO_DATA_OUT_ID,
                    CLIC_MEM_INT_ADDR_ID,
                    CLIC_PC_ADDR_OUT_ID,
                    CLIC_INTERRUPT_ID,
                    CLIC_MEPC_OUT_ID,
                    CLIC_STACK_DEPTH_OUT_ID,
                ],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let mut history_entry = CLICOp {
            csr_op: None,
            mmio_op: None,
            queue_op: vec![],
        };

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
        let data: u32 = simulator
            .get_input_value(&self.data)
            .try_into()
            .unwrap_or(0);
        let mret: u32 = simulator
            .get_input_value(&self.mret)
            .try_into()
            .unwrap_or(0);
        let pc: u32 = simulator.get_input_value(&self.pc).try_into().unwrap();
        let pc_next: u32 = simulator.get_input_value(&self.pc_next).try_into().unwrap();
        let data_size: u32 = simulator
            .get_input_value(&self.data_size)
            .try_into()
            .unwrap_or(0);
        let mut val = 0;
        let mut blu_int = SignalValue::Data(0); // default to pc
        let mut mmio_data = SignalValue::Uninitialized;
        let mut mem_int_addr = SignalValue::Uninitialized;

        // vanilla clic behavior
        let mepc;
        let mut stack_depth = *self.csrstore.borrow().get(&(0x350 as usize)).unwrap();
        if mret == 1 {
            //panic!("currently not supported");
            let mut csrstore = self.csrstore.borrow_mut();
            mepc = (*csrstore.get(&0x341).unwrap() as u32).into(); //infallible
            let mut mstatus = *csrstore.get(&0x300).unwrap(); //infallible
            if mstatus >> 7 & 1 == 1 {
                mstatus |= 0x8; //if mpie then set mie
            } else {
                mstatus &= !0x8; //if not mpie, ensure not mie
            }
            let old_val = mstatus;
            mstatus |= 0b1 << 7; //mpie is set on mret
            csrstore.insert(0x300, mstatus);
            history_entry.csr_op = Some(vec![(0x300, old_val as u32), (0x350, stack_depth as u32)]);
            trace!("mret");
            simulator.set_out_value(&self.id, "mem_int_addr", mem_int_addr);
            // select mepc signal on interrupt mux
            //simulator.set_out_value(id, field, value)
            simulator.set_out_value(&self.id, CLIC_INTERRUPT_ID, SignalValue::Data(2));
            simulator.set_out_value(&self.id, "csr_data_o", val as u32);
            simulator.set_out_value(&self.id, "mmio_data_o", mmio_data);
            simulator.set_out_value(&self.id, "mepc_out", mepc);
            // simulator.set_out_value(&self.id, "mret_out", mret_sig);
            self.history.borrow_mut().push(history_entry);
            stack_depth += 1;
            simulator.set_out_value(
                &self.id,
                CLIC_STACK_DEPTH_OUT_ID,
                SignalValue::Data(stack_depth as u32),
            );
            csrstore.insert(0x350, stack_depth);
            return Ok(());
        }

        match csr_ctl {
            0 => {}
            //write
            1 => {
                let mut csrstore = self.csrstore.borrow_mut();
                if csrstore.contains_key(&(csr_addr as usize)) {
                    if csr_addr == 0x305 {
                        //mtvec write
                        csr_data |= 0b11; //hardwire to vectored mode
                    }
                    if csr_addr != 0xf14 {
                        //mhartid RO
                        val = *csrstore.get(&(csr_addr as usize)).unwrap();
                        //println!("val:{}", val);
                        csrstore.insert(csr_addr as usize, csr_data as usize);
                        history_entry.csr_op = Some(vec![(csr_addr as usize, val as u32)]);
                        //println!("val:{}", val);
                    }
                }
            }
            //set
            2 => {
                let mut csrstore = self.csrstore.borrow_mut();
                if csrstore.contains_key(&(csr_addr as usize)) {
                    if csr_addr == 0x305 {
                        //mtvec set
                        csr_data |= 0b11; //hardwire to vectored mode
                    }
                    if csr_addr != 0xf14 {
                        //mhartid RO
                        val = *csrstore.get(&(csr_addr as usize)).unwrap();
                        csrstore.insert(csr_addr as usize, (csr_data as usize) | val);
                        history_entry.csr_op = Some(vec![(csr_addr as usize, val as u32)]);
                    }
                }
            }
            //clear
            3 => {
                let mut csrstore = self.csrstore.borrow_mut();
                if csrstore.contains_key(&(csr_addr as usize)) {
                    if csr_addr == 0x305 {
                        //mtvec clear
                        csr_data |= !0b11; //hardwire to vectored mode
                    }
                    if csr_addr != 0xf14 {
                        //mhartid RO
                        val = *csrstore.get(&(csr_addr as usize)).unwrap();
                        csrstore.insert(csr_addr as usize, (!(csr_data as usize)) & val);
                        history_entry.csr_op = Some(vec![(csr_addr as usize, val as u32)]);
                    }
                }
            }
            _ => {}
        }

        // mmio handling
        let offset = addr % 4;
        //let addr = addr - (addr % 4);
        let mut queue = self.queue.borrow_mut();
        let we: u32 = simulator.get_input_value(&self.data_we).try_into().unwrap();
        if (0x1000..=0x5000).contains(&addr) {
            //if within our mmio range
            if we == 2 {
                trace!("clic mmio write");
                let old_entries: [u32; 2] = [
                    self.read(addr as usize - offset as usize, 4_usize, false, false)
                        .try_into()
                        .unwrap(),
                    self.read(addr as usize - offset as usize + 4, 4_usize, false, false)
                        .try_into()
                        .unwrap(),
                ];
                history_entry.mmio_op = Some((old_entries, addr - offset));
                let mut mask: u64 = 0;
                for i in 0..data_size {
                    mask |= 0xFF << (i * 8);
                }
                mask <<= offset;
                let mmio_entries: [MMIOEntry; 2] = [
                    (old_entries[0] ^ (((data << (offset * 8)) ^ old_entries[0]) & mask as u32))
                        .into(),
                    (old_entries[1]
                        ^ (((data.checked_shr((4 - offset) * 8)).unwrap_or(0) << (offset * 8))
                            ^ old_entries[1])
                            & mask.checked_shr(32).unwrap_or(0) as u32)
                        .into(),
                ];
                for (i, mmio_entry) in mmio_entries.into_iter().enumerate() {
                    if mmio_entry.clicintie == 1 && mmio_entry.clicintip == 1 {
                        //enqueue self if pending status and enable status are 1, this changes prio dynamically with prio change also.
                        history_entry.queue_op.push((
                            (addr - offset + 4u32 * i as u32 - 0x1000) / 4,
                            mmio_entry.clicintctl,
                            false,
                        ));
                        queue.push(
                            (addr - offset + 4u32 * i as u32 - 0x1000) / 4,
                            mmio_entry.clicintctl,
                        );
                    }
                    if mmio_entry.clicintie != 1 || mmio_entry.clicintip != 1 {
                        //dequeue self if pending or enabled status is 0
                        if queue
                            .remove(&((addr - offset + 4u32 * i as u32 - 0x1000) / 4))
                            .is_some()
                        {
                            history_entry.queue_op.push((
                                (addr - offset + 4u32 * i as u32 - 0x1000) / 4,
                                mmio_entry.clicintctl,
                                true,
                            ));
                        };
                    }
                }
                self.write(
                    addr as usize,
                    data_size as usize,
                    false,
                    SignalValue::Data(data),
                );
                trace!("write: {:08x} addr: {:08x}", data, addr);
            } else if we == 1 {
                mmio_data = self.read(addr as usize, data_size as usize, false, false);
            }
        };

        //Interrupt dispatch
        let mut csrstore = self.csrstore.borrow_mut();
        let mstatus = *csrstore.get(&0x300).unwrap();
        let mut mintthresh = *csrstore.get(&0x347).unwrap();
        let mtvec = *csrstore.get(&0x305).unwrap();
        let super_mtvec = *csrstore.get(&0x351).unwrap();
        //let mut stack_depth = self.stack_depth.borrow_mut();

        // interrupt return, super-clic
        let mut return_from_interrupt = None;
        let mut clic_stack = self.clic_stack.borrow_mut();

        if let Some((old_threshold, current_mepc)) = clic_stack.last() {
            let (old_threshold, current_mepc) = (*old_threshold, *current_mepc);
            trace!("clic stack {:#x?}", clic_stack);
            trace!(
                "pc {:#X}, pc_next {:#X}, current_mepc {:#X}",
                pc,
                pc_next,
                current_mepc
            );
            // check if we are about to return from interrupt in super-clic mode
            if pc_next == current_mepc {
                trace!("---- return from interrupt ----");
                let _ = clic_stack.pop();
                csrstore.insert(0x347, old_threshold as usize); // set old threshold
                mintthresh = old_threshold as usize;
                stack_depth += 1;
                csrstore.insert(0x350, stack_depth);
                return_from_interrupt = Some(current_mepc);
            }
        }
        drop(clic_stack);
        // TODO: edge case of tail chaining
        if mstatus & 8 == 8 {
            //if MIE is set
            if !queue.is_empty() {
                let (interrupt_id, interrupt_priority) = queue.peek().unwrap(); // peek highest prio interrupt
                let (interrupt_id, interrupt_priority) = (*interrupt_id, *interrupt_priority);
                if interrupt_priority as usize > mintthresh {
                    let _ = queue.pop().unwrap(); //if above threshold, pop it
                                                  //now dispatch
                                                  //make memory output contents of mtvec + id*4 to branch mux
                                                  //set interrupt signal on branch control
                    history_entry
                        .queue_op
                        .push((interrupt_id, interrupt_priority, true));
                    match history_entry.csr_op {
                        Some(ref mut v) => {
                            v.push((0x300, *csrstore.get(&0x300_usize).unwrap() as u32));
                            v.push((0x341, *csrstore.get(&0x341_usize).unwrap() as u32));
                        }
                        None => {
                            history_entry.csr_op = Some(vec![
                                (0x300, *csrstore.get(&0x300_usize).unwrap() as u32),
                                (0x341, *csrstore.get(&0x341_usize).unwrap() as u32),
                            ])
                        }
                    }

                    // mepc
                    let new_mepc = if let Some(mepc_current) = return_from_interrupt {
                        mepc_current as usize
                    } else {
                        pc as usize
                    };

                    //facepalm
                    if (stack_depth as i32) <= 0 {
                        // vanilla mode
                        //panic!("no vanilla");
                        csrstore.insert(0x300, (mstatus & !0x8) | 0b1 << 7); //clear interrupt enable, set mpie
                                                                             // if vanilla mode, use level 0 register.
                        mem_int_addr =
                            SignalValue::Data((mtvec as u32 + (interrupt_id) * 4) & !0b11);
                    } else {
                        // super clic
                        //  let current_mepc = *csrstore.get(&0x341).unwrap() as u32;
                        // mepc
                        let current_threshold = *csrstore.get(&0x347_usize).unwrap() as u32;
                        self.clic_stack
                            .borrow_mut()
                            .push((current_threshold, new_mepc as u32));
                        csrstore.insert(0x347, interrupt_priority as usize); // set new threshold
                        mem_int_addr =
                            SignalValue::Data((super_mtvec as u32 + (interrupt_id) * 4) & !0b11);
                    }
                    // write to csr
                    csrstore.insert(0x341, new_mepc);
                    blu_int = SignalValue::Data(1);

                    stack_depth -= 1;
                    csrstore.insert(0x350, stack_depth);
                    trace!("STACK DEPTH: {}", stack_depth as i32);
                    //if stack_depth == 0 {
                    //    panic!("stack depleted, vanilla click not yet supported");
                    //}

                    trace!(
                        "interrupt dispatched id:{} prio:{}",
                        interrupt_id,
                        interrupt_priority
                    );
                }
            }
        }

        for entry in csrstore.clone().into_iter() {
            trace!("{:08x}:{:08x}", entry.0, entry.1);
        }
        trace!("CSR OUT:{:08x}", val);
        trace!("QUEUE:{:?}", queue);
        mepc = SignalValue::Data(*csrstore.get(&0x341).unwrap() as u32);
        trace!("MEPC:{:?}", mepc);
        self.history.borrow_mut().push(history_entry);
        simulator.set_out_value(
            &self.id,
            "stack_depth_out",
            SignalValue::Data(stack_depth as u32),
        );
        simulator.set_out_value(&self.id, "mem_int_addr", mem_int_addr);
        simulator.set_out_value(&self.id, CLIC_INTERRUPT_ID, blu_int);
        simulator.set_out_value(&self.id, "csr_data_o", val as u32);
        simulator.set_out_value(&self.id, "mmio_data_o", mmio_data);
        simulator.set_out_value(&self.id, "mepc_out", mepc);
        // simulator.set_out_value(&self.id, "mret_out", mret_sig);

        trace!("CLIC_INTERRUPT_ID {:?}", blu_int);
        Ok(())
    }

    fn un_clock(&self) {
        let mut entry = self.history.borrow_mut().pop().unwrap();
        if let Some(mut ops) = entry.csr_op {
            while let Some(op) = ops.pop() {
                // println!("insert csr {:03x}, {:08x}", op.0, op.1);
                self.csrstore.borrow_mut().insert(op.0, op.1 as usize);
            }
        }
        if let Some(op) = entry.mmio_op {
            self.write(
                op.1.try_into().unwrap(),
                4,
                false,
                SignalValue::Data(op.0[0]),
            );
            self.write(
                (op.1 + 4).try_into().unwrap(),
                4,
                false,
                SignalValue::Data(op.0[1]),
            );
        }
        // println!("queue_op:{:?}", entry.queue_op);
        while let Some(e) = entry.queue_op.pop() {
            //readd
            if e.2 {
                //   println!("re add id:{} prio:{}", e.0, e.1);
                self.queue.borrow_mut().push(e.0, e.1);
            }
            //remove
            else {
                //  println!("remove id:{} prio:{}", e.0, e.1);
                self.queue.borrow_mut().remove(&e.0);
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CLIC {
    fn read(&self, addr: usize, size: usize, sign: bool, big_endian: bool) -> SignalValue {
        let data: Vec<u8> = (0..size)
            .map(|i| *self.mmio.borrow().get(&(addr + i)).unwrap_or(&0))
            .collect();

        let data = data.as_slice();

        match size {
            1 => {
                if sign {
                    data[0] as i8 as SignalSigned as SignalUnsigned
                } else {
                    data[0] as SignalUnsigned
                }
            }
            2 => {
                if sign {
                    if big_endian {
                        trace!("read signed half word be");
                        let i_16 = i16::from_be_bytes(data.try_into().unwrap());
                        trace!("i_16 {:x?}", i_16);
                        let i_32 = i_16 as i32;
                        trace!("i_32 {:x?}", i_32);
                        i_32 as SignalUnsigned
                    } else {
                        trace!("read signed half word le");
                        let i_16 = i16::from_le_bytes(data.try_into().unwrap());
                        trace!("i_16 {:x?}", i_16);
                        let i_32 = i_16 as i32;
                        trace!("i_32 {:x?}", i_32);
                        i_32 as SignalUnsigned
                    }
                } else if big_endian {
                    trace!("read unsigned half word be");
                    let u_16 = u16::from_be_bytes(data.try_into().unwrap());
                    trace!("u_16 {:x?}", u_16);
                    let u_32 = u_16 as u32;
                    trace!("u_32 {:x?}", u_32);
                    u_32 as SignalUnsigned
                } else {
                    trace!("read unsigned half word le");
                    let u_16 = u16::from_le_bytes(data.try_into().unwrap());
                    trace!("u_16 {:x?}", u_16);
                    let u_32 = u_16 as u32;
                    trace!("u_32 {:x?}", u_32);
                    u_32 as SignalUnsigned
                }
            }
            4 => {
                if sign {
                    if big_endian {
                        i32::from_be_bytes(data.try_into().unwrap()) as SignalUnsigned
                    } else {
                        i32::from_le_bytes(data.try_into().unwrap()) as SignalUnsigned
                    }
                } else if big_endian {
                    u32::from_be_bytes(data.try_into().unwrap()) as SignalUnsigned
                } else {
                    u32::from_le_bytes(data.try_into().unwrap()) as SignalUnsigned
                }
            }
            _ => panic!("illegal sized memory operation"),
        }
        .into()
    }
    fn write(&self, addr: usize, size: usize, big_endian: bool, data: SignalValue) {
        let data: SignalUnsigned = data.try_into().unwrap();
        match size {
            1 => {
                trace!("write byte");
                self.mmio.borrow_mut().insert(addr, data as u8);
            }
            2 => {
                if big_endian {
                    trace!("write half word be");
                    (data as u16)
                        .to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.mmio.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    trace!("write half word le");
                    (data as u16)
                        .to_le_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.mmio.borrow_mut().insert(addr + i, *bytes);
                        })
                }
            }

            4 => {
                if big_endian {
                    trace!("write word be");
                    data.to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.mmio.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    trace!("write word le");
                    data.to_le_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.mmio.borrow_mut().insert(addr + i, *bytes);
                        })
                }
            }
            _ => {
                panic!("illegal sized memory operation, size = {}", size)
            }
        };
    }
}
