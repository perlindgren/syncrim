use clap::Parser;
use std::path::PathBuf;
use syncrim::common::ComponentStore;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the model to load on startup
    #[arg(short, long)]
    model: String,
}

fn main() {
    let args = Args::parse();
    let _path = PathBuf::from(args.model);

    let _cs = ComponentStore::load_file(&_path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&_cs, &_path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&_cs, &_path);
}
