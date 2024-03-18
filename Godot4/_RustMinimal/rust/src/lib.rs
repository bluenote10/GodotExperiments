use godot::engine::control::LayoutPreset;
use godot::engine::{Control, IControl, Label};
use godot::prelude::*;

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

        let mut label = Label::new_alloc();
        label.set_text("Hello World".into());

        base.to_gd().add_child(label.upcast());

        Self { _base: base }
    }
}
