use godot::engine::control::LayoutPreset;
use godot::engine::{Control, ControlVirtual, Label};
use godot::prelude::*;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TreeRoot {
    #[base]
    _base: Base<Control>,
}

#[godot_api]
impl ControlVirtual for TreeRoot {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("init");
        base.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT);

        let mut label = Label::new_alloc();
        label.set_text("Hello World".into());

        base.add_child(label.upcast());

        Self { _base: base }
    }
}
