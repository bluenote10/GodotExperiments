mod reactive;

use godot::engine::control::LayoutPreset;
use godot::engine::{Button, Control, IControl, Label, VBoxContainer};
use godot::prelude::*;

pub use reactive::{Consumer, Dynamic, IntoConsumer, OnChange};

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TreeRoot {
    _base: Base<Control>,
}

#[godot_api]
impl IControl for TreeRoot {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("init");
        base.to_gd().set_anchors_preset(LayoutPreset::FULL_RECT);

        base.to_gd().add_child(Counter::new_alloc().upcast());

        Self { _base: base }
    }
}

// ----------------------------------------------------------------------------
// Classic counter example
// ----------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ClassicCounter {
    base: Base<Control>,
    label: Gd<Label>,
    count: i32,
}

impl ClassicCounter {
    fn update_label(&mut self) {
        self.label
            .set_text(format!("Counter: {}", self.count).into());
    }
}

#[godot_api]
impl ClassicCounter {
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
impl IControl for ClassicCounter {
    fn init(base: Base<Self::Base>) -> Self {
        let mut vbox = VBoxContainer::new_alloc();

        let label = Label::new_alloc();
        vbox.add_child(label.clone().upcast());

        let mut button_inc = Button::new_alloc();
        button_inc.set_text("Inc".into());
        button_inc.connect(
            "pressed".into(),
            base.to_gd().callable("on_inc_button_pressed"),
        );
        vbox.add_child(button_inc.clone().upcast());

        let mut button_dec = Button::new_alloc();
        button_dec.set_text("Dec".into());
        button_dec.connect(
            "pressed".into(),
            base.to_gd().callable("on_dec_button_pressed"),
        );
        vbox.add_child(button_dec.clone().upcast());

        base.to_gd().add_child(vbox.clone().upcast());

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

// ----------------------------------------------------------------------------
// Counter example based on Dynamic<T>
// ----------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Counter {
    base: Base<Control>,
    label: Gd<Label>,
    count: Dynamic<i32>,
    count_consumer: Consumer<i32>,
}

#[godot_api]
impl Counter {
    // Note that there is no need for manually calling the `self.update_label()`.

    #[func]
    fn on_inc_button_pressed(&mut self) {
        godot_print!("inc pressed");
        self.count.update(|x| x + 1);
    }

    #[func]
    fn on_dec_button_pressed(&mut self) {
        godot_print!("dec pressed");
        self.count.update(|x| x - 1);
    }
}

#[godot_api]
impl IControl for Counter {
    fn init(base: Base<Self::Base>) -> Self {
        let count = Dynamic::new(0);
        let count_consumer = count.into_consumer();

        let mut vbox = VBoxContainer::new_alloc();

        let label = Label::new_alloc();
        vbox.add_child(label.clone().upcast());

        let mut button_inc = Button::new_alloc();
        button_inc.set_text("Inc".into());
        button_inc.connect(
            "pressed".into(),
            base.to_gd().callable("on_inc_button_pressed"),
        );
        vbox.add_child(button_inc.clone().upcast());

        let mut button_dec = Button::new_alloc();
        button_dec.set_text("Dec".into());
        button_dec.connect(
            "pressed".into(),
            base.to_gd().callable("on_dec_button_pressed"),
        );
        vbox.add_child(button_dec.clone().upcast());

        vbox.add_child(Gd::<Child>::from_init_fn(|base| Child::new(base, count.clone())).upcast());

        base.to_gd().add_child(vbox.clone().upcast());

        Self {
            base,
            label,
            count,
            count_consumer,
        }
    }

    fn process(&mut self, _delta: f64) {
        self.count_consumer.on_change(|c| {
            println!("Updating label to: {}", c);
            self.label
                .set_text(format!("Counter (via process): {}", c).into());
        });
    }
}

// ----------------------------------------------------------------------------
// Child demo
// ----------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=Control, no_init)]
pub struct Child {
    base: Base<Control>,
    label: Gd<Label>,
    count: Consumer<i32>,
}

impl Child {
    fn new(base: Base<<Child as godot::prelude::GodotClass>::Base>, count: Dynamic<i32>) -> Self {
        let mut vbox = VBoxContainer::new_alloc();

        let label = Label::new_alloc();
        vbox.add_child(label.clone().upcast());

        base.to_gd().add_child(vbox.clone().upcast());

        Self {
            base,
            label,
            count: count.into_consumer(),
        }
    }
}

#[godot_api]
impl IControl for Child {
    fn process(&mut self, _delta: f64) {
        self.count.on_change(|c| {
            println!("Updating child label to: {}", c);
            self.label.set_text(format!("Child counter: {}", c).into());
        });

        // TODO: How to react to changes in *multiple* dependencies? Like handle change
        // if either counter_a or counter_b changes? We'd somehow need to pass a list of
        // dependencies similar to how it is done in React?
    }
}
