use clap::Parser;
// The trait impls from here are used dynamically when json is loaded, so this is actually used
#[allow(unused_imports)]
use mips_lib::*;
use std::path::PathBuf;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{common::ComponentStore, fern::fern_setup};
// this is actually loaded dynamically, so it is used.
#[allow(unused_imports)]
use mips_lib::components::*;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the model to load on startup
    #[arg(short, long, default_value = "mips_pipe.json")]
    model: String,
}

fn main() {
    fern_setup();
    let args = Args::parse();
    let path = PathBuf::from(args.model);

    let cs = ComponentStore::load_file(&path).unwrap_or(ComponentStore { store: vec![] });

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);

    #[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
    syncrim::common::Simulator::new(cs).unwrap();
}
