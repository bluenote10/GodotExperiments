mod custom_audio_stream;

use godot::engine::ControlVirtual;
use godot::prelude::*;

use custom_audio_stream::CustomAudioStream;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TreeRoot {
    #[base]
    audio_player: Gd<AudioStreamPlayer>,
}

#[godot_api]
impl ControlVirtual for TreeRoot {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("init");

        let custom_audio_stream = Gd::<CustomAudioStream>::new_default();

        let mut audio_player = AudioStreamPlayer::new_alloc();
        audio_player.set_stream(custom_audio_stream.upcast());
        base.add_child(audio_player.share().upcast());

        Self { audio_player }
    }

    fn ready(&mut self) {
        self.audio_player.play();
    }
}
