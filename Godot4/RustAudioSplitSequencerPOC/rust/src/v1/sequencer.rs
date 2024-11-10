use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use godot::classes::Os;
use ringbuf::traits::{Consumer, Producer, Split};
use ringbuf::HeapRb;

use crate::atomic_float::AtomicF64;

#[derive(Debug)]
pub enum Command {
    SetFrequency(f32),
}

pub type CommandProducer = <HeapRb<Command> as Split>::Prod;
pub type CommandConsumer = <HeapRb<Command> as Split>::Cons;

type ParamDatabase = HashMap<i32, AtomicF64>;

/// This class serves as the "send/sync communication interface" that
/// is held on the main thread.
///
/// Why having a separate struct and not using an Arc<Sequencer> directly?
/// In general the Sequencer may not be send/sync, and splitting out the
/// send/sync communication allows for more flexibility in the Sequencer
/// implementation itself.
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

    // The following would be an example of a lock free / sync parameter database.
    // Its usage would require that all parameters get populated upfront, because
    // adding/removing keys from the hashmap is not possible with a shared `&` reference.
    // But it would allow to read and modify the values stored in the hashmap across
    // threads, if the value types are atomic based.
    param_database: ParamDatabase,
}

static_assertions::assert_impl_all!(SequencerInfo: Send);
static_assertions::assert_impl_all!(SequencerInfo: Sync);

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

    pub fn set_param(&self, idx: i32, value: f64) {
        if let Some(entry) = self.param_database.get(&idx) {
            entry.store(value, Ordering::Release);
        }
    }

    pub fn get_param(&self, idx: i32) -> Option<f64> {
        if let Some(entry) = self.param_database.get(&idx) {
            Some(entry.load(Ordering::Acquire))
        } else {
            None
        }
    }
}

/// A dummy sequencer implementation.
pub struct Sequencer {
    frequency: f32,
    sample_rate: f32,
    sample_index: usize,
    shared: Arc<SequencerInfo>,
    command_consumer: CommandConsumer,
    // Dummy phantom data to simulate a non-send / non-sync Sequencer.
    phantom: PhantomData<Rc<i32>>,
}

static_assertions::assert_not_impl_all!(Sequencer: Send);
static_assertions::assert_not_impl_all!(Sequencer: Sync);

impl Sequencer {
    pub fn new(sample_rate: f32) -> Self {
        let commands = HeapRb::<Command>::new(4);
        let (command_producer, command_consumer) = commands.split();

        let shared = Arc::new(SequencerInfo {
            sample_index: AtomicUsize::new(0),
            command_producer: Mutex::new(command_producer),
            // Emulate upfront population of parameter database.
            param_database: HashMap::from([
                (0, AtomicF64::new(1.0)),
                (1, AtomicF64::new(2.0)),
                (2, AtomicF64::new(3.0)),
            ]),
        });

        Self {
            frequency: 220.0,
            sample_rate,
            sample_index: 0,
            shared,
            command_consumer,
            phantom: PhantomData,
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
            "[{:08}] render_audio: {} (param 0: {}, param 1: {}, param 2: {})",
            Os::singleton().get_thread_caller_id(),
            self.sample_index,
            self.get_sequencer_info().get_param(0).unwrap(),
            self.get_sequencer_info().get_param(1).unwrap(),
            self.get_sequencer_info().get_param(2).unwrap(),
        );
    }
}
