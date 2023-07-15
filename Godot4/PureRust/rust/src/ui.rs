use crate::instantiation_order::InstantiationOrder;
use crate::render_stats::RenderStats;
use crate::utils::{gd_add_child, set_full_rect, set_full_rect_generic};
use godot::engine::{Control, ControlVirtual, Engine, Label, LabelVirtual, VBoxContainer};
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

        set_full_rect(&mut base);
        // set_full_rect_generic(base.deref_mut());
        // set_full_rect_generic(&mut *base);
        // set_full_rect_generic(base.bind_mut());

        Self { base, time: 0.0 }
    }

    fn ready(&mut self) {
        godot_print!("Ui::ready called");

        let mut vbox = VBoxContainer::new_alloc();

        let mut label = Label::new_alloc();
        label.set_text("Hello World".into());

        // Pointless, but just to demonstrate generic call:
        set_full_rect_generic(&mut label);

        gd_add_child!(vbox, label);

        // let another_node = Gd::<AnotherNode>::with_base(AnotherNode::init);
        let another_node = Gd::<AnotherNode>::new_default(); // This seems to call AnotherNode::init implicitly, which is nice.
        gd_add_child!(vbox, another_node);

        gd_add_child!(self.base, vbox);

        let render_stats = Gd::<RenderStats>::new_default();
        gd_add_child!(self.base, render_stats);

        let instantiation_order = Gd::<InstantiationOrder>::new_default();
        gd_add_child!(self.base, instantiation_order);
    }

    fn process(&mut self, delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            self.time += delta;
            // let mut label = self.base.get_node_as::<Label>("../Label");
            // label.set_text(format!("Hello world: {}", self.time).into());
            // label.set_rotation_degrees(self.time * 90.0);
            // godot_print!("Ui::process called {}", self.time);
        }
    }
}

#[derive(GodotClass)]
#[class(base=Label)]
pub struct AnotherNode {
    #[base]
    base: Base<Label>,
    /* // Used to work, syntax changed?
    #[export(
        getter = "get_rotation",
        setter = "set_rotation",
        variant_type = "::godot::sys::VariantType::Float" // Int, String, Bool, ...
    )]
    */
    rotation: f64,
}

#[godot_api]
impl AnotherNode {
    #[func]
    pub fn get_rotation(&self) -> f64 {
        self.rotation
    }

    #[func]
    pub fn set_rotation(&mut self, rotation: f64) {
        self.rotation = rotation;
        self.base.set_rotation_degrees(rotation as f32)
    }
}

#[godot_api]
impl LabelVirtual for AnotherNode {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("AnotherNode::init called");
        base.set_text("Another Node".into());
        Self {
            base,
            rotation: 0.0,
        }
    }

    fn ready(&mut self) {
        godot_print!("AnotherNode::ready called");
    }
}
