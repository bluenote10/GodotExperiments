use godot::prelude::*;

struct ExtensionImpl;

mod custom_audio_stream;
mod demo;
mod sequencer;

// mod debug;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
