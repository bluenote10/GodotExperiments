use std::sync::Arc;

use godot::classes::{AudioServer, IControl, Os};
use godot::prelude::*;

use crate::custom_audio_stream::CustomAudioStream;
use crate::sequencer::{Sequencer, SequencerInfo};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Demo {
    audio_player: Gd<AudioStreamPlayer>,
    sequencer_info: Arc<SequencerInfo>,
}

#[godot_api]
impl IControl for Demo {
    fn init(base: Base<Self::Base>) -> Self {
        println!("Demo::init");
        println!(
            "main thread id: {}, thread caller id: {}",
            Os::singleton().get_main_thread_id(),
            Os::singleton().get_thread_caller_id()
        );

        let sequencer = Sequencer::new(AudioServer::singleton().get_mix_rate());
        let sequencer_info = sequencer.get_sequencer_info();

        let mut audio_player = AudioStreamPlayer::new_alloc();
        audio_player.set_stream(Gd::<CustomAudioStream>::from_init_fn(move |_| {
            CustomAudioStream::new(sequencer)
        }));
        base.to_gd().add_child(audio_player.clone());

        Self {
            audio_player,
            sequencer_info,
        }
    }

    fn ready(&mut self) {
        // Play can only be called once the node is in the scene tree. This will internally
        // call `instantiate_playback`. Therefore we must never call `.play()` again, because
        // the consumer will be consumed internally.
        self.audio_player.play();
    }

    fn process(&mut self, _delta: f64) {
        let sample_index = self.sequencer_info.sample_index();
        println!(
            "[{:08}] process sample index: {}",
            Os::singleton().get_thread_caller_id(),
            sample_index
        );
    }
}
