use clap::Parser;
use std::path::PathBuf;
use syncrim::{common::ComponentStore, fern::fern_setup};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the model to load on startup
    #[arg(short, long)]
    model: String,
}

fn main() {
    fern_setup();
    let args = Args::parse();
    let _path = PathBuf::from(args.model);

    let cs = ComponentStore::load_file(&_path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &_path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &_path);

    // run headless
    #[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
    syncrim::common::Simulator::new(cs);
}
