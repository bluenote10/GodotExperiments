use crate::sequencer::Sequencer;
use crate::utils::gd_add_child;
use godot::engine::{Control, ControlVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Ui {
    #[base]
    base: Base<Control>,
    time: f64,
}

#[godot_api]
impl ControlVirtual for Ui {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("Ui::init called");
        Self { base, time: 0.0 }
    }

    fn ready(&mut self) {
        godot_print!("Ui::ready called");
        let sequencer = Gd::<Sequencer>::new_default();
        gd_add_child!(self.base, sequencer);
    }
}
