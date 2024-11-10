use godot::prelude::*;

struct ExtensionImpl;

mod atomic_float;
mod v1;
mod v2;

// mod debug;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
