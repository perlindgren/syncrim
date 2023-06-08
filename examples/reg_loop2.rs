use syncrim::{
    common::{Component, ComponentStore, Input, Simulator},
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
            id: "r2".to_string(),
            index: 0,
        },
    };

    let a = Rc::new(a);

    let r1 = Register {
        id: "r1".to_string(),
        pos: (100.0, 10.0),
        r_in: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };
    let r1 = Rc::new(r1);

    let r2 = Register {
        id: "r2".to_string(),
        pos: (100.0, 50.0),
        r_in: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };
    let r2 = Rc::new(r2);

    let cs = ComponentStore {
        store: vec![a, r1, r2],
    };

    println!("--- store id:s");
    cs.to_();

    let (_simulator, sim_state) = Simulator::new(&cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);
}
