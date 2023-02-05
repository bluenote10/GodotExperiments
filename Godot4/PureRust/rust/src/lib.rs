use godot::prelude::*;

mod ui;
mod utils;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
