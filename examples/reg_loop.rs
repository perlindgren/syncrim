use syncrim::{
    common::{Component, Input},
    component_store::ComponentStore,
    components::*,
    simulator::SimState,
};

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

    println!("--- store id:s");
    cs.to_();

    let sim_state = SimState::new(cs);
    println!("--- SimState\n {:#?}", sim_state);
}
