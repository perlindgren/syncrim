// use std::fmt::Alignment;
use log::*;
use serde::{Deserialize, Serialize};
use std::{any::Any, cell::RefCell};
use syncrim::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator},
    signal::SignalValue,
};

pub const TIMER_ADDRESS_IN_ID: &str = "timer_address_in";
pub const TIMER_DATA_IN_ID: &str = "timer_data_in";
pub const TIMER_WRITE_ENABLE: &str = "timer_write_enable_in";

pub const TIMER_DATA_OUT_ID: &str = "timer_data_out";
pub const TIMER_INTERRUPT_OUT_ID: &str = "timer_interrupt_out";

const COUNTER_ENABLE: u8 = 0b0000_0001;
const OVERFLOW_IE: u8 = 0b0000_0010;
const OVERFLOW_FG: u8 = 0b0000_0100;
const COMPARE1_IE: u8 = 0b0000_1000;
const COMPARE1_FG: u8 = 0b0001_0000;
const COMPARE1_CR: u8 = 0b0010_0000;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MipsTimer {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) address_in: Input,
    pub(crate) data_in: Input,
    pub(crate) we_in: Input,

    #[serde(skip)]
    pub data: RefCell<MipsTimerData>,
}

#[derive(Clone, Debug)]
pub struct MipsTimerData {
    // 1 = counter enabled, 2 = overflow_ie = 2, 4 overflow_FG,
    // 8 = compare1_IE, 16 = compare1_FG, 32 = compare1_CR
    pub flags: u8,
    pub counter: u32,
    pub compare: u32,
    pub divider: u32,
    pub div_counter: u32,
}
impl Default for MipsTimerData {
    fn default() -> Self {
        Self {
            flags: 0,
            counter: 0,
            compare: 0,
            divider: 16,
            div_counter: 0,
        }
    }
}

