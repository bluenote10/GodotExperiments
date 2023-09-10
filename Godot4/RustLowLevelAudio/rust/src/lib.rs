mod custom_audio_stream;

use godot::engine::{ControlVirtual, Os};
use godot::prelude::*;

use custom_audio_stream::{CustomAudioStream, CustomAudioStreamPlayback, WrappedAudioFrame};

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TreeRoot {
    #[base]
    audio_player: Gd<AudioStreamPlayer>,
    num_samples: usize,
}

#[godot_api]
impl ControlVirtual for TreeRoot {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("init");
        godot_print!(
            "main thread id: {}, thread caller id: {}",
            Os::singleton().get_main_thread_id(),
            Os::singleton().get_thread_caller_id()
        );

        let custom_audio_stream = Gd::<CustomAudioStream>::new_default();

        let mut audio_player = AudioStreamPlayer::new_alloc();
        audio_player.set_stream(custom_audio_stream.upcast());
        base.add_child(audio_player.share().upcast());

        Self {
            audio_player,
            num_samples: 0,
        }
    }

    fn ready(&mut self) {
        self.audio_player.play();
    }

    fn process(&mut self, _delta: f64) {
        let mut audio_stream_playback_gd = self
            .audio_player
            .get_stream_playback()
            .unwrap()
            .cast::<CustomAudioStreamPlayback>();
        let mut audio_stream_playback = audio_stream_playback_gd.bind_mut();

        let frames_available = audio_stream_playback.get_frames_available();
        godot_print!(
            "[{:08}] frames_available: {frames_available}",
            Os::singleton().get_thread_caller_id()
        );

        if frames_available > 0 {
            let frames: Vec<_> = (0..frames_available)
                .map(|_| {
                    let value = 0.5
                        * (2.0 * std::f32::consts::PI * 440.0 * self.num_samples as f32 / 44100.0)
                            .sin();
                    self.num_samples += 1;
                    WrappedAudioFrame {
                        left: value,
                        right: value,
                    }
                })
                .collect();
            audio_stream_playback.push_buffer(&frames);
        }
    }
}
