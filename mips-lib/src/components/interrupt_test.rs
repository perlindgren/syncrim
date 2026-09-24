// Timer and IO interrupt lines wired to CP0 in one simulation,
// checks that clocking, unclocking and replaying an interrupt driven run gives the same result
use crate::components::test_utils::*;
use crate::components::*;
use std::rc::Rc;
use syncrim::{
    common::{Input, Simulator},
    signal::SignalValue::{self, Data},
};

const TIMER_FLAGS: u32 = 0xFFFF_0010;
const TIMER_COMPARE: u32 = 0xFFFF_0018;
// counter enable, compare interrupt enable, compare reset
const TIMER_EN_CIE_CR: u32 = 0b10_1001;
const IO_CONTROL: u32 = 0;
const IO_KEY: u32 = 1;
const IO_IE: u32 = 0b10;
// interrupt enable, timer mask and io mask
const SR_ALL: u32 = 0xC01;

type Script = Vec<(usize, &'static str, SignalValue)>;

fn timer_write(cycle: usize, adr: u32, data: u32) -> Script {
    vec![
        (cycle, "t_we", Data(1)),
        (cycle, "t_adr", Data(adr)),
        (cycle, "data", Data(data)),
    ]
}

fn io_write(cycle: usize, adr: u32, data: u32) -> Script {
    vec![
        (cycle, "io_we", Data(1)),
        (cycle, "io_adr", Data(adr)),
        (cycle, "data", Data(data)),
    ]
}

fn io_read(cycle: usize, adr: u32) -> Script {
    vec![(cycle, "io_re", Data(1)), (cycle, "io_adr", Data(adr))]
}

fn cp0_write(cycle: usize, data: u32) -> Script {
    vec![
        (cycle, "c_we", Data(1)),
        (cycle, "c_adr", Data(0x6000)),
        (cycle, "data", Data(data)),
    ]
}

fn rfe(cycle: usize) -> Script {
    vec![(cycle, "rfe", Data(1))]
}

fn sim(script: Script, keys: &[(usize, &[u8])]) -> Simulator {
    let ports = [
        "t_we", "t_adr", "io_we", "io_re", "io_adr", "data", "c_we", "c_adr", "rfe", "sys", "pc",
        "ex4", "ovf",
    ];
    let driver = TestDriver {
        outputs: ports.iter().map(|p| (*p, Data(0))).collect(),
        script,
    };
    let i = |port| Input::new(DRIVER_ID, port);
    let s = test_sim(
        driver,
        vec![
            CP0::rc_new(
                "cp0",
                (0.0, 0.0),
                i("c_we"),
                i("c_adr"),
                i("data"),
                i("rfe"),
                Input::new("timer", TIMER_INTERRUPT_OUT_ID),
                Input::new("io", IO_INTERRUPT_OUT_ID),
                i("sys"),
                i("pc"),
                i("ex4"),
                i("ovf"),
            ),
            Rc::new(MipsTimer::new(
                "timer",
                (0.0, 0.0),
                i("t_adr"),
                i("data"),
                i("t_we"),
            )),
            Rc::new(MipsIO::new(
                "io",
                (0.0, 0.0),
                i("io_adr"),
                i("data"),
                i("io_we"),
                i("io_re"),
            )),
        ],
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

#[derive(Debug, PartialEq)]
struct Snapshot {
    cp0: Regs,
    timer: (u8, u32, u32, u32),
    io: (bool, u32, usize, usize),
    cp0_is_int: SignalValue,
    timer_int: SignalValue,
    io_int: SignalValue,
}

fn snapshot(sim: &Simulator) -> Snapshot {
    let t = get_component::<MipsTimer>(sim).data.borrow();
    let io = get_component::<MipsIO>(sim).data.borrow();
    Snapshot {
        cp0: get_component::<CP0>(sim).registers.borrow().clone(),
        timer: (t.flags, t.counter, t.compare, t.div_counter),
        io: (io.interrupt, io.input_control, io.read_pos, io.end_pos),
        cp0_is_int: sim.get_input_value(&Input::new("cp0", CP0_IS_INT_OUT_ID)),
        timer_int: sim.get_input_value(&Input::new("timer", TIMER_INTERRUPT_OUT_ID)),
        io_int: sim.get_input_value(&Input::new("io", IO_INTERRUPT_OUT_ID)),
    }
}

fn script() -> Script {
    [
        timer_write(1, TIMER_COMPARE, 3),
        timer_write(2, TIMER_FLAGS, TIMER_EN_CIE_CR),
        io_write(3, IO_CONTROL, IO_IE),
        cp0_write(4, SR_ALL),
        // timer interrupt is taken at 50, handler clears the flag and returns
        timer_write(80, TIMER_FLAGS, TIMER_EN_CIE_CR),
        rfe(81),
        // key arrives at 100, handler reads it and returns
        io_read(110, IO_KEY),
        rfe(111),
        // timer fires every 64 cycles, taken at 114, handler clears the flag and returns
        timer_write(160, TIMER_FLAGS, TIMER_EN_CIE_CR),
        rfe(161),
    ]
    .concat()
}

#[test]
fn interrupts_are_taken() {
    let mut s = sim(script(), &[(100, b"k")]);
    let mut entries = vec![];
    for _ in 0..200 {
        s.clock();
        if s.get_input_value(&Input::new("cp0", CP0_IS_INT_OUT_ID)) == Data(1) {
            let ecr = get_component::<CP0>(&s).registers.borrow().ecr;
            entries.push((s.cycle - 1, ecr));
        }
    }
    // timer, io, timer
    // (cycle, cause) timer, io, timer, timer
    assert_eq!(
        entries,
        vec![(50, 0x400), (100, 0x800), (114, 0x400), (178, 0x400)]
    );
}

#[test]
fn unclock_interrupts() {
    assert_round_trip(sim(script(), &[(100, b"k")]), 200, snapshot);
}
