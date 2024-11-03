use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use godot::classes::Os;

pub struct SequencerInfo {
    sample_index: AtomicUsize,
}

impl SequencerInfo {
    pub fn sample_index(&self) -> usize {
        self.sample_index.load(Ordering::Acquire)
    }
}

pub struct Sequencer {
    sample_rate: f32,
    sample_index: usize,
    shared: Arc<SequencerInfo>,
}

const FREQUENCY: f32 = 220.0;

impl Sequencer {
    pub fn new(sample_rate: f32) -> Self {
        let shared = Arc::new(SequencerInfo {
            sample_index: AtomicUsize::new(0),
        });

        Self {
            sample_rate,
            sample_index: 0,
            shared,
        }
    }

    pub fn get_sequencer_info(&self) -> Arc<SequencerInfo> {
        self.shared.clone()
    }

    pub fn render_audio(&mut self, num_to_generate: usize, buffers: &mut [&mut [f32]]) {
        assert_eq!(buffers.len(), 2);
        for i in 0..num_to_generate {
            let phase = 2.0 * std::f32::consts::PI * FREQUENCY * (self.sample_index as f32)
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
