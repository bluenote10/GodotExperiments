use godot::prelude::*;

mod render_stats;
mod ui;
mod utils;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
