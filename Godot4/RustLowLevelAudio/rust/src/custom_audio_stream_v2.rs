use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::sync::Arc;

use godot::engine::native::AudioFrame;
use godot::engine::{AudioStreamPlayback, AudioStreamPlaybackVirtual, AudioStreamVirtual, Os};
use godot::prelude::*;
use ringbuf::{Consumer, Producer, SharedRb};

pub type AudioProducer = Producer<
    WrappedAudioFrame,
    Arc<SharedRb<WrappedAudioFrame, Vec<MaybeUninit<WrappedAudioFrame>>>>,
>;
pub type AudioConsumer = Consumer<
    WrappedAudioFrame,
    Arc<SharedRb<WrappedAudioFrame, Vec<MaybeUninit<WrappedAudioFrame>>>>,
>;

// ----------------------------------------------------------------------------
// CustomAudioStream
// ----------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=AudioStream)]
pub struct CustomAudioStream {
    consumer: RefCell<Option<AudioConsumer>>,
}

#[godot_api]
impl AudioStreamVirtual for CustomAudioStream {
    fn instantiate_playback(&self) -> Option<Gd<AudioStreamPlayback>> {
        // Since instantiate_playback doesn't allow for &mut self we need interior mutability here.
        let consumer = self.consumer.borrow_mut().take();
        if let Some(consumer) = consumer {
            Some(
                Gd::<CustomAudioStreamPlayback>::with_base(|_base| {
                    CustomAudioStreamPlayback::new(consumer)
                })
                .upcast(),
            )
        } else {
            godot_warn!("Tried to instantiate playback, but consumer has already been consumed.");
            None
        }
    }
}

impl CustomAudioStream {
    pub fn new(consumer: AudioConsumer) -> Self {
        Self {
            consumer: RefCell::new(Some(consumer)),
        }
    }
}

// ----------------------------------------------------------------------------
// CustomAudioStreamPlayback
// ----------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=AudioStreamPlayback)]
pub struct CustomAudioStreamPlayback {
    consumer: AudioConsumer,
    local_buffer: Vec<WrappedAudioFrame>,
}

#[godot_api]
impl AudioStreamPlaybackVirtual for CustomAudioStreamPlayback {
    unsafe fn mix(
        &mut self,
        buffer: *mut AudioFrame,
        _rate_scale: f32,
        num_requested_frames: i32,
    ) -> i32 {
        let num_available = self.consumer.len();
        let num_to_read = num_available.min(num_requested_frames as usize);
        godot_print!("[{:08}] reading {num_to_read} frames (available: {num_available}, requested: {num_requested_frames})", Os::singleton().get_thread_caller_id());
        self.local_buffer.resize(
            num_to_read,
            WrappedAudioFrame {
                left: 0.0,
                right: 0.0,
            },
        );

        self.consumer.pop_slice(&mut self.local_buffer);

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
    fn new(consumer: AudioConsumer) -> Self {
        let capacity = consumer.capacity();
        Self {
            consumer,
            local_buffer: Vec::with_capacity(capacity),
        }
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
    pub fn new(left: f32, right: f32) -> Self {
        Self { left, right }
    }

    pub fn to_audio_frame(&self) -> AudioFrame {
        (*self).into()
    }
}
