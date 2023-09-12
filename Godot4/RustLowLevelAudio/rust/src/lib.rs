mod custom_audio_stream_v1;
mod custom_audio_stream_v2;
mod demo_v1;
mod demo_v2;

use godot::prelude::*;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
