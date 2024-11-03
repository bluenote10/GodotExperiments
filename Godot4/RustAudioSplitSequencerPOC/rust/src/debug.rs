use godot::classes::native::AudioFrame;
use godot::classes::{AudioServer, AudioStreamPlayback, IAudioStream, IAudioStreamPlayback, Os};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Demo {
    audio_player: Gd<AudioStreamPlayer>,
}

#[godot_api]
impl INode for Demo {
    fn init(base: Base<Self::Base>) -> Self {
        println!(
            "Demo::init is running on thread {}",
            Os::singleton().get_thread_caller_id()
        );

        let mut audio_player = AudioStreamPlayer::new_alloc();
        audio_player.set_stream(Gd::<CustomAudioStream>::from_init_fn(|_| {
            CustomAudioStream::new()
        }));
        base.to_gd().add_child(audio_player.clone());

        Self { audio_player }
    }

    fn ready(&mut self) {
        println!(
            "Demo::ready is running on thread {}",
            Os::singleton().get_thread_caller_id()
        );
        self.audio_player.play();
    }
}

// CustomAudioStream

#[derive(GodotClass)]
#[class(base=AudioStream, no_init)]
pub struct CustomAudioStream {}

#[godot_api]
impl IAudioStream for CustomAudioStream {
    fn instantiate_playback(&self) -> Option<Gd<AudioStreamPlayback>> {
        println!(
            "CustomAudioStream::instantiate_playback is running on thread {}",
            Os::singleton().get_thread_caller_id()
        );
        Some(
            Gd::<CustomAudioStreamPlayback>::from_init_fn(|_base| {
                CustomAudioStreamPlayback::new(Sequencer {
                    sample_rate: AudioServer::singleton().get_mix_rate(),
                    sample_index: 0,
                })
            })
            .upcast(),
        )
    }
}

impl CustomAudioStream {
    pub fn new() -> Self {
        println!(
            "CustomAudioStream::new is running on thread {}",
            Os::singleton().get_thread_caller_id()
        );
        Self {}
    }
}

// CustomAudioStreamPlayback

#[derive(GodotClass)]
#[class(base=AudioStreamPlayback, no_init)]
pub struct CustomAudioStreamPlayback {
    sequencer: Sequencer,
}

#[godot_api]
impl IAudioStreamPlayback for CustomAudioStreamPlayback {
    unsafe fn mix(
        &mut self,
        buffer: *mut AudioFrame,
        _rate_scale: f32,
        num_requested_frames: i32,
    ) -> i32 {
        println!(
            "CustomAudioStreamPlayback::mix is running on thread {}",
            Os::singleton().get_thread_caller_id()
        );
        self.sequencer.render_audio(num_requested_frames, buffer);
        num_requested_frames
    }

    fn start(&mut self, _from_pos: f64) {}
    fn stop(&mut self) {}
    fn is_playing(&self) -> bool {
        true
    }
}

impl CustomAudioStreamPlayback {
    fn new(sequencer: Sequencer) -> Self {
        println!(
            "CustomAudioStreamPlayback::new is running on thread {}",
            Os::singleton().get_thread_caller_id()
        );
        Self { sequencer }
    }
}

// Sequencer

pub struct Sequencer {
    sample_rate: f32,
    sample_index: usize,
}

impl Sequencer {
    fn render_audio(&mut self, num_requested_frames: i32, buffer: *mut AudioFrame) {
        const FREQUENCY: f32 = 440.0;
        for i in 0..num_requested_frames {
            let phase = 2.0 * std::f32::consts::PI * FREQUENCY * (self.sample_index as f32)
                / self.sample_rate;
            let sample = 0.5 * phase.sin();
            unsafe {
                *buffer.offset(i as isize) = AudioFrame {
                    left: sample,
                    right: sample,
                };
            }
            self.sample_index += 1;
        }
    }
}
