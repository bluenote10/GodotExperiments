use godot::engine::control::LayoutPreset;
use godot::engine::{Control, IControl, Label};
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

// Works
#[godot_api]
impl IControl for TreeRoot {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("init");

        base.to_gd()
            .set_anchors_preset(LayoutPreset::PRESET_FULL_RECT);

        let mut label = Label::new_alloc();
        label.set_text("Hello World".into());

        base.to_gd().add_child(label.upcast());

        Self { _base: base }
    }
}

/*
// Crashes / ugly
#[godot_api]
impl IControl for TreeRoot {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("init");

        let slf = Self { _base: base };

        slf.to_gd()
            .set_anchors_preset(LayoutPreset::PRESET_FULL_RECT);

        let mut label = Label::new_alloc();
        label.set_text("Hello World".into());

        slf.to_gd().add_child(label.upcast());

        slf
    }
}
*/

/*
#[godot_api]
// Crashes / ugly
impl IControl for TreeRoot {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("init");

        let mut slf = Self { _base: base };

        slf.base_mut()
            .set_anchors_preset(LayoutPreset::PRESET_FULL_RECT);

        let mut label = Label::new_alloc();
        label.set_text("Hello World".into());

        slf.base_mut().add_child(label.upcast());

        slf
    }
}
*/
