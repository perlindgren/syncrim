use log::*;
use serde::{Deserialize, Serialize};
use std::{any::Any, cell::RefCell};
use syncrim::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator},
    signal::SignalValue,
};

pub const IO_REGISTER_SELECT_IN_ID: &str = "io_reg_select_in";
pub const IO_DATA_IN_ID: &str = "io_data_in";
pub const IO_WRITE_ENABLE_IN: &str = "io_write_enable_in";
pub const IO_READ_ENABLE_IN: &str = "io_read_enable_in";

pub const IO_DATA_OUT_ID: &str = "io_data_out";
pub const IO_INTERRUPT_OUT_ID: &str = "io_interrupt_out";

/// # IO component
/// This compote provides IO functionality with output buffer and input buffer.
/// At reg select 0, the input control register is available, this register contains the following flags
/// - 0b01 this register denotes if there is input data available to be read *
/// - 0b10, interrupt control bit, when set and input is received the interrupt line will be set to 0x1
///
/// \* this bit is read only \
/// Reg select 1, is the input buffer, when read this will return the first u8 available,
/// it will also set input bit to 0 and interrupt to zero when the buffer becomes empty.
/// If empty the line output stays the same as before  \
/// Reg select 2, is the output buffer, data written to this ins truncated to a u8,
/// when using egui this is rendered as an utf8 string
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MipsIO {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) address_in: Input,
    pub(crate) data_in: Input,
    pub(crate) we_in: Input,
    pub(crate) re_in: Input,

    #[serde(skip)]
    pub data: RefCell<MipsIOData>,

    // used in egui to determine if the input/output window should show
    #[cfg(feature = "gui-egui")]
    #[serde(skip)]
    pub gui_show: RefCell<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct MipsIOData {
    pub interrupt: bool,
    pub input_control: u32,
    // vec of (cycle, amount_data_written)
    pub key_buff_write_history: Vec<(usize, usize)>,
    pub read_pos: usize,
    pub end_pos: usize,
    pub key_buff: Vec<u8>,
    pub out_buff: Vec<u8>,
    // (cycle, state before that cycle was clocked), only pushed if the clock changed the state
    history: Vec<(usize, IOState)>,
}

// the state changed by clock, key_buff and key_buff_write_history are inputs and are kept
#[derive(Debug, Clone, PartialEq)]
struct IOState {
    interrupt: bool,
    input_control: u32,
    read_pos: usize,
    end_pos: usize,
    // out_buff is only pushed to, so its length is enough to restore it
    out_len: usize,
}

impl MipsIOData {
    fn state(&self) -> IOState {
        IOState {
            interrupt: self.interrupt,
            input_control: self.input_control,
            read_pos: self.read_pos,
            end_pos: self.end_pos,
            out_len: self.out_buff.len(),
        }
    }

    fn restore(&mut self, state: IOState) {
        self.interrupt = state.interrupt;
        self.input_control = state.input_control;
        self.read_pos = state.read_pos;
        self.end_pos = state.end_pos;
        self.out_buff.truncate(state.out_len);
    }
}

