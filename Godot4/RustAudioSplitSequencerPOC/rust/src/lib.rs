use godot::prelude::*;

struct ExtensionImpl;

mod v1;

// mod debug;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
