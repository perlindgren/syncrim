// setup fern log

#[allow(unused_imports)]
use log::LevelFilter;

pub fn fern_setup() {
    let f = fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        // Add blanket level filter -
        // .level(log::LevelFilter::Debug);
        .level(log::LevelFilter::Trace);

    // - and per-module overrides
    #[cfg(feature = "gui-vizia")]
    let f = f
        .level_for("vizia_core::systems::hover", LevelFilter::Info)
        .level_for("vizia_core::context", LevelFilter::Info)
        .level_for("cosmic_text::buffer", LevelFilter::Warn)
        .level_for("cosmic_text", LevelFilter::Warn)
        .level_for("selectors::matching", LevelFilter::Warn)
        .level_for("cosmic_text::font::system::std", LevelFilter::Warn)
        .level_for("cosmic_text::font::fallback", LevelFilter::Warn)
        .level_for("async_io::driver", LevelFilter::Warn);

    #[cfg(feature = "gui-egui")]
    let f = f
        .level_for("eframe", LevelFilter::Warn)
        .level_for("async_io", LevelFilter::Warn)
        .level_for("polling", LevelFilter::Warn)
        .level_for("arboard", LevelFilter::Warn)
        .level_for("egui_glow", LevelFilter::Warn)
        .level_for("syncrim::gui_egui::components", LevelFilter::Warn)
        // .level_for("egui", LevelFilter::Error)
        ;

    f
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        // Apply globally
        .apply()
        .unwrap()
}
