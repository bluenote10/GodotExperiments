use godot::engine::native::AudioFrame;
use godot::engine::{AudioStreamPlayback, AudioStreamPlaybackVirtual, AudioStreamVirtual, Os};
use godot::prelude::*;
use ringbuf::ring_buffer::RbBase;
use ringbuf::{HeapRb, Rb};

/// I think we cannot just use the specified capacity, because it also depends
/// on the buffer size Godot is using internally for the audio server (which
/// is hardcode to 512). And it is perhaps wise to allow for multiples of that
/// in case things get delayed?
const MIN_CAPACITY: usize = 512 * 4;

#[derive(GodotClass)]
#[class(base=AudioStream)]
pub struct CustomAudioStream {
    capacity: usize,
}

#[godot_api]
impl AudioStreamVirtual for CustomAudioStream {
    fn init(_base: Base<Self::Base>) -> Self {
        Self { capacity: 128 }
    }

    fn instantiate_playback(&self) -> Option<Gd<AudioStreamPlayback>> {
        Some(
            Gd::<CustomAudioStreamPlayback>::with_base(|_base| {
                CustomAudioStreamPlayback::new(self.capacity.max(MIN_CAPACITY))
            })
            .upcast(),
        )
    }
}

#[derive(GodotClass)]
#[class(base=AudioStreamPlayback)]
pub struct CustomAudioStreamPlayback {
    ring_buffer: HeapRb<WrappedAudioFrame>,
    local_buffer: Vec<WrappedAudioFrame>,
}

#[godot_api]
impl AudioStreamPlaybackVirtual for CustomAudioStreamPlayback {
    /*
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
    */

    unsafe fn mix(
        &mut self,
        buffer: *mut AudioFrame,
        _rate_scale: f32,
        num_requested_frames: i32,
    ) -> i32 {
        let num_available = self.ring_buffer.occupied_len();
        let num_to_read = num_available.min(num_requested_frames as usize);
        godot_print!("[{:08}] reading {num_to_read} frames (available: {num_available}, requested: {num_requested_frames})", Os::singleton().get_thread_caller_id());
        self.local_buffer.resize(
            num_to_read,
            WrappedAudioFrame {
                left: 0.0,
                right: 0.0,
            },
        );
        self.ring_buffer.pop_slice(&mut self.local_buffer);

        let mut i = 0;
        for frame in &self.local_buffer {
            *buffer.offset(i as isize) = (*frame).into();
            i += 1;
        }
        while i < num_requested_frames {
            // Buffer underrun
            *buffer.offset(i as isize) = AudioFrame {
                left: 0.0,
                right: 0.0,
            };
            i += 1;
        }
        num_requested_frames
    }

    fn start(&mut self, _from_pos: f64) {}
    fn stop(&mut self) {}
    fn is_playing(&self) -> bool {
        true
    }
}

impl CustomAudioStreamPlayback {
    fn new(capacity: usize) -> Self {
        let buffer = HeapRb::new(capacity);
        Self {
            ring_buffer: buffer,
            local_buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn get_frames_available(&self) -> usize {
        self.ring_buffer.free_len()
    }

    pub fn push_buffer(&mut self, frames: &[WrappedAudioFrame]) {
        godot_print!(
            "[{:08}] pushing {} frames",
            Os::singleton().get_thread_caller_id(),
            frames.len()
        );
        self.ring_buffer.push_slice_overwrite(frames);
    }
}

#[derive(Clone, Copy)]
pub struct WrappedAudioFrame {
    pub left: f32,
    pub right: f32,
}

impl From<WrappedAudioFrame> for AudioFrame {
    fn from(value: WrappedAudioFrame) -> Self {
        AudioFrame {
            left: value.left,
            right: value.right,
        }
    }
}

impl WrappedAudioFrame {
    pub fn to_audio_frame(&self) -> AudioFrame {
        (*self).into()
    }
}
