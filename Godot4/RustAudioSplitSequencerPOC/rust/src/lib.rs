use godot::prelude::*;

struct ExtensionImpl;

// mod custom_audio_stream;
// mod demo;
mod debug;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
