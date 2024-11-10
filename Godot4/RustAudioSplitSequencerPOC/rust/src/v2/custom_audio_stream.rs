use std::cell::RefCell;

use godot::classes::native::AudioFrame;
use godot::classes::{AudioServer, AudioStreamPlayback, IAudioStream, IAudioStreamPlayback, Os};
use godot::prelude::*;

use super::sequencer::{Command, Sequencer, SequencerControlOutput};

// ----------------------------------------------------------------------------
// CustomAudioStream
// ----------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=AudioStream, no_init)]
pub struct CustomAudioStreamV2 {
    sequencer_control: RefCell<Option<SequencerControlOutput>>,
}

#[godot_api]
impl IAudioStream for CustomAudioStreamV2 {
    fn instantiate_playback(&self) -> Option<Gd<AudioStreamPlayback>> {
        println!(
            "[{:08}] instantiate_playback",
            Os::singleton().get_thread_caller_id()
        );
        // Since instantiate_playback doesn't allow for &mut self we need interior mutability here.
        let sequencer_control = self.sequencer_control.borrow_mut().take();
        let sample_rate = AudioServer::singleton().get_mix_rate();
        if let Some(sequencer_control) = sequencer_control {
            Some(
                Gd::<CustomAudioStreamPlaybackV2>::from_init_fn(|_base| {
                    CustomAudioStreamPlaybackV2::new(sample_rate, sequencer_control)
                })
                .upcast(),
            )
        } else {
            godot_warn!("Tried to instantiate playback, but sequencer has already been consumed.");
            None
        }
    }
}

impl CustomAudioStreamV2 {
    pub fn new(sequencer_control: SequencerControlOutput) -> Self {
        println!("CustomAudioStream::new");
        let sequencer_control = RefCell::new(Some(sequencer_control));
        Self { sequencer_control }
    }
}

// ----------------------------------------------------------------------------
// CustomAudioStreamPlayback
// ----------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=AudioStreamPlayback, no_init)]
pub struct CustomAudioStreamPlaybackV2 {
    sample_rate: f32,
    sequencer_control: SequencerControlOutput,
    sequencer: Option<Sequencer>,
    temp_buffers: [Vec<f32>; 2],
}

#[godot_api]
impl IAudioStreamPlayback for CustomAudioStreamPlaybackV2 {
    unsafe fn mix(
        &mut self,
        buffer: *mut AudioFrame,
        _rate_scale: f32,
        num_requested_frames: i32,
    ) -> i32 {
        self.handle_control();

        if let Some(sequencer) = self.sequencer.as_mut() {
            for buffer in self.temp_buffers.iter_mut() {
                buffer.resize(num_requested_frames as usize, 0.0);
            }

            let mut temp_buffers = self
                .temp_buffers
                .iter_mut()
                .map(|buffer| buffer.as_mut_slice())
                .collect::<Vec<&mut [f32]>>();

            sequencer.render_audio(num_requested_frames as usize, temp_buffers.as_mut_slice());

            for i in 0..num_requested_frames {
                unsafe {
                    *buffer.offset(i as isize) = AudioFrame {
                        left: temp_buffers[0][i as usize],
                        right: temp_buffers[1][i as usize],
                    };
                }
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

impl CustomAudioStreamPlaybackV2 {
    fn new(sample_rate: f32, sequencer_control: SequencerControlOutput) -> Self {
        let temp_buffers = [const { Vec::<f32>::new() }; 2];
        Self {
            sample_rate,
            sequencer_control,
            sequencer: None,
            temp_buffers,
        }
    }

    fn handle_control(&mut self) {
        while let Some(command) = self.sequencer_control.try_pop() {
            match command {
                Command::InitSequencer() => {
                    self.sequencer = Some(Sequencer::new(
                        self.sample_rate,
                        self.sequencer_control.atomics.clone(),
                    ))
                }
                Command::SetFrequency(freq) => {
                    if let Some(sequencer) = self.sequencer.as_mut() {
                        sequencer.set_frequency(freq)
                    }
                }
            }
        }
    }
}
