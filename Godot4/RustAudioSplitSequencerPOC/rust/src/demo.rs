use godot::classes::{IControl, Os};
use godot::prelude::*;

use super::custom_audio_stream::CustomAudioStream;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Demo {
    // #[base]
    audio_player: Gd<AudioStreamPlayer>,
}

#[godot_api]
impl IControl for Demo {
    fn init(base: Base<Self::Base>) -> Self {
        // godot_print!("init");
        // godot_print!(
        //     "main thread id: {}, thread caller id: {}",
        //     Os::singleton().get_main_thread_id(),
        //     Os::singleton().get_thread_caller_id()
        // );

        // let buffer: HeapRb<WrappedAudioFrame> = HeapRb::new(RING_BUF_SIZE);
        // let (producer, consumer) = buffer.split();

        println!("Demo::init");

        let mut audio_player = AudioStreamPlayer::new_alloc();
        audio_player.set_stream(Gd::<CustomAudioStream>::from_init_fn(|_| {
            CustomAudioStream::new()
        }));
        base.to_gd().add_child(audio_player.clone());

        Self { audio_player }
    }

    fn ready(&mut self) {
        // Play can only be called once the node is in the scene tree. This will internally
        // call `instantiate_playback`. Therefore we must never call `.play()` again, because
        // the consumer will be consumed internally.
        self.audio_player.play();
    }

    fn process(&mut self, _delta: f64) {
        // godot_print!("[{:08}] process...", Os::singleton().get_thread_caller_id());
    }
}
