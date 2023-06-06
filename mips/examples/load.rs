use mips::*;
use std::fs::File;
use std::io::prelude::*;
use syncrim::{component_store::ComponentStore, simulator::SimState};

fn main() {
    let mut file = File::open("mips.json").unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();

    let cs = ComponentStore::load(&json);

    let sim_state = SimState::new(cs);
    println!("SimState {:#?}", sim_state);
}
