use syncrim::{
    common::{Component, ComponentStore, Input, Simulator},
    components::*,
};

fn main() {
    let a1 = Add {
        id: "add1".to_string(),
        a_in: Input {
            id: "r1".to_string(),
            index: 0,
        },

        b_in: Input {
            id: "r2".to_string(),
            index: 0,
        },
    };

    let a1 = Box::new(a1) as Box<dyn Component>;

    let a2 = Add {
        id: "add2".to_string(),
        a_in: Input {
            id: "add1".to_string(),
            index: 0,
        },

        b_in: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };

    let a2 = Box::new(a2) as Box<dyn Component>;

    let r1 = Register {
        id: "r1".to_string(),
        r_in: Input {
            id: "add2".to_string(),
            index: 0,
        },
    };

    let r1 = Box::new(r1) as Box<dyn Component>;

    let r2 = Register {
        id: "r2".to_string(),
        r_in: Input {
            id: "add2".to_string(),
            index: 0,
        },
    };
    let r2 = Box::new(r2) as Box<dyn Component>;

    let cs = ComponentStore {
        store: vec![r2, a2, r1, a1],
    };

    println!("--- store id:s");
    cs.to_();

    let (_simulator, sim_state) = Simulator::new(&cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);
}
