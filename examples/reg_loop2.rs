use syncrim::{
    common::{Component, ComponentStore, Input, Simulator},
    components::*,
};

fn main() {
    let a = Add {
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

    let a = Box::new(a) as Box<dyn Component>;

    let r1 = Register {
        id: "r1".to_string(),
        r_in: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };
    let r1 = Box::new(r1) as Box<dyn Component>;

    let r2 = Register {
        id: "r2".to_string(),
        r_in: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };
    let r2 = Box::new(r2) as Box<dyn Component>;

    let cs = ComponentStore {
        store: vec![a, r1, r2],
    };

    println!("--- store id:s");
    cs.to_();

    let (_simulator, sim_state) = Simulator::new(&cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);
}
