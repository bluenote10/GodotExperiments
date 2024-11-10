use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use godot::classes::Os;
use ringbuf::traits::{Consumer, Producer, Split};
use ringbuf::HeapRb;

// ----------------------------------------------------------------------------
// Shared atomics
// ----------------------------------------------------------------------------

pub struct Atomics {
    sample_index: AtomicUsize,
}

impl Atomics {
    pub fn sample_index(&self) -> usize {
        self.sample_index.load(Ordering::Acquire)
    }

    pub fn set_sample_index(&self, value: usize) {
        self.sample_index.store(value, Ordering::Release);
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

        println!(
            "[{:08}] render_audio: {}",
            Os::singleton().get_thread_caller_id(),
            self.sample_index,
        );
    }
}
