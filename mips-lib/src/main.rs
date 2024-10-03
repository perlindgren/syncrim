use clap::Parser;
use std::path::PathBuf;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{common::ComponentStore, fern::fern_setup};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the model to load on startup
    #[arg(short, long, default_value = "mips_singel_cycle.json")]
    model: String,
}

fn main() {
    fern_setup();
    let args = Args::parse();
    let path = PathBuf::from(args.model);

    let cs = ComponentStore::load_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);

    #[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
    syncrim::common::Simulator::new(cs).unwrap();
}
