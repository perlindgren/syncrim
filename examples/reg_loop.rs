use syncrim::{
    common::{ComponentStore, Input, Simulator},
    components::*,
};

use std::rc::Rc;

fn main() {
    let a = Add {
        id: "add1".to_string(),
        pos: (10.0, 10.0),
        a_in: Input {
            id: "r1".to_string(),
            index: 0,
        },

        b_in: Input {
            id: "r1".to_string(),
            index: 0,
        },
    };

    let a = Rc::new(a);

    let r = Register {
        id: "r1".to_string(),
        pos: (100.0, 100.0),
        r_in: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };
    let r = Rc::new(r);

    let cs = ComponentStore { store: vec![a, r] };

    println!("--- store id:s");
    cs.to_();

    let (simulator, mut sim_state) = Simulator::new(&cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);

    // set initial value
    simulator.set_id_index(&mut sim_state, "add1", 0, 1);
    println!("--- SimState\n {:#?}", sim_state.lens_values);

    // clock one cycle
    simulator.clock(&mut sim_state);
    println!("--- SimState\n {:#?}", sim_state.lens_values);
}
