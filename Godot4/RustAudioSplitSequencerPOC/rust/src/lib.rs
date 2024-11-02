use godot::prelude::*;

struct ExtensionImpl;

mod custom_audio_stream;
mod demo;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
