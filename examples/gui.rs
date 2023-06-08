use syncrim::{
    common::{Component, ComponentStore, Input, Simulator},
    components::*,
};

fn main() {
    let r = Register {
        id: "r1".to_string(),
        r_in: Input {
            id: "r1".to_string(),
            index: 0,
        },
    };
    let r = Box::new(r) as Box<dyn Component>;

    let cs = ComponentStore { store: vec![r] };

    println!("--- store id:s");
    cs.to_();

    let (simulator, mut sim_state) = Simulator::new(&cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);

    syncrim::gui::gui(cs);
}
