#[allow(unused_imports)]
use mips::components::*;
use std::path::PathBuf;
use syncrim::{common::ComponentStore, fern::fern_setup};
fn main() {
    fern_setup();

    let path = PathBuf::from("mips.json");
    let cs = ComponentStore::load_file(&path);

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);

    #[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
    syncrim::common::Simulator::new(cs);
}
