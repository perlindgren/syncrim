use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::common::InputPort;
use syncrim::{
    common::{Component, Condition, Id, Input, OutputType, Ports, Simulator},
    signal::{SignalSigned, SignalUnsigned, SignalValue},
};

use priority_queue::PriorityQueue;

use std::{cell::RefCell, collections::HashMap};
const CLIC_TIMESTAMP_BASE: u32 = 0xB40;
const CLIC_TIMESTAMP_PRESCALER: usize = 0x0;
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
pub const CLIC_INTERRUPT_INV_ID: &str = "interrupt_inv";
pub const CLIC_WRITE_RA_ENABLE_ID: &str = "write_ra_enable";
pub const CLIC_PC_ADDR_OUT_ID: &str = "pc_addr_out";
pub const CLIC_MEPC_OUT_ID: &str = "mepc_out";
pub const CLIC_MEPC_ISR_MUX: &str = "isr_mepc_sel";
pub const CLIC_RF_RA_WE: &str = "rf_ra_we";
// pub const CLIC_REG_FILE_WRITE_ID: &str = "reg_file_write";
pub const CLIC_STACK_DEPTH_OUT_ID: &str = "stack_depth_out";

pub const TIMER_WIDTH: u32 = 16;
pub const TIMER_PRES_WIDTH: u32 = 4;
pub const TIMER_ADDR: u32 = 0x400;
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
    #[serde(skip)]
    pub mtime: RefCell<u64>,
    #[serde(skip)]
    pub monotonic: RefCell<u64>,
    #[serde(skip)]
    pub mtimecomp: RefCell<u64>,
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

//it would be nice to have a bitfield type instead of wildly using u32s and hoping for the best
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TimerCSR {
    pub counter_top: u32,
    pub prescaler: u32,
}

impl Into<TimerCSR> for u32 {
    fn into(self) -> TimerCSR {
        trace!("TIMER CSR: {}", self);
        TimerCSR {
            counter_top: (self & ((2_u32.pow(TIMER_WIDTH) - 1) << TIMER_PRES_WIDTH))
                >> TIMER_PRES_WIDTH,
            prescaler: self & (2_u32.pow(TIMER_PRES_WIDTH) - 1),
        }
    }
}

