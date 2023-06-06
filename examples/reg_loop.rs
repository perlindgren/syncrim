use mips::*;
use syncrim::components::Component;
use syncrim::{components::*, *};

fn main() {
    let a = Add {
        id: "add1".to_string(),
        a_in: Input {
            id: "r1".to_string(),
            index: 0,
        },

        b_in: Input {
            id: "r1".to_string(),
            index: 0,
        },
    };

    let a = Box::new(a) as Box<dyn Component>;

    let r = Register {
        id: "r1".to_string(),
        r_in: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };
    let r = Box::new(r) as Box<dyn Component>;

    let cs = ComponentStore { store: vec![a, r] };

    cs.to_();

    let json = serde_json::to_string(&cs).unwrap();
    println!("json: {}", json);

    let cs = ComponentStore::load(&json);

    let sim_state = SimState::new(cs);
    println!("SimState {:#?}", sim_state);
}
