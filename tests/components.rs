use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input, Signal, Simulator},
    components::*,
};

#[test]
fn test_add() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(ProbeOut {
                id: "po1".to_string(),
            }),
            Rc::new(ProbeOut {
                id: "po2".to_string(),
            }),
            Rc::new(Add {
                id: "add".to_string(),
                pos: (0.0, 0.0),
                a_in: Input {
                    id: "po1".to_string(),
                    index: 0,
                },

                b_in: Input {
                    id: "po2".to_string(),
                    index: 0,
                },
            }),
        ],
    };
    let mut clock = 0;
    let mut simulator = Simulator::new(&cs, &mut clock);

    assert_eq!(clock, 1);

    // outputs
    let add_val = &Input {
        id: "add".to_string(),
        index: 0,
    };
    let add_overflow = &Input {
        id: "add".to_string(),
        index: 1,
    };

    // reset
    assert_eq!(simulator.get_input_val(add_val), 0);
    assert_eq!(simulator.get_input_val(add_overflow), false as Signal);

    println!("<setup for clock 2>");
    simulator.set_id_index("po1", 0, 42);
    simulator.set_id_index("po2", 0, 1337);
    println!("sim_state {:?}", simulator.sim_state);
    println!("<clock>");
    simulator.clock(&mut clock);
    println!("sim_state {:?}", simulator.sim_state);
    assert_eq!(clock, 2);
    assert_eq!(simulator.get_input_val(add_val), 42 + 1337);
    assert_eq!(simulator.get_input_val(add_overflow), false as Signal);

    // trigger positive overflow
    println!("<setup for clock 3>");
    simulator.set_id_index("po1", 0, Signal::MAX / 2);
    simulator.set_id_index("po2", 0, 1);
    println!("sim_state {:?}", simulator.sim_state);
    println!("<clock>");
    simulator.clock(&mut clock);
    println!("sim_state {:?}", simulator.sim_state);
    assert_eq!(clock, 3);
    assert_eq!(simulator.get_input_val(add_val), Signal::MAX / 2 + 1);
    assert_eq!(simulator.get_input_val(add_overflow), true as Signal);
}
