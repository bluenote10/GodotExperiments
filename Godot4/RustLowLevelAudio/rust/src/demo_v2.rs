use godot::engine::{ControlVirtual, Os};
use godot::prelude::*;
use ringbuf::HeapRb;

use super::custom_audio_stream_v2::{AudioProducer, CustomAudioStream, WrappedAudioFrame};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct DemoV2 {
    #[base]
    audio_player: Gd<AudioStreamPlayer>,
    producer: AudioProducer,
    num_samples: usize,
}

const RING_BUF_SIZE: usize = 1024;

#[godot_api]
impl ControlVirtual for DemoV2 {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("init");
        godot_print!(
            "main thread id: {}, thread caller id: {}",
            Os::singleton().get_main_thread_id(),
            Os::singleton().get_thread_caller_id()
        );

        let buffer: HeapRb<WrappedAudioFrame> = HeapRb::new(RING_BUF_SIZE);
        let (producer, consumer) = buffer.split();

        let custom_audio_stream =
            Gd::<CustomAudioStream>::with_base(|_| CustomAudioStream::new(consumer));

        let mut audio_player = AudioStreamPlayer::new_alloc();
        audio_player.set_stream(custom_audio_stream.upcast());
        base.add_child(audio_player.share().upcast());

        Self {
            audio_player,
            producer,
            num_samples: 0,
        }
    }

    fn ready(&mut self) {
        // Play can only be called once the node is in the scene tree. This will internally
        // call `instantiate_playback`. Therefore we must never call `.play()` again, because
        // the consumer will be consumed internally.
        self.audio_player.play();
    }

    fn process(&mut self, _delta: f64) {
        let frames_available = self.producer.free_len();
        godot_print!(
            "[{:08}] frames_available: {frames_available}",
            Os::singleton().get_thread_caller_id()
        );

        if frames_available > 0 {
            let frames: Vec<_> = (0..frames_available)
                .map(|_| {
                    let value = 0.5
                        * (2.0 * std::f32::consts::PI * 440.0 * self.num_samples as f32 / 44100.0)
                            .sin();
                    self.num_samples += 1;
                    WrappedAudioFrame::new(value, value)
                })
                .collect();
            self.producer.push_slice(&frames);
        }
    }
}