#[typetag::serde]
impl Component for MipsIO {
    fn to_(&self) {
        trace!("mips_io");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: IO_REGISTER_SELECT_IN_ID.to_string(),
                        input: self.address_in.clone(),
                    },
                    &InputPort {
                        port_id: IO_DATA_IN_ID.to_string(),
                        input: self.data_in.clone(),
                    },
                    &InputPort {
                        port_id: IO_WRITE_ENABLE_IN.to_string(),
                        input: self.we_in.clone(),
                    },
                    &InputPort {
                        port_id: IO_READ_ENABLE_IN.to_string(),
                        input: self.re_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![IO_DATA_OUT_ID, IO_INTERRUPT_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            IO_REGISTER_SELECT_IN_ID => self.address_in = new_input,
            IO_DATA_IN_ID => self.data_in = new_input,
            IO_WRITE_ENABLE_IN => self.we_in = new_input,
            IO_READ_ENABLE_IN => self.re_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // check if read and write is high, return error
        if matches!(
            (
                simulator.get_input_value(&self.we_in),
                simulator.get_input_value(&self.re_in)
            ),
            (SignalValue::Data(1), SignalValue::Data(1))
        ) {
            return Err(Condition::Error(
                "can't have read and write signal at the same time".to_string(),
            ));
        };

        let mut data = self.data.borrow_mut();
        let state_before = data.state();
        let mut ret: Result<(), Condition> = Ok(());

        // if we have new data
        if let Some((_, new_data_amount)) = data
            .key_buff_write_history
            .iter()
            .find(|(cycle, _)| *cycle == simulator.cycle)
            .cloned()
        {
            data.end_pos += new_data_amount;
            data.input_control |= 0b1;
            // if interrupt flag is set, cause an interrupt
            if data.input_control & 0b10 == 0b10 {
                data.interrupt = true;
            }
        }

        // if write enable
        if simulator.get_input_value(&self.we_in) == SignalValue::Data(0x1) {
            // get the data component data ref
            // if data is valid
            if let SignalValue::Data(in_data) = simulator.get_input_value(&self.data_in) {
                // the register/address to write data at
                match simulator.get_input_value(&self.address_in) {
                    SignalValue::Data(0) => {
                        data.input_control = in_data & 0xFFFF_FFFE | data.input_control & 0x1
                    }
                    SignalValue::Data(2) => {
                        data.out_buff.push(in_data as u8);
                    }
                    // if the address dont exist in our IO part
                    SignalValue::Data(_) => {
                        ret = Err(Condition::Warning("Write address out of range".to_string()))
                    }
                    SignalValue::DontCare => {}
                    // if signal is uninitialized or unknown return error
                    _ => {
                        ret = Err(Condition::Error(
                            "Address is uninitialized or unknown".to_string(),
                        ))
                    }
                }
            } else {
                // trying to write a signal that isn't data
                ret = Err(Condition::Error(
                    "Not valid data to write, SignalValue is not data".to_string(),
                ))
            }
        }

        // if read enable
        if simulator.get_input_value(&self.re_in) == SignalValue::Data(0x1) {
            // the register/address to read at
            match simulator.get_input_value(&self.address_in) {
                SignalValue::Data(0x0) => {
                    simulator.set_out_value(&self.id, IO_DATA_OUT_ID, data.input_control);
                }
                SignalValue::Data(0x1) => {
                    if data.read_pos < data.end_pos {
                        simulator.set_out_value(
                            &self.id,
                            IO_DATA_OUT_ID,
                            data.key_buff[data.read_pos] as u32,
                        );
                        data.read_pos += 1;

                        // if we have read all the data
                        if data.read_pos == data.end_pos {
                            data.interrupt = false;
                            // clear data available flag
                            data.input_control &= 0xFFFF_FFFE
                        }
                    }
                }
                _ => {}
            }
        }

        // set the interrupt signal
        simulator.set_out_value(&self.id, IO_INTERRUPT_OUT_ID, data.interrupt);

        // save the state before this cycle, used for unclock
        if data.state() != state_before {
            data.history.push((simulator.cycle, state_before));
        }

        ret
    }

    fn un_clock(&self, sim: &Simulator) {
        let mut data = self.data.borrow_mut();
        // +1 since cycle increased after clock, but not decreased before unclock
        if data
            .history
            .last()
            .is_some_and(|(cycle, _)| cycle + 1 == sim.cycle)
        {
            let (_, state) = data.history.pop().unwrap();
            data.restore(state);
        }
    }

    fn reset(&self) {
        *self.data.borrow_mut() = MipsIOData::default()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MipsIO {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        address_in: Input,
        data_in: Input,
        write_enable_in: Input,
        read_enable_in: Input,
    ) -> Self {
        MipsIO {
            id: id.to_string(),
            pos,
            address_in,
            data_in,
            we_in: write_enable_in,
            re_in: read_enable_in,
            data: Default::default(),
            #[cfg(feature = "gui-egui")]
            gui_show: RefCell::new(false),
        }
    }
}

#[cfg(test)]
mod unclock_test {
    use super::*;
    use crate::components::test_utils::*;
    use std::rc::Rc;
    use SignalValue::Data;

    const CONTROL: u32 = 0;
    const KEY: u32 = 1;
    const OUT: u32 = 2;
    const IE: u32 = 0b10;

    type Snapshot = (
        bool,
        u32,
        usize,
        usize,
        Vec<u8>,
        usize,
        SignalValue,
        SignalValue,
    );

    fn snapshot(sim: &Simulator) -> Snapshot {
        let d = get_component::<MipsIO>(sim).data.borrow();
        (
            d.interrupt,
            d.input_control,
            d.read_pos,
            d.end_pos,
            d.out_buff.clone(),
            d.history.len(),
            sim.get_input_value(&Input::new("io", IO_INTERRUPT_OUT_ID)),
            sim.get_input_value(&Input::new("io", IO_DATA_OUT_ID)),
        )
    }

    // keys are (cycle, bytes), available during the clock of that cycle, same as the gui does
    fn sim(script: &[(usize, &'static str, SignalValue)], keys: &[(usize, &[u8])]) -> Simulator {
        let driver = TestDriver {
            outputs: ["we", "re", "adr", "din"]
                .iter()
                .map(|p| (*p, Data(0)))
                .collect(),
            script: script.to_vec(),
        };
        let i = |port| Input::new(DRIVER_ID, port);
        let s = test_sim(
            driver,
            vec![Rc::new(MipsIO::new(
                "io",
                (0.0, 0.0),
                i("adr"),
                i("din"),
                i("we"),
                i("re"),
            ))],
        );
        {
            let mut d = get_component::<MipsIO>(&s).data.borrow_mut();
            for (cycle, bytes) in keys {
                d.key_buff.extend_from_slice(bytes);
                d.key_buff_write_history.push((*cycle, bytes.len()));
            }
        }
        s
    }

    fn write(cycle: usize, adr: u32, data: SignalValue) -> [(usize, &'static str, SignalValue); 3] {
        [
            (cycle, "we", Data(1)),
            (cycle, "adr", Data(adr)),
            (cycle, "din", data),
        ]
    }

    fn read(cycle: usize, adr: u32) -> [(usize, &'static str, SignalValue); 2] {
        [(cycle, "re", Data(1)), (cycle, "adr", Data(adr))]
    }

    #[test]
    fn unclock_output() {
        let script = [
            write(1, OUT, Data(b'h' as u32)),
            write(2, OUT, Data(b'i' as u32)),
            // control write must not touch the out buffer
            write(3, CONTROL, Data(IE)),
            write(5, OUT, Data(b'!' as u32)),
        ]
        .concat();
        {
            let mut s = sim(&script, &[]);
            for _ in 0..5 {
                s.clock();
            }
            assert_eq!(get_component::<MipsIO>(&s).data.borrow().out_buff, b"hi!");
        }
        assert_round_trip(sim(&script, &[]), 10, snapshot);
    }

    #[test]
    fn unclock_polling() {
        // poll control and key registers while there is no data, then when there is
        let script: Vec<_> = (1..40)
            .flat_map(|c| read(c, if c % 3 == 0 { KEY } else { CONTROL }))
            .collect();
        assert_round_trip(sim(&script, &[(10, b"ab"), (25, b"c")]), 50, snapshot);
    }

    #[test]
    fn unclock_interrupt() {
        let script = [
            write(1, CONTROL, Data(IE)).to_vec(),
            // drain the first key input
            read(8, KEY).to_vec(),
            read(9, KEY).to_vec(),
            // read with nothing left
            read(10, KEY).to_vec(),
            // key arrives the same cycle it is read
            read(15, KEY).to_vec(),
            // disable interrupt, then key arrives
            write(18, CONTROL, Data(0)).to_vec(),
            read(22, KEY).to_vec(),
        ]
        .concat();
        let keys: &[(usize, &[u8])] = &[(5, b"xy"), (15, b"z"), (20, b"w")];
        {
            let mut s = sim(&script, keys);
            for _ in 0..5 {
                s.clock();
            }
            assert_eq!(
                s.get_input_value(&Input::new("io", IO_INTERRUPT_OUT_ID)),
                Data(1)
            );
        }
        assert_round_trip(sim(&script, keys), 30, snapshot);
    }

    #[test]
    fn unclock_bad_writes() {
        // these return conditions and must not change or undo anything
        let script = [
            write(1, OUT, Data(b'a' as u32)),
            write(2, 7, Data(1)),
            write(3, OUT, SignalValue::Unknown),
            write(4, CONTROL, Data(IE)),
            // read and write at the same time
            [
                (5, "we", Data(1)),
                (5, "re", Data(1)),
                (5, "adr", Data(OUT)),
            ],
            write(6, OUT, Data(b'b' as u32)),
        ]
        .concat();
        assert_round_trip(sim(&script, &[(4, b"k")]), 10, snapshot);
    }
}
