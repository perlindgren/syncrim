use log::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, Range};
use std::{cell::RefCell, rc::Rc};
use syncrim::common::{
    Component, Input, InputPort, OutputType, Ports, Signal, SignalUnsigned, Simulator,
};

#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Reg {
    zero    = 0,    // Constant 0
    at      = 1,    // Reserved for assembler
    v0      = 2,    // Expression evaluation and results of function
    v1      = 3,    // Expression evaluation and results of function
    a0      = 4,    // Argument 1
    a1      = 5,    // Argument 2
    a2      = 6,    // Argument 3
    a3      = 7,    // Argument 4
    t0      = 8,    // Temporary (not preserved across calls)
    t1      = 9,    // Temporary (not preserved across calls)
    t2      = 10,   // Temporary (not preserved across calls)
    t3      = 11,   // Temporary (not preserved across calls)
    t4      = 12,   // Temporary (not preserved across calls)
    t5      = 13,   // Temporary (not preserved across calls)
    t6      = 14,   // Temporary (not preserved across calls)
    t7      = 15,   // Temporary (not preserved across calls)
    s0      = 16,   // Temporary (not preserved across calls)
    s1      = 17,   // Temporary (not preserved across calls)
    s2      = 18,   // Temporary (not preserved across calls)
    s3      = 19,   // Temporary (not preserved across calls)
    s4      = 20,   // Temporary (not preserved across calls)
    s5      = 21,   // Temporary (not preserved across calls)
    s6      = 22,   // Temporary (not preserved across calls)
    s7      = 23,   // Temporary (not preserved across calls)
    t8      = 24,   // Temporary (not preserved across calls)
    t9      = 25,   // Temporary (not preserved across calls)
    k0      = 26,   // Reserved for OS kernel
    k1      = 27,   // Reserved for OS kernel
    gp      = 28,   // Pointer to global area
    sp      = 29,   // Stack pointer
    fp      = 30,   // Frame pointer
    ra      = 31,   // Return address (used by function calls)
}

#[derive(Serialize, Deserialize)]
pub struct RegFile {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // ports
    pub read_addr1: InputPort,
    pub read_addr2: InputPort,
    pub write_data: InputPort,
    pub write_addr: InputPort,
    pub write_enable: InputPort,

