use godot::engine::{CanvasLayer, CanvasLayerVirtual, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Ui {
    #[base]
    base: Base<CanvasLayer>,
    time: f64,
}

#[godot_api]
impl Ui {
    #[func]
    pub fn hello_world(&self, text: GodotString) {
        godot_print!("Hello world: {}", text);
    }
}

#[godot_api]
impl CanvasLayerVirtual for Ui {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base, time: 0.0 }
    }

    fn process(&mut self, delta: f64) {
        self.time += delta;
        let mut label = self.base.get_node_as::<Label>("../Label");
        label.set_text(format!("Hello world: {}", self.time).into());
    }
}
