use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use arc_swap::ArcSwap;
use godot::classes::Os;
use ringbuf::traits::{Consumer, Producer, Split};
use ringbuf::HeapRb;

pub struct WithTwoFields {
    foo: f32,
    bar: f32,
}

// ----------------------------------------------------------------------------
// Shared atomics
// ----------------------------------------------------------------------------

pub struct Atomics {
    sample_index: AtomicUsize,
    // Note that in general it is also possible to turn structs that are not
    // themselves atomic into lock-free atomics by "double atomic wrapping".
    // The idea is basically to use an atomic usize for the pointer, which
    // in turn points to an Arc (which is slightly confusing because its "A"
    // also means "atomic", but refers to its internal reference counting).
    // Basically the outer atomic allows to swap out the inner Arc itself,
    // allowing for atomically communicating larger structs between threads.
    // There are a few libraries like arc-swap, arc-atomic, and aarc that
    // can help with that:
    // - https://docs.rs/arc-swap/latest/arc_swap/
    // - https://docs.rs/arc-atomic/latest/arc_atomic/
    // - https://www.reddit.com/r/rust/comments/1bilk82/announcing_aarc_010_atomic_variants_of_arc_and/
    with_two_fields: ArcSwap<WithTwoFields>,
}

impl Atomics {
    pub fn sample_index(&self) -> usize {
        self.sample_index.load(Ordering::Acquire)
    }
    pub fn set_sample_index(&self, value: usize) {
        self.sample_index.store(value, Ordering::Release);
    }

    pub fn with_two_fields(&self) -> Arc<WithTwoFields> {
        self.with_two_fields.load_full()
    }
    pub fn set_with_two_fields(&self, foo: f32, bar: f32) {
        self.with_two_fields
            .store(Arc::new(WithTwoFields { foo, bar }));
    }
}

// ----------------------------------------------------------------------------
// Command + control input / output split
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub enum Command {
    InitSequencer(),
    SetFrequency(f32),
}

pub type CommandInput = <HeapRb<Command> as Split>::Prod;
pub type CommandOutput = <HeapRb<Command> as Split>::Cons;

pub struct SequencerControlInput {
    pub atomics: Arc<Atomics>,
    // TODO: Most likely the produce/input side actually doesn't need interior mutability?
    command_producer: Mutex<CommandInput>,
}

static_assertions::assert_impl_all!(SequencerControlInput: Send);
static_assertions::assert_impl_all!(SequencerControlInput: Sync);

impl SequencerControlInput {
    pub fn init_sequencer(&self) {
        let mut command_producer = self.command_producer.lock().unwrap();
        command_producer.try_push(Command::InitSequencer()).unwrap();
    }

    pub fn set_frequency(&self, freq: f32) {
        let mut command_producer = self.command_producer.lock().unwrap();
        command_producer
            .try_push(Command::SetFrequency(freq))
            .unwrap();
    }
}

pub struct SequencerControlOutput {
    pub atomics: Arc<Atomics>,
    command_consumer: CommandOutput,
}

static_assertions::assert_impl_all!(SequencerControlOutput: Send);
static_assertions::assert_not_impl_all!(SequencerControlOutput: Sync); // No need to be sync, send is all we need

impl SequencerControlOutput {
    pub fn try_pop(&mut self) -> Option<Command> {
        self.command_consumer.try_pop()
    }
}

/// Initializes the atomics and splits the command input/output buffer.
pub fn create_sequencer_control() -> (SequencerControlInput, SequencerControlOutput) {
    let atomics = Arc::new(Atomics {
        sample_index: AtomicUsize::new(0),
        with_two_fields: ArcSwap::new(Arc::new(WithTwoFields { foo: 1.0, bar: 2.0 })),
    });

    let commands = HeapRb::<Command>::new(4);
    let (command_producer, command_consumer) = commands.split();

    let control_in = SequencerControlInput {
        atomics: atomics.clone(),
        command_producer: Mutex::new(command_producer),
    };
    let control_out = SequencerControlOutput {
        atomics: atomics.clone(),
        command_consumer,
    };
    (control_in, control_out)
}

// ----------------------------------------------------------------------------
// Sequencer
// ----------------------------------------------------------------------------

/// A dummy sequencer implementation.
pub struct Sequencer {
    frequency: f32,
    sample_rate: f32,
    sample_index: usize,
    atomics: Arc<Atomics>,
    // Dummy phantom data to simulate a non-send / non-sync Sequencer.
    phantom: PhantomData<Rc<i32>>,
}

static_assertions::assert_not_impl_all!(Sequencer: Send);
static_assertions::assert_not_impl_all!(Sequencer: Sync);

impl Sequencer {
    pub fn new(sample_rate: f32, atomics: Arc<Atomics>) -> Self {
        Self {
            frequency: 220.0,
            sample_rate,
            sample_index: 0,
            atomics,
            phantom: PhantomData,
        }
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn render_audio(&mut self, num_to_generate: usize, buffers: &mut [&mut [f32]]) {
        assert_eq!(buffers.len(), 2);

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
        self.atomics.set_sample_index(self.sample_index);

        let with_two_fields = self.atomics.with_two_fields();

        println!(
            "[{:08}] render_audio: {} (foo: {}, bar: {})",
            Os::singleton().get_thread_caller_id(),
            self.sample_index,
            with_two_fields.foo,
            with_two_fields.bar,
        );
    }
}