    // data
    pub registers: RegStore,
    pub history: RegHistory,
}
impl RegFile {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        width: f32,
        height: f32,
        read_addr1: Input,
        read_addr2: Input,
        write_data: Input,
        write_addr: Input,
        write_enable: Input,
        registers: RegStore,
        history: RegHistory,
    ) -> Self {
        RegFile {
            id: id.to_string(),
            pos,
            width,
            height,
            read_addr1: InputPort {
                port_id: "read_addr1".to_string(),
                input: read_addr1,
            },
            read_addr2: InputPort {
                port_id: "read_addr2".to_string(),
                input: read_addr2,
            },
            write_data: InputPort {
                port_id: "write_data".to_string(),
                input: write_data,
            },
            write_addr: InputPort {
                port_id: "write_addr".to_string(),
                input: write_addr,
            },
            write_enable: InputPort {
                port_id: "write_enable".to_string(),
                input: write_enable,
            },
            registers,
            history,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegOp {
    read_addr1: u8,
    read_addr2: u8,
    write_addr2: Option<(u8, u32)>,
    old_data: Option<u8>,
}
// TODO: Perhaps we want registers to be of Signal type (containing potentially Signal::Unknown)

#[derive(Serialize, Deserialize, Clone)]
pub struct RegHistory(RefCell<Vec<RegOp>>);

impl RegHistory {
    pub fn new() -> Self {
        RegHistory(RefCell::new(Vec::new()))
    }
}

impl Default for RegHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegStore(pub Rc<RefCell<[u32; 32]>>);

impl RegStore {
    pub fn new() -> Self {
        RegStore(Rc::new(RefCell::new([0; 32])))
    }

    pub fn full_range() -> Range<u8> {
        Range { start: 0, end: 32 }
    }

    pub fn lo_range() -> Range<u8> {
        Range { start: 0, end: 16 }
    }

    pub fn hi_range() -> Range<u8> {
        Range { start: 16, end: 32 }
    }
}

impl Default for RegStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for RegStore {
    type Target = RefCell<[u32; 32]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RegFile {
    fn read_reg(&self, simulator: &Simulator, input: &Input) -> u32 {
        let read_addr: SignalUnsigned = simulator.get_input_val(input).try_into().unwrap();
        trace!("read_addr {}", read_addr);

        // mips always reads 0;
        if read_addr > 0 {
            self.registers.borrow()[read_addr as usize]
        } else {
            0
        }
    }
}

#[typetag::serde()]
impl Component for RegFile {
    fn to_(&self) {
        trace!("RegFile");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.read_addr1.clone(), self.read_addr2.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec!["reg_a".into(), "reg_b".into()],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        if simulator.get_input_val(&self.write_enable.input) == (true as SignalUnsigned).into() {
            let data = simulator.get_input_val(&self.write_data.input);
            trace!("data {:?}", data);
            let write_addr: SignalUnsigned = simulator
                .get_input_val(&self.write_addr.input)
                .try_into()
                .unwrap();
            trace!("write_addr {}", write_addr);
            self.registers.borrow_mut()[write_addr as usize] = data.try_into().unwrap();
        }

        // read after write
        let reg_value_a = self.read_reg(simulator, &self.read_addr1.input);
        trace!("reg_value {}", reg_value_a);
        simulator.set_out_val(&self.id, "reg_a", Signal::Data(reg_value_a));

        let reg_value_b = self.read_reg(simulator, &self.read_addr2.input);
        trace!("reg_value {}", reg_value_b);
        simulator.set_out_val(&self.id, "reg_b", Signal::Data(reg_value_b));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;
    use syncrim::{
        common::{ComponentStore, Input, Simulator},
        components::ProbeOut,
    };

    // an example of integration test for a mips specific component
    #[test]
    fn test_reg_file() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("read_reg_1")),
                Rc::new(ProbeOut::new("read_reg_2")),
                Rc::new(ProbeOut::new("write_data")),
                Rc::new(ProbeOut::new("write_addr")),
                Rc::new(ProbeOut::new("write_enable")),
                // regfile
                Rc::new(RegFile::new(
                    "reg_file",
                    (200.0, 150.0),
                    100.0,
                    150.0,
                    // ports
                    Input::new("read_reg_1", "out"),
                    Input::new("read_reg_2", "out"),
                    Input::new("write_data", "out"),
                    Input::new("write_addr", "out"),
                    Input::new("write_enable", "out"),
                    // data
                    RegStore::new(),
                    RegHistory::new(),
                )),
            ],
        };

        let mut simulator = Simulator::new(cs);

        assert_eq!(simulator.cycle, 1);

        // outputs
        let out_reg_1 = &Input::new("reg_file", "reg_a");
        let out_reg_2 = &Input::new("reg_file", "reg_b");

        // reset
        assert_eq!(simulator.get_input_val(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_val(out_reg_2), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_val("read_reg_1", "out", 0);
        simulator.set_out_val("read_reg_2", "out", 1);
        simulator.set_out_val("write_data", "out", 1337);
        simulator.set_out_val("write_addr", "out", 1);
        simulator.set_out_val("write_enable", "out", true as SignalUnsigned);

        // test write and read to reg # 1 in same cycle
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_val(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_val(out_reg_2), 1337.into());

        // test write and read to reg # 0 in same cycle (red #0 should always read 0)
        println!("<setup for clock 3>");
        simulator.set_out_val("read_reg_1", "out", 0);
        simulator.set_out_val("read_reg_2", "out", 1);
        simulator.set_out_val("write_data", "out", 42);
        simulator.set_out_val("write_addr", "out", 0);
        simulator.set_out_val("write_enable", "out", true as SignalUnsigned);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_val(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_val(out_reg_2), 1337.into());
    }
}
