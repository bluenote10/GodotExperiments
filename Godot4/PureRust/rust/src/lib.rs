use godot::prelude::*;

mod ui;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
