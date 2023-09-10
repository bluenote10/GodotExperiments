use godot::engine::native::AudioFrame;
use godot::engine::{AudioStreamPlayback, AudioStreamPlaybackVirtual, AudioStreamVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=AudioStream, init)]
pub struct CustomAudioStream {}

#[godot_api]
impl AudioStreamVirtual for CustomAudioStream {
    fn instantiate_playback(&self) -> Option<Gd<AudioStreamPlayback>> {
        Some(Gd::<CustomAudioStreamPlayback>::new_default().upcast())
    }
}

#[derive(GodotClass)]
#[class(base=AudioStreamPlayback, init)]
pub struct CustomAudioStreamPlayback {
    num_samples: u64,
}

#[godot_api]
impl AudioStreamPlaybackVirtual for CustomAudioStreamPlayback {
    unsafe fn mix(&mut self, buffer: *mut AudioFrame, _rate_scale: f32, frames: i32) -> i32 {
        for i in 0..frames {
            let value = 0.5
                * (2.0 * std::f32::consts::PI * 440.0 * self.num_samples as f32 / 44100.0).sin();
            *buffer.offset(i as isize) = AudioFrame {
                left: value,
                right: value,
            };
            self.num_samples += 1
        }
        frames
    }
    fn start(&mut self, _from_pos: f64) {}
    fn stop(&mut self) {}
    fn is_playing(&self) -> bool {
        true
    }
}
