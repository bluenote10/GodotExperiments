use crate::utils;
use godot::engine::{AudioStreamGenerator, AudioStreamGeneratorPlayback, Engine};
use godot::prelude::*;

/*
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Sequencer {
    audio_stream_player: Gd<AudioStreamPlayer>,
    audio_stream_generator: Gd<AudioStreamGenerator>,
    audio_stream_generator_playback: Gd<AudioStreamGeneratorPlayback>,
    i: u64,
}
*/

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Sequencer {
    #[base]
    base: Base<Node>,
    i: u64,
    audio_refs: Option<AudioRefs>,
}

struct AudioRefs {
    audio_stream_player: Gd<AudioStreamPlayer>,
    audio_stream_generator: Gd<AudioStreamGenerator>,
    audio_stream_generator_playback: Gd<AudioStreamGeneratorPlayback>,
}

#[godot_api]
impl NodeVirtual for Sequencer {
    /*
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("Sequencer::init called");

        let mut audio_stream_player = AudioStreamPlayer::new_alloc();
        utils::gd_add_child!(base, audio_stream_player.share());

        let mut audio_stream_generator = AudioStreamGenerator::new();
        audio_stream_generator.set_mix_rate(44100.0);
        audio_stream_generator.set_buffer_length(0.1);

        audio_stream_player.set_stream(audio_stream_generator.share().upcast());

        // Play must be called before get_stream_playback
        audio_stream_player.play(0.0);

        let audio_stream_generator_playback = audio_stream_player
            .get_stream_playback()
            .unwrap()
            .cast::<AudioStreamGeneratorPlayback>();

        Self {
            audio_stream_player,
            audio_stream_generator,
            audio_stream_generator_playback,
            i: 0,
        }
    }
    */

    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Sequencer::init called");
        Self {
            base,
            audio_refs: None,
            i: 0,
        }
    }

    fn ready(&mut self) {
        godot_print!("Sequencer::ready called");

        // Observations:
        // - `get_stream_playback` can only be called after `play` has been called.
        // - `play` can only be called after the AudioStreamPlayer has been added to
        //   the node tree.
        // Therefore, we probably cannot do this in `init` but have to postpone to `ready`
        let mut audio_stream_player = AudioStreamPlayer::new_alloc();
        utils::gd_add_child!(self.base, audio_stream_player.share());

        let mut audio_stream_generator = AudioStreamGenerator::new();
        audio_stream_generator.set_mix_rate(44100.0);
        audio_stream_generator.set_buffer_length(0.1);

        audio_stream_player.set_stream(audio_stream_generator.share().upcast());

        // Play must be called before get_stream_playback
        audio_stream_player.play();

        let audio_stream_generator_playback = audio_stream_player
            .get_stream_playback()
            .unwrap()
            .cast::<AudioStreamGeneratorPlayback>();

        godot_print!(
            "AudioStreamGeneratorPlayback = {:?}",
            audio_stream_generator_playback
        );
        godot_print!(
            "audio_stream_generator.get_length() = {}",
            audio_stream_generator.get_length()
        );

        self.audio_refs = Some(AudioRefs {
            audio_stream_player,
            audio_stream_generator,
            audio_stream_generator_playback,
        })
    }

    fn process(&mut self, _delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            let Some(audio_refs) = &mut self.audio_refs else {
                return;
            };
            // let playback = &mut audio_refs.audio_stream_generator_playback;

            let mut playback = audio_refs
                .audio_stream_player
                .get_stream_playback()
                .unwrap()
                .cast::<AudioStreamGeneratorPlayback>();

            godot_print!("playback = {:?}", playback);

            let to_fill = playback.get_frames_available() as usize;

            if to_fill > 0 {
                godot_print!("Filling buffer of length {}", to_fill);
                let mut buffer = PackedVector2Array::new();

                let freq = 440.0;

                for _ in 0..to_fill {
                    let phase = (2.0 * std::f32::consts::PI * self.i as f32 / 44100.0 * freq).sin();
                    buffer.push(Vector2::new(phase as f32, phase as f32));
                    self.i += 1;
                }

                playback.push_buffer(buffer);
            }
            // Strange: This print makes it more likely that it works?!
            // godot_print!("done");
        }
    }
}

/*
use crate::utils;
use godot::engine::{AudioStreamGenerator, AudioStreamGeneratorPlayback, Engine};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Sequencer {
    i: u64,
    audio_refs: Option<AudioRefs,
}

struct AudioRefs {
    audio_stream_player: Gd<AudioStreamPlayer>,
    audio_stream_generator: Gd<AudioStreamGenerator>,
    audio_stream_generator_playback: Gd<AudioStreamGeneratorPlayback>,
}

#[godot_api]
impl GodotExt for Sequencer {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("Sequencer::init called");
        Self {
            audio_refs: None,
            i: 0,
        }
    }

    fn ready(&mut self) {
        godot_print!("Sequencer::ready called");

        ///
        /// Observations:
        /// - `get_stream_playback` can only be called after `play` has been called.
        /// - `play` can only be called after the AudioStreamPlayer has been added to
        ///   the node tree.
        /// Therefore, we probably cannot do this in `init` but have to postpone to `ready`

        let mut audio_stream_player = AudioStreamPlayer::new_alloc();
        utils::gd_add_child!(base, audio_stream_player.share());

        let mut audio_stream_generator = AudioStreamGenerator::new();
        audio_stream_generator.set_mix_rate(44100.0);
        audio_stream_generator.set_buffer_length(0.1);

        audio_stream_player.set_stream(audio_stream_generator.share().upcast());

        // Play must be called before get_stream_playback
        audio_stream_player.play(0.0);

        let audio_stream_generator_playback = audio_stream_player
            .get_stream_playback()
            .unwrap()
            .cast::<AudioStreamGeneratorPlayback>();


        self.audio_refs = Some(AudioRefs{
            audio_stream_player,
            audio_stream_generator,
            audio_stream_generator_playback,
        })
    }

    fn process(&mut self, _delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            let playback = &mut self.audio_stream_generator_playback;

            let to_fill = playback.get_frames_available() as usize;

            if to_fill > 0 {
                let mut buffer = PackedVector2Array::new();

                let freq = 440.0;

                for _ in 0..to_fill {
                    let phase = (2.0 * std::f32::consts::PI * self.i as f32 / 44100.0 * freq).sin();
                    buffer.push(Vector2::new(phase as f32, phase as f32));
                    self.i += 1;
                }

                playback.push_buffer(buffer.clone());
            }
        }
    }
}
*/
