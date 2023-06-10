use syncrim::{
    common::{ComponentStore, Input, Simulator},
    components::*,
};

use std::rc::Rc;

fn main() {
    let a1 = Add {
        id: "add1".to_string(),
        pos: (200.0, 200.0),
        a_in: Input {
            id: "r1".to_string(),
            index: 0,
        },

        b_in: Input {
            id: "r2".to_string(),
            index: 0,
        },
    };

    let a1 = Rc::new(a1);
    let a2 = Add {
        id: "add2".to_string(),
        pos: (100.0, 100.0),
        a_in: Input {
            id: "add1".to_string(),
            index: 0,
        },

        b_in: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };

    let a2 = Rc::new(a2);

    let r1 = Register {
        id: "r1".to_string(),
        pos: (50.0, 100.0),
        r_in: Input {
            id: "add2".to_string(),
            index: 0,
        },
    };

    let r1 = Rc::new(r1);

    let r2 = Register {
        id: "r2".to_string(),
        pos: (50.0, 50.0),
        r_in: Input {
            id: "add2".to_string(),
            index: 0,
        },
    };
    let r2 = Rc::new(r2);

    let cs = ComponentStore {
        store: vec![r2, a2, r1, a1],
    };

    println!("--- store id:s");
    cs.to_();

    let (_simulator, sim_state) = Simulator::new(&cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);
}
