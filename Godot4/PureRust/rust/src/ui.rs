use godot::engine::control::{GrowDirection, LayoutPreset};
use godot::engine::global::Side;
use godot::engine::node::InternalMode;
use godot::engine::{Control, Engine, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Ui {
    #[base]
    base: Base<Control>,
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
impl GodotExt for Ui {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("init called");

        set_full_rect(&mut base);
        // set_full_rect_generic(base.deref_mut());
        // set_full_rect_generic(&mut *base);
        // set_full_rect_generic(base.bind_mut());

        Self { base, time: 0.0 }
    }

    fn ready(&mut self) {
        godot_print!("ready called");
        let mut label = Label::new_alloc();
        label.set_text("Hello World".into());

        // Pointless, but just to demonstrate generic call:
        set_full_rect_generic(&mut label);

        self.base
            .add_child(label.upcast(), false, InternalMode::INTERNAL_MODE_DISABLED);
    }

    fn process(&mut self, delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            self.time += delta;
            // let mut label = self.base.get_node_as::<Label>("../Label");
            // label.set_text(format!("Hello world: {}", self.time).into());
            // label.set_rotation_degrees(self.time * 90.0);
            godot_print!("process called {}", self.time);
        }
    }
}

fn set_full_rect(control: &mut Gd<Control>) {
    control.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT, false);
    control.set_anchor(Side::SIDE_RIGHT, 1.0, false, true);
    control.set_anchor(Side::SIDE_BOTTOM, 1.0, false, true);
    control.set_h_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
    control.set_v_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
}

fn set_full_rect_generic<C>(control: &mut Gd<C>)
where
    C: GodotClass + Inherits<Control>,
{
    let mut control = control.share().upcast();
    control.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT, false);
    control.set_anchor(Side::SIDE_RIGHT, 1.0, false, true);
    control.set_anchor(Side::SIDE_BOTTOM, 1.0, false, true);
    control.set_h_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
    control.set_v_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
}
