use bitvec::vec::BitVec;
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

const RESET_OVERFLOW: bool = false;
const RESET_COMPARE: bool = true;

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
    // since we do data.flag |= x, we cant reverse the operation
    // so we need to keep track of their history
    overflow_fg_history: BitVec,
    compare1_fg_history: BitVec,
    reset_cause_history: BitVec,
}
impl Default for MipsTimerData {
    fn default() -> Self {
        Self {
            flags: 0,
            counter: 0,
            compare: 0,
            divider: 16,
            div_counter: 0,
            overflow_fg_history: BitVec::default(),
            compare1_fg_history: BitVec::default(),
            reset_cause_history: BitVec::default(),
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
                    // push old OVERFLOW_FG
                    let old = data.flags & OVERFLOW_FG == OVERFLOW_FG;
                    data.reset_cause_history.push(RESET_OVERFLOW);
                    data.overflow_fg_history.push(old);
                    data.flags |= OVERFLOW_FG
                }
                // reset timer when compare is reached
                // +1 for syncsim compatibility
                else if (data.counter == data.compare.wrapping_add(1))
                    && data.flags & COMPARE1_CR == COMPARE1_CR
                {
                    // push old OVERFLOW_FG
                    // this is because unclock don't know how we got to counter = 0
                    // could have been by overflow OR by counter reset
                    // also why we have this in else clause
                    // so it don't push old state twice
                    let old = data.flags & OVERFLOW_FG == OVERFLOW_FG;
                    data.overflow_fg_history.push(old);
                    data.reset_cause_history.push(RESET_COMPARE);
                    data.counter = 0
                }

                // if our compare matches
                if data.counter == data.compare {
                    // push old COMPARE1_FG
                    let old = data.flags & COMPARE1_FG == COMPARE1_FG;
                    data.compare1_fg_history.push(old);
                    data.flags |= COMPARE1_FG
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
                    // write old value to out, used for unclock
                    match simulator.get_input_value(&self.address_in) {
                        SignalValue::Data(0xFFFF_0010) => {
                            simulator.set_out_value(&self.id, TIMER_DATA_OUT_ID, data.flags as u32);
                            data.flags = in_data as u8
                        }
                        SignalValue::Data(0xFFFF_0014) => {
                            simulator.set_out_value(&self.id, TIMER_DATA_OUT_ID, data.counter);
                            data.counter = in_data
                        }
                        SignalValue::Data(0xFFFF_0018) => {
                            simulator.set_out_value(&self.id, TIMER_DATA_OUT_ID, data.compare);
                            data.compare = in_data
                        }
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

    fn un_clock(&self, sim: &Simulator) {
        let mut data = self.data.borrow_mut();

        let data_out = Input::new(&self.id, TIMER_DATA_OUT_ID);
        // undo write
        // Read write enable, if 0x1 write data, if 0x0 read data, other dont do anything
        match sim.get_input_value(&self.we_in) {
            // write line is zero
            SignalValue::Data(0x0) => {}
            // write enable write data, only if data was valid, same as in clock
            // otherwise nothing was written and data out holds a stale value
            SignalValue::Data(_)
                if matches!(sim.get_input_value(&self.data_in), SignalValue::Data(_)) =>
            {
                // get the old value (is written to out when we is high)
                if let SignalValue::Data(out_data) = sim.get_input_value(&data_out) {
                    // set data according to address
                    // write old value to out, used for unclock
                    match sim.get_input_value(&self.address_in) {
                        SignalValue::Data(0xFFFF_0010) => data.flags = out_data as u8,
                        SignalValue::Data(0xFFFF_0014) => data.counter = out_data,
                        SignalValue::Data(0xFFFF_0018) => data.compare = out_data,
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        // undo timer clocking
        if (data.flags & COUNTER_ENABLE) == COUNTER_ENABLE {
            // if we try to decrease a divider which is at zero
            let (previous_divider, overflow) = data.div_counter.overflowing_sub(1);
            data.div_counter = previous_divider;

            if overflow {
                // same check as in clock, done before we undo the counter step
                let compare_matched = data.counter == data.compare;

                // -1 since never are at data.divider since its set to 0 if so
                data.div_counter = data.divider - 1;

                let (previous_counter, overflow) = data.counter.overflowing_sub(1);
                data.counter = previous_counter;

                if overflow {
                    // if we had an overflow on previous cycle we know that history is not empty so unwrap is safe
                    let previous_overflow = data.overflow_fg_history.pop().unwrap();
                    // clear overflow bit
                    data.flags &= !OVERFLOW_FG;
                    // set bit if the previous state was set
                    if previous_overflow {
                        data.flags |= OVERFLOW_FG;
                    }

                    // if the cause for counter being zero
                    // unwrap is safe since we push when we reset to zero
                    if data.reset_cause_history.pop().unwrap() == RESET_COMPARE {
                        data.counter = data.compare
                    }
                }
                // if compare set the bit this cycle
                // set our old bit
                if compare_matched {
                    let previous_compare = data.compare1_fg_history.pop().unwrap();
                    // clear overflow bit
                    data.flags &= !COMPARE1_FG;
                    // set bit if the previous state was set
                    if previous_compare {
                        data.flags |= COMPARE1_FG;
                    }
                }

                // no need to deal with compare, since it only matters when going forward
            }
        }
    }

    fn reset(&self) {
        *self.data.borrow_mut() = MipsTimerData {
            flags: 0,
            counter: 0,
            compare: 0,
            divider: 16,
            div_counter: 0,
            overflow_fg_history: BitVec::default(),
            compare1_fg_history: BitVec::default(),
            reset_cause_history: BitVec::default(),
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
                overflow_fg_history: BitVec::default(),
                compare1_fg_history: BitVec::default(),
                reset_cause_history: BitVec::default(),
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

#[cfg(test)]
mod unclock_test {
    use super::*;
    use crate::components::test_utils::*;
    use std::rc::Rc;
    use syncrim::common::Simulator;

    type Snapshot = (u8, u32, u32, u32, usize, usize, usize, SignalValue);

    fn snapshot(sim: &Simulator) -> Snapshot {
        let d = get_component::<MipsTimer>(sim).data.borrow();
        (
            d.flags,
            d.counter,
            d.compare,
            d.div_counter,
            d.overflow_fg_history.len(),
            d.compare1_fg_history.len(),
            d.reset_cause_history.len(),
            sim.get_input_value(&Input::new("timer", TIMER_INTERRUPT_OUT_ID)),
        )
    }

    // we is high on the scripted cycles, (cycle, address, data)
    fn assert_round_trip(script: &[(usize, u32, SignalValue)], cycles: usize) {
        let driver = TestDriver {
            outputs: vec![
                ("we", SignalValue::Data(0)),
                ("adrs", SignalValue::Data(0)),
                ("data", SignalValue::Data(0)),
            ],
            script: script
                .iter()
                .flat_map(|&(cycle, adrs, data)| {
                    [
                        (cycle, "we", SignalValue::Data(1)),
                        (cycle, "adrs", SignalValue::Data(adrs)),
                        (cycle, "data", data),
                    ]
                })
                .collect(),
        };
        let sim = test_sim(
            driver,
            vec![Rc::new(MipsTimer::new(
                "timer",
                (0.0, 0.0),
                Input::new(DRIVER_ID, "adrs"),
                Input::new(DRIVER_ID, "data"),
                Input::new(DRIVER_ID, "we"),
            ))],
        );
        crate::components::test_utils::assert_round_trip(sim, cycles, snapshot);
    }

    const EN: u32 = COUNTER_ENABLE as u32;
    const CIE: u32 = COMPARE1_IE as u32;
    const CR: u32 = COMPARE1_CR as u32;
    const FLAGS: u32 = 0xFFFF_0010;
    const COUNT: u32 = 0xFFFF_0014;
    const COMPARE: u32 = 0xFFFF_0018;
    use SignalValue::Data;

    // Simulator::new clocks once, so scripts start at cycle 1
    #[test]
    fn unclock_compare_zero() {
        assert_round_trip(&[(1, FLAGS, Data(EN))], 100);
        assert_round_trip(&[(1, FLAGS, Data(EN | CR))], 100);
    }

    #[test]
    fn unclock_compare() {
        assert_round_trip(&[(1, COMPARE, Data(5)), (2, FLAGS, Data(EN | CIE))], 200);
        assert_round_trip(
            &[(1, COMPARE, Data(3)), (2, FLAGS, Data(EN | CIE | CR))],
            200,
        );
    }

    #[test]
    fn unclock_overflow() {
        let script = [
            (1, COMPARE, Data(7)),
            (2, COUNT, Data(0xFFFF_FFFE)),
            (3, FLAGS, Data(EN)),
        ];
        assert_round_trip(&script, 100);
    }

    #[test]
    fn unclock_compare_max() {
        assert_round_trip(
            &[(1, COMPARE, Data(u32::MAX)), (2, FLAGS, Data(EN | CR))],
            40,
        );
        let script = [
            (1, COMPARE, Data(u32::MAX)),
            (2, COUNT, Data(0xFFFF_FFFD)),
            (3, FLAGS, Data(EN | CIE)),
        ];
        assert_round_trip(&script, 100);
    }

    #[test]
    fn unclock_writes() {
        // clear flags and change count while running
        let script = [
            (1, COMPARE, Data(2)),
            (2, FLAGS, Data(EN | CIE | CR)),
            (60, FLAGS, Data(EN | CIE | CR)),
            (61, COUNT, Data(1)),
        ];
        assert_round_trip(&script, 150);
        // move compare below count
        let script = [
            (1, COMPARE, Data(9)),
            (2, FLAGS, Data(EN | CR)),
            (80, COMPARE, Data(2)),
        ];
        assert_round_trip(&script, 150);
    }

    #[test]
    fn unclock_invalid_write() {
        // the invalid write is not done in clock, so it must not be undone either
        let script = [
            (1, COUNT, Data(7)),
            (2, COUNT, SignalValue::Unknown),
            (3, FLAGS, Data(EN)),
            (20, FLAGS, SignalValue::Uninitialized),
        ];
        assert_round_trip(&script, 60);
    }
}
