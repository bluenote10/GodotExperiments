use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use godot::classes::Os;
use ringbuf::traits::{Consumer, Producer, Split};
use ringbuf::HeapRb;

#[derive(Debug)]
pub enum Command {
    SetFrequency(f32),
}

pub type CommandProducer = <HeapRb<Command> as Split>::Prod;
pub type CommandConsumer = <HeapRb<Command> as Split>::Cons;

/// This class serves as the "send/sync communication interface" that
/// is held on the main thread.
///
/// Why having a separate struct and not using an Arc<Sequencer> directly?
/// In general the Sequencer may not be send/sync, and splitting out the
/// send/sync communication allows for more flexibility in the Sequencer
/// implementation itself
pub struct SequencerInfo {
    sample_index: AtomicUsize,
    // Note that we need a Mutex, because the user only holds an `Arc`, and
    // we need interior mutability for inserting into the buffer.
    command_producer: Mutex<CommandProducer>,
    // Note that just using the HeapRb for both producer/consumer side does
    // note make sense: Due to Arc the SequencerInfo is immutable, so consumer
    // wouldn't be able to consume from it. Wrapping the entire buffer into a
    // Mutex would be pointless, because then the consumer would no longer be
    // able to consume from it lock-free. Essentially we are fine with locking
    // on the producer side (not the audio thread), but want to avoid any kind
    // of lock/mutex on the consumer side (audio thread).
    // commands: HeapRb<Command>,
}

impl SequencerInfo {
    pub fn sample_index(&self) -> usize {
        self.sample_index.load(Ordering::Acquire)
    }

    pub fn set_frequency(&self, freq: f32) {
        // Why encoding this as a commend instead of just setting an atomic?
        // Just to demonstrate how it would work with "compound" non atomic data, that
        // contains *several* fields. Think of setting a loop, which has a start and an
        // end field. It could be modeled using two atomics, but then setting the entire
        // loop would not be atomic, because it would require two atomic operations. The
        // command queue pattern allows for transferring data "atomically" in general.
        let mut command_producer = self.command_producer.lock().unwrap();
        command_producer
            .try_push(Command::SetFrequency(freq))
            .unwrap();
    }
}

/// A dummy sequencer implementation.
pub struct Sequencer {
    frequency: f32,
    sample_rate: f32,
    sample_index: usize,
    shared: Arc<SequencerInfo>,
    command_consumer: CommandConsumer,
}

impl Sequencer {
    pub fn new(sample_rate: f32) -> Self {
        let commands = HeapRb::<Command>::new(4);
        let (command_producer, command_consumer) = commands.split();

        let shared = Arc::new(SequencerInfo {
            sample_index: AtomicUsize::new(0),
            command_producer: Mutex::new(command_producer),
        });

        Self {
            frequency: 220.0,
            sample_rate,
            sample_index: 0,
            shared,
            command_consumer,
        }
    }

    pub fn get_sequencer_info(&self) -> Arc<SequencerInfo> {
        self.shared.clone()
    }

    pub fn render_audio(&mut self, num_to_generate: usize, buffers: &mut [&mut [f32]]) {
        assert_eq!(buffers.len(), 2);

        while let Some(command) = self.command_consumer.try_pop() {
            match command {
                Command::SetFrequency(freq) => self.frequency = freq,
            }
        }

        for i in 0..num_to_generate {
            let phase = 2.0 * std::f32::consts::PI * self.frequency * (self.sample_index as f32)
                / self.sample_rate;
            let sample = 0.5 * phase.sin();

            for j in 0..buffers.len() {
                buffers[j][i] = sample;
            }

            self.sample_index += 1;
        }

        // Sync state into `shared`
        self.shared
            .sample_index
            .store(self.sample_index, Ordering::Release);

        println!(
            "[{:08}] render_audio: {}",
            Os::singleton().get_thread_caller_id(),
            self.sample_index
        );
    }
}
