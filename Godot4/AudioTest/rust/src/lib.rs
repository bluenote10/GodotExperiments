use godot::prelude::*;

mod sequencer;
mod ui;
mod utils;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
