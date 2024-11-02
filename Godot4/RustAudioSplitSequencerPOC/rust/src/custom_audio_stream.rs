use std::cell::RefCell;

use godot::classes::native::AudioFrame;
use godot::classes::{AudioStreamPlayback, IAudioStream, IAudioStreamPlayback, Os};
use godot::prelude::*;

type Sequencer = ();

// ----------------------------------------------------------------------------
// CustomAudioStream
// ----------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=AudioStream, no_init)]
pub struct CustomAudioStream {
    sequencer: RefCell<Option<Sequencer>>,
}

#[godot_api]
impl IAudioStream for CustomAudioStream {
    fn instantiate_playback(&self) -> Option<Gd<AudioStreamPlayback>> {
        // Since instantiate_playback doesn't allow for &mut self we need interior mutability here.
        let sequencer = self.sequencer.borrow_mut().take();
        if let Some(sequencer) = sequencer {
            Some(
                Gd::<CustomAudioStreamPlayback>::from_init_fn(|_base| {
                    CustomAudioStreamPlayback::new(sequencer)
                })
                .upcast(),
            )
        } else {
            godot_warn!("Tried to instantiate playback, but sequencer has already been consumed.");
            None
        }
    }
}

impl CustomAudioStream {
    pub fn new() -> Self {
        Self {
            sequencer: RefCell::new(Some(())),
        }
    }
}

// ----------------------------------------------------------------------------
// CustomAudioStreamPlayback
// ----------------------------------------------------------------------------

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
        // println!("foo");
        // let id = Os::singleton().get_thread_caller_id();
        // println!("bar");
        // godot_print!(
        //     "[{:08}] requested: {num_requested_frames})",
        //     Os::singleton().get_thread_caller_id()
        // );
        // println!("baz");
        //
        // let mut i = 0;
        // while i < num_requested_frames {
        //     // Buffer underrun
        //     *buffer.offset(i as isize) = AudioFrame {
        //         left: 0.0,
        //         right: 0.0,
        //     };
        //     i += 1;
        // }
        // num_requested_frames
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
        Self { sequencer }
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
