use godot::engine::ControlVirtual;
use godot::prelude::*;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TreeRoot {
    accum_f64: f64,
    accum_f32: f32,
}

#[godot_api]
impl ControlVirtual for TreeRoot {
    fn init(_base: Base<Self::Base>) -> Self {
        Self {
            accum_f64: 0.0,
            accum_f32: 0.0,
        }
    }

    fn process(&mut self, delta: f64) {
        let delta_f32 = delta as f32;
        self.accum_f32 += delta_f32;
        self.accum_f64 += delta;
        godot_print!(
            "accum_f32: {:.16}    accum_f64: {:.16}    diff: {:6.3} %",
            self.accum_f32,
            self.accum_f64,
            100.0 * (self.accum_f32 as f64 / self.accum_f64) - 100.0
        );
    }
}
