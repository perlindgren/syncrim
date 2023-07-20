use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input, Signal, Simulator},
    components::*,
};

#[test]
fn test_add() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(ProbeOut::new("po1")),
            Rc::new(ProbeOut::new("po2")),
            Rc::new(Add {
                id: "add".to_string(),
                pos: (0.0, 0.0),
                a_in: Input::new("po1", "out"),
                b_in: Input::new("po2", "out"),
            }),
        ],
    };
    let mut clock = 0;
    let mut simulator = Simulator::new(&cs, &mut clock);

    assert_eq!(clock, 1);

    // outputs
    let add_val = &Input::new("add", "out");
    let add_overflow = &Input::new("add", "overflow");

    // reset
    assert_eq!(simulator.get_input_val(add_val), 0 + 0);
    assert_eq!(simulator.get_input_val(add_overflow), false as Signal);

    println!("<setup for clock 2>");
    simulator.set_out_val("po1", "out", 42);
    simulator.set_out_val("po2", "out", 1337);
    println!("sim_state {:?}", simulator.sim_state);
    println!("<clock>");
    simulator.clock(&mut clock);
    println!("sim_state {:?}", simulator.sim_state);
    assert_eq!(clock, 2);
    assert_eq!(simulator.get_input_val(add_val), 42 + 1337);
    assert_eq!(simulator.get_input_val(add_overflow), false as Signal);

    // trigger positive overflow
    println!("<setup for clock 3>");
    simulator.set_out_val("po1", "out", Signal::MAX / 2);
    simulator.set_out_val("po2", "out", 1);
    println!("sim_state {:?}", simulator.sim_state);
    println!("<clock>");
    simulator.clock(&mut clock);
    println!("sim_state {:?}", simulator.sim_state);
    assert_eq!(clock, 3);
    assert_eq!(simulator.get_input_val(add_val), Signal::MAX / 2 + 1);
    assert_eq!(simulator.get_input_val(add_overflow), true as Signal);
}

#[test]
fn test_sext() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(ProbeOut::new("po")),
            Rc::new(Sext {
                id: "sext32".to_string(),
                pos: (0.0, 0.0),
                sext_in: Input::new("po", "out"),
                in_size: 4,
                out_size: 32,
            }),
            Rc::new(Sext {
                id: "sext16".to_string(),
                pos: (0.0, 0.0),
                sext_in: Input::new("po", "out"),
                in_size: 4,
                out_size: 16,
            }),
        ],
    };
    let mut clock = 0;
    let mut simulator = Simulator::new(&cs, &mut clock);

    assert_eq!(clock, 1);

    // outputs
    let sext32_out = &Input::new("sext32", "out");
    let sext16_out = &Input::new("sext16", "out");

    // reset
    assert_eq!(simulator.get_input_val(sext32_out), 0 as Signal);
    assert_eq!(simulator.get_input_val(sext16_out), 0 as Signal);

    // Sign-extended
    println!("<setup for clock 2>");
    simulator.set_out_val("po", "out", 0b1111 as Signal);
    println!("sim_state {:?}", simulator.sim_state);
    println!("<clock>");
    simulator.clock(&mut clock);
    println!("sim_state {:?}", simulator.sim_state);
    assert_eq!(clock, 2);
    assert_eq!(simulator.get_input_val(sext32_out), 0xFFFFFFFF as Signal);
    assert_eq!(simulator.get_input_val(sext16_out), 0xFFFF as Signal);

    // Zero-extended
    println!("<setup for clock 3>");
    simulator.set_out_val("po", "out", 0b111);
    println!("sim_state {:?}", simulator.sim_state);
    println!("<clock>");
    simulator.clock(&mut clock);
    println!("sim_state {:?}", simulator.sim_state);
    assert_eq!(clock, 3);
    assert_eq!(simulator.get_input_val(sext32_out), 0b111 as Signal);
    assert_eq!(simulator.get_input_val(sext16_out), 0b111 as Signal);

    // Unclean upper bits
    println!("<setup for clock 4>");
    simulator.set_out_val("po", "out", 0b10111);
    println!("sim_state {:?}", simulator.sim_state);
    println!("<clock>");
    simulator.clock(&mut clock);
    println!("sim_state {:?}", simulator.sim_state);
    assert_eq!(clock, 4);
    assert_eq!(simulator.get_input_val(sext32_out), 0b111 as Signal);
    assert_eq!(simulator.get_input_val(sext16_out), 0b111 as Signal);
}
