use crate::instantiation_order::InstantiationOrder;
use crate::render_stats::RenderStats;
use crate::utils::{gd_add_child, set_full_rect, set_full_rect_generic};
use godot::engine::{Button, Control, ControlVirtual, Engine, Label, LabelVirtual, VBoxContainer};
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

        let counter = Gd::<Counter>::new_default();
        gd_add_child!(vbox, counter);

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

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Counter {
    #[base]
    base: Base<Control>,
    label: Gd<Label>,
    count: i32,
}

impl Counter {
    fn update_label(&mut self) {
        self.label
            .set_text(format!("Counter: {}", self.count).into());
    }
}

#[godot_api]
impl Counter {
    #[func]
    fn on_inc_button_pressed(&mut self) {
        godot_print!("inc pressed");
        self.count += 1;
        self.update_label();
    }

    #[func]
    fn on_dec_button_pressed(&mut self) {
        godot_print!("dec pressed");
        self.count -= 1;
        self.update_label();
    }
}

#[godot_api]
impl ControlVirtual for Counter {
    fn init(mut base: Base<Self::Base>) -> Self {
        let mut vbox = VBoxContainer::new_alloc();

        let label = Label::new_alloc();
        gd_add_child!(vbox, label.share());

        let mut button_inc = Button::new_alloc();
        button_inc.set_text("Inc".into());
        button_inc.connect(
            "pressed".into(),
            base.callable("on_inc_button_pressed"), //Callable::from_object_method(base.get_node_as::<Counter>("."), "on_inc_button_pressed"),
        );
        gd_add_child!(vbox, button_inc);

        let mut button_dec = Button::new_alloc();
        button_dec.set_text("Dec".into());
        button_dec.connect(
            "pressed".into(),
            base.callable("on_dec_button_pressed"), //Callable::from_object_method(base.get_node_as::<Counter>("."), "on_inc_button_pressed"),
        );
        gd_add_child!(vbox, button_dec);

        gd_add_child!(base, vbox);

        Self {
            base,
            label,
            count: 0,
        }
    }

    fn ready(&mut self) {
        self.update_label();
    }
}