#[typetag::serde]
impl Component for MipsTimer {
    fn to_(&self) {
        trace!("mips_timer");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: TIMER_ADDRESS_IN_ID.to_string(),
                        input: self.address_in.clone(),
                    },
                    &InputPort {
                        port_id: TIMER_DATA_IN_ID.to_string(),
                        input: self.data_in.clone(),
                    },
                    &InputPort {
                        port_id: TIMER_WRITE_ENABLE.to_string(),
                        input: self.we_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![TIMER_DATA_OUT_ID, TIMER_INTERRUPT_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            TIMER_ADDRESS_IN_ID => self.address_in = new_input,
            TIMER_DATA_IN_ID => self.data_in = new_input,
            TIMER_WRITE_ENABLE => self.we_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // from lab 3a we have this information for regarding hardware addresses
        //
        // IO component address 0xFFFF_0000
        // +8 IO out
        //
        //
        // timer address 0xFFFF_0010
        // +0 Timer Control Register
        // +4 Timer Count Register
        // +8 Timer Compare Register
        let mut data = self.data.borrow_mut();
        if data.flags & COUNTER_ENABLE == COUNTER_ENABLE {
            // inc our dive counter so there is only one counter inc per divider amount of cycles
            data.div_counter = data.div_counter.wrapping_add(1);

            if data.div_counter >= data.divider {
                data.div_counter = 0;

                data.counter = data.counter.wrapping_add(1);

                // if overflow set the overflow_fg flag
                if data.counter == 0 {
                    data.flags |= OVERFLOW_FG
                }
                // if our compare matches
                if data.counter == data.compare {
                    data.flags |= COMPARE1_FG
                }

                // reset timer when compare is reached
                // +1 for syncsim compatibility
                if (data.counter == data.compare + 1) && data.flags & COMPARE1_CR == COMPARE1_CR {
                    data.counter = 0
                }
            }
        }

        // NOTE addresses are designed for mips there is the possibilty for
        // the addresses to be modified by the mmu so that different values
        // could be used so that the timer component becomes more generic
        let mut ret: Result<(), Condition> = Ok(());

        // Read write enable, if 0x1 write data, if 0x0 read data, other dont do anything
        match simulator.get_input_value(&self.we_in) {
            // write line is zero
            SignalValue::Data(0x0) => match simulator.get_input_value(&self.address_in) {
                SignalValue::Data(0xFFFF_0010) => {
                    simulator.set_out_value(&self.id, TIMER_DATA_OUT_ID, data.flags as u32)
                }
                SignalValue::Data(0xFFFF_0014) => {
                    simulator.set_out_value(&self.id, TIMER_DATA_OUT_ID, data.counter)
                }
                SignalValue::Data(0xFFFF_0018) => {
                    simulator.set_out_value(&self.id, TIMER_DATA_OUT_ID, data.compare)
                }
                _ => {}
            },
            // write enable write data
            SignalValue::Data(_) => {
                // if data is valid, aka not undefined or unset
                if let SignalValue::Data(in_data) = simulator.get_input_value(&self.data_in) {
                    // set data according to address
                    match simulator.get_input_value(&self.address_in) {
                        SignalValue::Data(0xFFFF_0010) => data.flags = in_data as u8,
                        SignalValue::Data(0xFFFF_0014) => data.counter = in_data,
                        SignalValue::Data(0xFFFF_0018) => data.compare = in_data,
                        SignalValue::Data(_) => {
                            ret = Err(Condition::Warning("Write address out of range".to_string()))
                        }
                        SignalValue::DontCare => {}
                        _ => {
                            ret = Err(Condition::Error(
                                "Address is uninitialized or unknown".to_string(),
                            ))
                        }
                    }
                } else {
                    ret = Err(Condition::Error(
                        "Not valid data to write, SignalValue is not data".to_string(),
                    ));
                };
            }
            _ => {}
        };

        // if flag overflow_ie is set and overflow occurred according to overflow_fg flag
        // or
        // if flag compare_ie is set and compare happened according to compare_fg flag
        // set overflow out signal
        simulator.set_out_value(
            &self.id,
            TIMER_INTERRUPT_OUT_ID,
            if data.flags & (OVERFLOW_FG | OVERFLOW_IE) == (OVERFLOW_FG | OVERFLOW_IE)
                || data.flags & (COMPARE1_FG | COMPARE1_IE) == (COMPARE1_FG | COMPARE1_IE)
            {
                1
            } else {
                0
            },
        );

        ret
    }
    fn reset(&self) {
        *self.data.borrow_mut() = MipsTimerData {
            flags: 0,
            counter: 0,
            compare: 0,
            divider: 16,
            div_counter: 0,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MipsTimer {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        address_in: Input,
        data_in: Input,
        write_enable_in: Input,
    ) -> Self {
        MipsTimer {
            id: id.to_string(),
            pos,
            address_in,
            data_in,
            we_in: write_enable_in,
            data: RefCell::new(MipsTimerData {
                flags: 0,
                counter: 0,
                compare: 0,
                divider: 16,
                div_counter: 0,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::rc::Rc;
    use syncrim::{
        common::{ComponentStore, Input, RunningState, Simulator},
        components::ProbeOut,
    };

    fn create_test_sim() -> Simulator {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("adrs")),
                Rc::new(ProbeOut::new("data")),
                Rc::new(ProbeOut::new("we")),
                Rc::new(MipsTimer::new(
                    "timer",
                    (0.0, 0.0),
                    Input::new("adrs", "out"),
                    Input::new("data", "out"),
                    Input::new("we", "out"),
                )),
            ],
        };
        let simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);

        simulator
    }

    #[test]
    fn test_read_write() {
        let mut s = create_test_sim();
        let out_data = Input::new("timer", TIMER_DATA_OUT_ID);

        // Set address to 0xffff_0014, address of count register
        s.set_out_value("adrs", "out", 0xffff_0014);
        // set data to something
        s.set_out_value("data", "out", 42);
        // set we to true
        s.set_out_value("we", "out", 1);

        // clock the simulation
        s.clock();

        // set address to 0xffff_0018, compare register, and data to 420
        s.set_out_value("adrs", "out", 0xffff_0018);
        s.set_out_value("data", "out", 1337);
        s.clock();

        s.set_out_value("adrs", "out", 0xffff_0010);
        s.set_out_value("data", "out", 63);
        s.clock();

        // test if write was successful
        {
            let timer: &MipsTimer = s.ordered_components[3].as_any().downcast_ref().unwrap();
            assert_eq!(timer.data.borrow().counter, 42);
            assert_eq!(timer.data.borrow().compare, 1337);
            assert_eq!(timer.data.borrow().flags, 63);
        }

        // test reads, address 0xffff_0018: flags already set, set we to false
        s.set_out_value("we", "out", 0);
        s.clock();

        // see if out put is same as flags
        assert_eq!(s.get_input_value(&out_data), SignalValue::Data(63));

        // set address to 0xffff_0014, count register
        s.set_out_value("adrs", "out", 0xffff_0014);
        s.clock();
        // since it haven't passed 16 cycles since timer enable, count wont have increased
        assert_eq!(s.get_input_value(&out_data), SignalValue::Data(42));

        // set address to 0xffff_0018, compare register
        s.set_out_value("adrs", "out", 0xffff_0018);
        s.clock();
        // since it haven't passed 16 cycles since timer enable, count wont have increased
        assert_eq!(s.get_input_value(&out_data), SignalValue::Data(1337));
    }

    #[test]
    fn test_interrupt() {
        let mut s = create_test_sim();
        let out_data = Input::new("timer", TIMER_DATA_OUT_ID);
        let out_interrupt = Input::new("timer", TIMER_INTERRUPT_OUT_ID);

        // set we to true
        s.set_out_value("we", "out", 1);

        // set address to 0xffff_0018, compare register, and data to 420
        s.set_out_value("adrs", "out", 0xffff_0018);
        s.set_out_value("data", "out", 10);
        s.clock();
        assert!(s.component_condition.is_empty());

        // Enable timer(1) and compare interrupt(8) as well as the compare reset flag(32)
        s.set_out_value("adrs", "out", 0xffff_0010);
        s.set_out_value("data", "out", 0b101001);
        s.clock();

        // disable write, and set address to count register
        s.set_out_value("we", "out", 0);
        s.set_out_value("adrs", "out", 0xffff_0014);

        // incresa our count 10 times
        for i in 1..10 {
            // divider defaults to 16, which means that for each step of count we ned 16 clock cycles
            for _ in 0..16 {
                s.clock();
            }
            // read if count is correct, and there is interrupt
            assert_eq!(s.get_input_value(&out_data), SignalValue::Data(i));
            assert_eq!(s.get_input_value(&out_interrupt), SignalValue::Data(0),);
        }

        for _ in 0..16 {
            s.clock();
        }
        // read if count is correct
        // test if interrupt signal is active
        assert_eq!(s.get_input_value(&out_data), SignalValue::Data(10));
        assert_eq!(s.get_input_value(&out_interrupt), SignalValue::Data(1));

        // test if reset flag works
        for _ in 0..16 {
            s.clock();
        }
        assert_eq!(s.get_input_value(&out_data), SignalValue::Data(0));
    }

    #[test]
    fn test_return_condition() {
        let mut s = create_test_sim();

        // set address to 0xffff_abcd,
        s.set_out_value("we", "out", 1);
        s.set_out_value("adrs", "out", 0xffff_abcd);
        s.clock();

        assert!(s.component_condition.iter().any(|(id, cond)| id == "timer"
            && cond == &Condition::Warning("Write address out of range".to_string())));

        // test if Address of is not Data or Don't care
        s.set_out_value("adrs", "out", SignalValue::Uninitialized);
        s.clock();

        assert!(s.component_condition.iter().any(|(id, cond)| id == "timer"
            && cond == &Condition::Error("Address is uninitialized or unknown".to_string())));

        // set running state to stopped, so we can ignore that we had an error (this took 2h to debug)
        s.running_state = RunningState::Stopped;

        s.set_out_value("adrs", "out", SignalValue::Data(0xFFFF_0014));
        s.set_out_value("data", "out", SignalValue::Unknown);

        s.clock();
        assert_eq!(
            s.component_condition.iter().find(|(id, _)| id == "timer"),
            Some(&(
                "timer".to_string(),
                Condition::Error("Not valid data to write, SignalValue is not data".to_string())
            ))
        );
    }
}
