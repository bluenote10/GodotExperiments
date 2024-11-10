use std::cell::RefCell;

use godot::classes::native::AudioFrame;
use godot::classes::{AudioStreamPlayback, IAudioStream, IAudioStreamPlayback, Os};
use godot::prelude::*;

use super::sequencer::Sequencer;

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
        println!(
            "[{:08}] instantiate_playback",
            Os::singleton().get_thread_caller_id()
        );
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
    pub fn new(sequencer: Sequencer) -> Self {
        println!("CustomAudioStream::new");
        let sequencer = RefCell::new(Some(sequencer));
        Self { sequencer }
    }
}

// ----------------------------------------------------------------------------
// CustomAudioStreamPlayback
// ----------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=AudioStreamPlayback, no_init)]
pub struct CustomAudioStreamPlayback {
    sequencer: Sequencer,
    temp_buffers: [Vec<f32>; 2],
}

#[godot_api]
impl IAudioStreamPlayback for CustomAudioStreamPlayback {
    unsafe fn mix(
        &mut self,
        buffer: *mut AudioFrame,
        _rate_scale: f32,
        num_requested_frames: i32,
    ) -> i32 {
        for buffer in self.temp_buffers.iter_mut() {
            buffer.resize(num_requested_frames as usize, 0.0);
        }

        let mut temp_buffers = self
            .temp_buffers
            .iter_mut()
            .map(|buffer| buffer.as_mut_slice())
            .collect::<Vec<&mut [f32]>>();

        self.sequencer
            .render_audio(num_requested_frames as usize, temp_buffers.as_mut_slice());

        for i in 0..num_requested_frames {
            unsafe {
                *buffer.offset(i as isize) = AudioFrame {
                    left: temp_buffers[0][i as usize],
                    right: temp_buffers[1][i as usize],
                };
            }
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
    fn new(sequencer: Sequencer) -> Self {
        let temp_buffers = [const { Vec::<f32>::new() }; 2];
        Self {
            sequencer,
            temp_buffers,
        }
    }
}
