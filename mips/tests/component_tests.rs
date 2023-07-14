use mips::components::RegFile;
use std::cell::Cell;
use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input, Signal, Simulator},
    components::*,
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
            Rc::new(RegFile {
                id: "reg_file".to_string(),
                pos: (200.0, 150.0),
                width: 100.0,
                height: 150.0,

                // ports
                read_addr1: Input::new("read_reg_1", 0),
                read_addr2: Input::new("read_reg_2", 0),
                write_data: Input::new("write_data", 0),
                write_addr: Input::new("write_addr", 0),
                write_enable: Input::new("write_enable", 0),

                // data
                registers: vec![Cell::new(0); 32],
            }),
        ],
    };
    let mut clock = 0;
    let mut simulator = Simulator::new(&cs, &mut clock);

    assert_eq!(clock, 1);

    // outputs
    let out_reg_1 = &Input::new("reg_file", 0);
    let out_reg_2 = &Input::new("reg_file", 1);

    // reset
    assert_eq!(simulator.get_input_val(out_reg_1), 0);
    assert_eq!(simulator.get_input_val(out_reg_2), 0);

    println!("<setup for clock 2>");
    simulator.set_out_val("read_reg_1", "out", 0);
    simulator.set_out_val("read_reg_2", "out", 1);
    simulator.set_out_val("write_data", "out", 1337);
    simulator.set_out_val("write_addr", "out", 1);
    simulator.set_out_val("write_enable", "out", true as Signal);

    // test write and read to reg # 1 in same cycle
    println!("sim_state {:?}", simulator.sim_state);
    println!("<clock>");
    simulator.clock(&mut clock);
    println!("sim_state {:?}", simulator.sim_state);
    assert_eq!(clock, 2);
    assert_eq!(simulator.get_input_val(out_reg_1), 0);
    assert_eq!(simulator.get_input_val(out_reg_2), 1337);

    // test write and read to reg # 0 in same cycle (red #0 should always read 0)
    println!("<setup for clock 3>");
    simulator.set_out_val("read_reg_1", "out", 0);
    simulator.set_out_val("read_reg_2", "out", 1);
    simulator.set_out_val("write_data", "out", 42);
    simulator.set_out_val("write_addr", "out", 0);
    simulator.set_out_val("write_enable", "out", true as Signal);
    println!("<clock>");
    simulator.clock(&mut clock);
    println!("sim_state {:?}", simulator.sim_state);
    assert_eq!(clock, 3);
    assert_eq!(simulator.get_input_val(out_reg_1), 0);
    assert_eq!(simulator.get_input_val(out_reg_2), 1337);
}

// An example of a test that should panic (fail)
// Useful to assert that illegal models and/or states does not pass unnoticed
#[test]
#[should_panic(expected = "assertion failed")]
fn should_fail() {
    assert!(false)
}
