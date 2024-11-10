use godot::classes::{IControl, Os};
use godot::prelude::*;

use crate::v2::sequencer::create_sequencer_control;

use super::custom_audio_stream::CustomAudioStreamV2;
use super::sequencer::SequencerControlInput;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct DemoV2 {
    audio_player: Gd<AudioStreamPlayer>,
    control_in: SequencerControlInput,
}

#[godot_api]
impl IControl for DemoV2 {
    fn init(base: Base<Self::Base>) -> Self {
        println!("Demo::init");
        println!(
            "main thread id: {}, thread caller id: {}",
            Os::singleton().get_main_thread_id(),
            Os::singleton().get_thread_caller_id()
        );

        let (control_in, control_out) = create_sequencer_control();

        control_in.init_sequencer();

        let mut audio_player = AudioStreamPlayer::new_alloc();
        audio_player.set_stream(Gd::<CustomAudioStreamV2>::from_init_fn(move |_| {
            CustomAudioStreamV2::new(control_out)
        }));
        base.to_gd().add_child(audio_player.clone());

        Self {
            audio_player,
            control_in,
        }
    }

    fn ready(&mut self) {
        // Play can only be called once the node is in the scene tree. This will internally
        // call `instantiate_playback`. Therefore we must never call `.play()` again, because
        // the consumer will be consumed internally.
        self.audio_player.play();
    }

    fn process(&mut self, _delta: f64) {
        let sample_index = self.control_in.atomics.sample_index();
        println!(
            "[{:08}] process sample index: {}",
            Os::singleton().get_thread_caller_id(),
            sample_index
        );

        if sample_index > 44100 * 5 {
            self.control_in.set_frequency(440.0);
            // let old_value = self.sequencer_info.get_param(0).unwrap();
            // if old_value < 100.0 {
            //     self.sequencer_info.set_param(0, old_value * 2.0);
            // }
        }
    }
}
