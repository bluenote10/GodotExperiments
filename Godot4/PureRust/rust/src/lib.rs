use godot::prelude::*;

mod instantiation_order;
mod render_stats;
mod ui;
mod utils;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
