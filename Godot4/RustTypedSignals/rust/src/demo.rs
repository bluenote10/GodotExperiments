use godot::engine::control::LayoutPreset;
use godot::engine::{Button, Control, ControlVirtual};
use godot::prelude::*;

use crate::signal::{Callback, Signal};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TreeRoot {
    #[base]
    base: Base<Control>,
    child: Gd<Child>,
    counter: i32,
}

#[godot_api]
impl ControlVirtual for TreeRoot {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("init");
        base.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT);

        let child = Gd::<Child>::new_default();
        base.add_child(child.share().upcast());

        child
            .share()
            .bind_mut()
            .click_signal
            .connect(Callback::new(base.share(), |node, i| {
                let mut node = node.share().cast::<Self>();
                godot_print!("{node:?} {i}");
                let mut this = node.bind_mut();
                this.counter += 1;
                godot_print!("counter (binding 1): {}", this.counter);
            }));

        Self {
            base,
            child,
            counter: 0,
        }
    }

    fn ready(&mut self) {
        self.child
            .bind_mut()
            .click_signal
            .connect(Callback::new(self.base.share(), |node, i| {
                let mut node = node.share().cast::<Self>();
                godot_print!("{node:?} {i}");
                let mut this = node.bind_mut();
                this.counter += 1;
                godot_print!("counter (binding 2): {}", this.counter);
            }))
    }
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Child {
    pub click_signal: Signal<i32>,
}

#[godot_api]
impl ControlVirtual for Child {
    fn init(mut base: Base<Self::Base>) -> Self {
        base.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT);

        let mut button = Button::new_alloc();
        button.set_text("Click Me".into());
        button.connect("pressed".into(), base.callable("on_pressed"));

        base.add_child(button.upcast());

        Self {
            click_signal: Signal::new(),
        }
    }
}

#[godot_api]
impl Child {
    #[func]
    fn on_pressed(&mut self) {
        godot_print!("Button pressed");
        self.click_signal.emit(42);
    }
}