impl Into<u32> for TimerCSR {
    fn into(self) -> u32 {
        self.counter_top << TIMER_PRES_WIDTH | self.prescaler
    }
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
impl From<MMIOEntry> for usize {
    fn from(val: MMIOEntry) -> usize {
        val.clicintip as usize
            | ((val.clicintie as usize) << 8)
            | ((val.clicintattr as usize) << 16)
            | ((val.clicintctl as usize) << 24)
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
                csrstore.insert(0x350, 7); //stack_depth
                for i in 0xB00..0xBC0 {
                    csrstore.insert(i, 0); //set up individual interrupt config CSRs
                }
                RefCell::new(csrstore)
            },
            monotonic: RefCell::new(0),
            mmio: {
                let mut mmio = HashMap::new();
                for i in 0x1000..0x10C0 {
                    //for now, we support 0xBF interrupts to give ourselves a
                    //continuous CSR range
                    mmio.insert(i, 0);
                }
                RefCell::new(mmio)
            },
            queue: RefCell::new(PriorityQueue::new()),
            // lines: lines,
            csr_ctl,
            clic_stack: RefCell::new(Vec::new()),
            history: RefCell::new(vec![]),
            mtime: 0.into(),
            mtimecomp: 0.into(),
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
            csrstore.insert(0x350, 7); //stack_depth, vanilla clic config
            csrstore.insert(0x351, 0); //super mtvec
            csrstore.insert(0x400, 0); //timer
            for i in 0xB00..=0xBBF {
                csrstore.insert(i, 0); //set up individual interrupt config CSRs
            }
            for i in 0xD00..=0xDBF {
                csrstore.insert(i, 0); //set up timestamping CSRs
            }
            &RefCell::new(csrstore)
        });
        self.mmio.swap({
            let mut mmio = HashMap::new();
            for i in 0x1000..0x5010 {
                mmio.insert(i, 0);
            }
            &RefCell::new(mmio)
        });
        self.queue.swap(&RefCell::new(PriorityQueue::new()));
        self.history.swap(&RefCell::new(vec![]));
        self.clic_stack.swap(&RefCell::new(Vec::new()));
        self.monotonic.swap(&RefCell::new(0));
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
                    CLIC_INTERRUPT_INV_ID,
                    CLIC_MEPC_OUT_ID,
                    CLIC_STACK_DEPTH_OUT_ID,
                    CLIC_MEPC_ISR_MUX,
                    CLIC_RF_RA_WE,
                ],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        //get inputs
        let csr_ctl: u32 = simulator
            .get_input_value(&self.csr_ctl)
            .try_into()
            .unwrap_or(0);
        let csr_addr: u32 = simulator
            .get_input_value(&self.csr_addr)
            .try_into()
            .unwrap_or(0);
        let csr_data: u32 = simulator
            .get_input_value(&self.csr_data)
            .try_into()
            .unwrap_or(0);
        let mmio_addr: u32 = simulator
            .get_input_value(&self.addr)
            .try_into()
            .unwrap_or(0);
        let mmio_data: u32 = simulator
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
        let mmio_we: u32 = simulator.get_input_value(&self.data_we).try_into().unwrap();

        // define outputs
        let csr_out;
        let mut blu_int = false; // default to pc
                                 //let mut int_mux_ctl = 0; // default to pc
        let mut mem_int_addr = SignalValue::Uninitialized;
        let mut rf_ra_we = SignalValue::Data(0);
        let mut isr_mepc_select = SignalValue::Uninitialized;
        let mut pc_out_signal = SignalValue::Uninitialized;

        // get state
        //csr store
        let mut csrstore = self.csrstore.borrow_mut();
        // operation history for reversing
        let mut history = self.history.borrow_mut();
        // interrupt priority queue
        let mut queue = self.queue.borrow_mut();
        // super-clic threshold/return address stack
        let mut clic_stack = self.clic_stack.borrow_mut();

        // init a history entry for this cycle
        let mut history_entry = CLICOp {
            csr_op: None,
            mmio_op: None,
            queue_op: vec![],
        };
        //dispatched interrupt id, used to unpend in csr store
        let mut dispatched_interrupt_id = None;
        let mut monotonic = self.monotonic.borrow_mut();
        *monotonic += 1;
        // handle CSR op if there was any
        csr_out = self.csr_op(
            &mut csrstore,
            &mut history_entry,
            &mut queue,
            csr_ctl,
            csr_data,
            csr_addr,
        );

        // with CSR IO handled, get all of the neccessary CSR values
        let mut stack_depth = *csrstore.get(&0x350).unwrap() as i32;
        let mut mstatus = *csrstore.get(&0x300).unwrap();
        let mut mintthresh = *csrstore.get(&0x347).unwrap();
        let mtvec = *csrstore.get(&0x305).unwrap();
        let super_mtvec = *csrstore.get(&0x351).unwrap();

        let mut mepc = *csrstore.get(&0x341).unwrap();

        // INTERRUPT RETURN
        // super-clic
        let mut return_from_interrupt = None;

        if let Some((old_threshold, current_mepc)) = clic_stack.last() {
            let (old_threshold, current_mepc) = (*old_threshold, *current_mepc);
            // trace!("clic stack {:#x?}", clic_stack);
            trace!(
                "pc {:#X}, pc_next {:#X}, current_mepc {:#X}",
                pc,
                pc_next,
                current_mepc
            );
            // check if we are about to return from interrupt in super-clic mode
            // by comparing against magic number
            //if pc_next == current_mepc {
            if pc_next == 0xFFFF_FFFF {
                let _ = clic_stack.pop();
                // set old threshold
                mintthresh = old_threshold as usize;
                stack_depth += 1;

                return_from_interrupt = Some(current_mepc);
                isr_mepc_select = SignalValue::Data(0);
                blu_int = true;
                pc_out_signal = SignalValue::Data(mepc as u32);
            }
        }
        // vanilla clic (MRET)
        if mret == 1 {
            //we are changing mstatus and stack_depth, push to history
            if mstatus >> 7 & 1 == 1 {
                mstatus |= 0x8; //if mpie then set mie
            } else {
                mstatus &= !0x8; //if not mpie, ensure not mie
            }
            mstatus |= 0b1 << 7; //mpie is set on mret
            trace!("mret");
            // select mepc on the mux since mret
            isr_mepc_select = SignalValue::Data(0);
            // select interrupt mux on the PC adder mux
            blu_int = true;
            stack_depth += 1;
            pc_out_signal = SignalValue::Data(mepc as u32);
        }
        // END INTERRUPT RETURN
        // handle mmio
        let mmio_data = self.mmio_op(
            mmio_addr,
            mmio_we,
            data_size,
            mmio_data,
            &mut history_entry,
            &mut queue,
            &mut csrstore,
        );

        let mut mtime = self.mtime.borrow_mut();
        let timer_t: TimerCSR = (*csrstore.get(&(TIMER_ADDR as usize)).unwrap_or(&0) as u32).into();
        let mtimecomp = timer_t.counter_top;
        if *mtime << timer_t.prescaler >= mtimecomp as u64 {
            // set pending bit of interrupt 0, call it the timer interrupt
            //self.csr_op(&mut csrstore, &mut history_entry,&mut queue, 2, 1, 0xB09);
            trace!("COUNTER_TOP:{}", mtimecomp);
            self.mmio_op(
                0x1000,
                2,
                1,
                1,
                &mut history_entry,
                &mut queue,
                &mut csrstore,
            );
            *mtime = 0;
        } else {
            /*self.mmio_op(
                0x1000,
                2,
                1,
                0,
                &mut history_entry,
                &mut queue,
                &mut csrstore,
            );*/
            *mtime += 1;
            // clear pending bit if compare fails we shouldn't do this...
            //self.csr_op(&mut csrstore, &mut history_entry, &mut queue, 3, 1, 0xB09);
        }
        //Interrupt dispatch

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
                    // mepc

                    let new_mepc = if mret == 1 {
                        mepc
                    } else if let Some(mepc_current) = return_from_interrupt {
                        mepc_current as usize
                    } else {
                        // take into account any jumps
                        pc_next as usize
                    };
                    //vanilla mode
                    if (stack_depth) <= 0 {
                        mstatus = (mstatus & !0x8) | 0b1 << 7; //clear interrupt enable, set mpie
                        mem_int_addr =
                            SignalValue::Data((mtvec as u32 + (interrupt_id) * 4) & !0b11);
                    } else {
                        // super clic
                        clic_stack.push((mintthresh as u32, new_mepc as u32));
                        mintthresh = interrupt_priority as usize;
                        mem_int_addr =
                            SignalValue::Data((super_mtvec as u32 + (interrupt_id) * 4) & !0b11);
                    }
                    // write to csr
                    mepc = new_mepc;
                    blu_int = true;
                    rf_ra_we = SignalValue::Data(1);
                    stack_depth -= 1;
                    trace!("STACK DEPTH: {}", stack_depth);
                    isr_mepc_select = SignalValue::Data(0);
                    pc_out_signal = SignalValue::Data(
                        (*csrstore.get(&(0xB00 + interrupt_id as usize)).unwrap() as u32) << 2,
                    );

                    trace!(
                        "interrupt dispatched id:{} prio:{}",
                        interrupt_id,
                        interrupt_priority
                    );
                    csrstore.insert(
                        (CLIC_TIMESTAMP_BASE + interrupt_id) as usize,
                        ((*monotonic as u32) >> CLIC_TIMESTAMP_PRESCALER) as usize,
                    );
                    dispatched_interrupt_id = Some(interrupt_id);
                }
            }
        }
        // END INTERRUPT_DISPATCH
        // tracing...
        for entry in csrstore.clone().into_iter() {
            if entry.0 >= 0xB20 && entry.0 <= 0xB2A {}
        }
        //   trace!("CSR OUT:{:08x}", csr_out);
        trace!("QUEUE:{:?}", queue);
        simulator.set_out_value(
            &self.id,
            CLIC_STACK_DEPTH_OUT_ID,
            SignalValue::Data(stack_depth as u32),
        );
        trace!("clic_stack:{:x?}", clic_stack);
        let blu_int_value: SignalValue = blu_int.into();
        let blu_int_inv_value: SignalValue = (!blu_int).into();

        // write the new CSR values back to the csr store
        self.csr_op(
            &mut csrstore,
            &mut history_entry,
            &mut queue,
            1,
            stack_depth as u32,
            0x350,
        );
        self.csr_op(
            &mut csrstore,
            &mut history_entry,
            &mut queue,
            1,
            mstatus as u32,
            0x300,
        );
        self.csr_op(
            &mut csrstore,
            &mut history_entry,
            &mut queue,
            1,
            mintthresh as u32,
            0x347,
        );
        self.csr_op(
            &mut csrstore,
            &mut history_entry,
            &mut queue,
            1,
            mtvec as u32,
            0x305,
        );
        self.csr_op(
            &mut csrstore,
            &mut history_entry,
            &mut queue,
            1,
            super_mtvec as u32,
            0x351,
        );
        self.csr_op(
            &mut csrstore,
            &mut history_entry,
            &mut queue,
            1,
            mepc as u32,
            0x341,
        );

        if let Some(interrupt_id) = dispatched_interrupt_id {
            self.csr_op(
                &mut csrstore,
                &mut history_entry,
                &mut queue,
                3,
                0x1,
                0xB20 + interrupt_id,
            );
        }
        history.push(history_entry);
        simulator.set_out_value(&self.id, "mem_int_addr", mem_int_addr);
        simulator.set_out_value(&self.id, CLIC_INTERRUPT_ID, blu_int_value);
        simulator.set_out_value(&self.id, CLIC_INTERRUPT_INV_ID, blu_int_inv_value);
        // simulator.set_out_value(&self.id, CLIC_INTERRUPT_MUX, blu_int);
        simulator.set_out_value(&self.id, "csr_data_o", csr_out as u32);
        simulator.set_out_value(
            &self.id,
            "mmio_data_o",
            if let Some(data) = mmio_data {
                data
            } else {
                SignalValue::Data(0)
            },
        );
        simulator.set_out_value(&self.id, "mepc_out", pc_out_signal);
        simulator.set_out_value(&self.id, CLIC_MEPC_ISR_MUX, isr_mepc_select);
        simulator.set_out_value(&self.id, CLIC_RF_RA_WE, rf_ra_we);
        // simulator.set_out_value(&self.id, "mret_out", mret_sig);
        trace!("MINTTHRESH {}", mintthresh);
        trace!("CLIC_INTERRUPT_ID {:?}", blu_int);
        Ok(())
    }

    fn un_clock(&self) {
        // TODO: Add super-clic stack ops
        let mut entry = self.history.borrow_mut().pop().unwrap();
        if let Some(mut ops) = entry.csr_op {
            while let Some(op) = ops.pop() {
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

        while let Some(e) = entry.queue_op.pop() {
            //readd
            if e.2 {
                self.queue.borrow_mut().push(e.0, e.1);
            }
            //remove
            else {
                self.queue.borrow_mut().remove(&e.0);
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CLIC {
    fn mmio_op(
        &self,
        addr: SignalUnsigned,
        we: SignalUnsigned,
        data_size: SignalUnsigned,
        data: SignalUnsigned,
        history_entry: &mut CLICOp,
        queue: &mut PriorityQueue<u32, u8>,
        csrstore: &mut HashMap<usize, usize>,
    ) -> Option<SignalValue> {
        let mut mmio_data = None;
        let offset = addr % 4;
        trace!("mmio_op {:x} {:x} {:x} {:x}", addr, we, data_size, data);
        if (0x1000..=0x5000).contains(&addr) {
            //if within our mmio range
            if we == 2 {
                //trace!("clic mmio write");

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
                /* trace!(
                    "CSRSTORE INSERT {:?}, {:?}, addr: {:x}, {:x}",
                    mmio_entries[0],
                    mmio_entries[1],
                    (addr as usize - offset as usize - 0x1000_usize) / 4 + 0xB00,
                    (addr as usize - offset as usize + 4 - 0x1000_usize) / 4 + 0xB00,
                );*/

                csrstore.insert(
                    (addr as usize - offset as usize - 0x1000_usize) / 4 + 0xB20,
                    mmio_entries[0].into(),
                );
                csrstore.insert(
                    (addr as usize - offset as usize + 4 - 0x1000_usize) / 4 + 0xB20,
                    mmio_entries[1].into(),
                );
                for (i, mmio_entry) in mmio_entries.into_iter().enumerate() {
                    if mmio_entry.clicintie == 1 && mmio_entry.clicintip == 1 {
                        //enqueue self if pending status and enable status are 1, this changes prio dynamically with prio change also.
                        history_entry.queue_op.push((
                            ((addr - offset + 4u32 * i as u32 - 0x1000) / 4),
                            mmio_entry.clicintctl,
                            false,
                        ));

                        /* trace!(
                            "MMIO QUEUE INTERRUPT {:x}",
                            ((addr - offset + 4u32 * i as u32 - 0x1000) / 4)
                        );*/
                        queue.push(
                            (addr - offset + 4u32 * i as u32 - 0x1000) / 4,
                            mmio_entry.clicintctl,
                        );
                    }
                    if mmio_entry.clicintie != 1 || mmio_entry.clicintip != 1 {
                        //dequeue self if pending or enabled status is 0

                        let entry_idx = (((addr - offset + 4u32 * i as u32).wrapping_sub(0x1000))
                            / 4)
                        .wrapping_sub(0x20);
                        if queue.remove(&entry_idx).is_some() {
                            history_entry.queue_op.push((
                                ((addr - offset + 4u32 * i as u32 - 0x1000) / 4),
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
                mmio_data = Some(self.read(addr as usize, data_size as usize, false, false));
            }
        } else if (0x5000..=0x500F).contains(&addr) {
            if we == 1 {
                mmio_data = Some(self.read(addr as usize, data_size as usize, false, false));
            } else if we == 2 {
                self.write(
                    addr as usize,
                    data_size as usize,
                    false,
                    SignalValue::Data(data),
                );
            }
        }
        mmio_data
    }

    fn csr_op(
        &self,
        csrstore: &mut HashMap<usize, usize>,
        history_entry: &mut CLICOp,
        queue: &mut PriorityQueue<u32, u8>,
        csr_ctl: SignalUnsigned,
        csr_data: SignalUnsigned,
        csr_addr: SignalUnsigned,
    ) -> u32 {
        // handle CSR operations
        // trace!("CSR OP");
        let mut val = 0;
        let mut csr_data = csr_data.clone();
        match csr_ctl {
            0 => {}
            //write
            1 => {
                if csrstore.contains_key(&(csr_addr as usize)) {
                    //mtvec write
                    if csr_addr == 0x305 {
                        csr_data |= 0b11; //hardwire to vectored mode
                    }
                    // if not mhartid, mhartid is RO
                    if csr_addr != 0xf14 {
                        val = *csrstore.get(&(csr_addr as usize)).unwrap();
                        if 0xB20 <= csr_addr && csr_addr <= 0xB39 {
                            csr_data = ((csr_data & (0b11100)) << 22)
                                | ((csr_data & 0b10) << 7)
                                | (csr_data & 0b1);
                            //val = ((val & (0b11100)) << 22) | ((val & 0b10) << 7) | (val & 0b1);
                        }

                        csrstore.insert(csr_addr as usize, csr_data as usize);
                        history_entry.csr_op = Some(vec![(csr_addr as usize, val as u32)]);
                    }
                    // interrupt config write, mirror in mmio
                    // trace!("CSR_ADDR_NEW:{:x}", csr_addr);
                    if 0xB20 <= csr_addr && csr_addr <= 0xB39 {
                        // trace!("ok do thing");
                        self.mmio_op(
                            0x1000 + ((csr_addr - 0xb20) * 4),
                            2,
                            4,
                            csr_data,
                            history_entry,
                            queue,
                            csrstore,
                        );
                    }
                }
            }
            //set
            2 => {
                if csrstore.contains_key(&(csr_addr as usize)) {
                    if csr_addr == 0x305 {
                        //mtvec set
                        csr_data |= 0b11; //hardwire to vectored mode
                    }
                    if csr_addr != 0xf14 {
                        //mhartid RO
                        val = *csrstore.get(&(csr_addr as usize)).unwrap();
                        if 0xB20 <= csr_addr && csr_addr <= 0xB39 {
                            csr_data = ((csr_data & (0b11100)) << 22)
                                | ((csr_data & 0b10) << 7)
                                | (csr_data & 0b1);
                            //val = ((val & (0b11100)) << 22) | ((val & 0b10) << 7) | (val & 0b1);
                        }
                        csrstore.insert(csr_addr as usize, (csr_data as usize) | val);
                        history_entry.csr_op = Some(vec![(csr_addr as usize, val as u32)]);

                        //interrupt config CSR
                        // trace!("SET CSR: {:x}, curr val: {:x}", csr_addr, val);
                        if 0xB20 <= csr_addr && csr_addr <= 0xB39 {
                            self.mmio_op(
                                0x1000 + ((csr_addr - 0xb20) * 4),
                                2,
                                4,
                                (csr_data) | val as u32,
                                history_entry,
                                queue,
                                csrstore,
                            );
                        }
                    }
                }
            }
            //clear
            3 => {
                // trace!("csr clear");
                if csrstore.contains_key(&(csr_addr as usize)) {
                    //  trace!("ADDR:{:x}", csr_addr);
                    if csr_addr == 0x305 {
                        //mtvec clear
                        csr_data |= !0b11; //hardwire to vectored mode
                    }
                    if csr_addr != 0xf14 {
                        //mhartid RO
                        val = *csrstore.get(&(csr_addr as usize)).unwrap();
                        //trace!("val:{:x}, csr_data:{:x}", val, csr_data);
                        // trace!("{:x}", (val as u32 & !csr_data));
                        if 0xB20 <= csr_addr && csr_addr <= 0xB39 {
                            csr_data = ((csr_data & (0b11100)) << 22)
                                | ((csr_data & 0b10) << 7)
                                | (csr_data & 0b1);
                            //  val = ((val & (0b11100)) << 22) | ((val & 0b10) << 7) | (val & 0b1);
                        }
                        csrstore.insert(csr_addr as usize, val & !(csr_data as usize));
                        history_entry.csr_op = Some(vec![(csr_addr as usize, val as u32)]);
                        if 0xB20 <= csr_addr && csr_addr <= 0xB39 {
                            self.mmio_op(
                                0x1000 + ((csr_addr - 0xb20) * 4),
                                2,
                                4,
                                (val as u32) & !csr_data,
                                history_entry,
                                queue,
                                csrstore,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        val as u32
    }

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
