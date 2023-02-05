use crate::utils;
use godot::engine::control::GrowDirection;
use godot::engine::global::HorizontalAlignment;
use godot::engine::{Engine, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct RenderStats {
    label: Gd<Label>,
}

#[godot_api]
impl GodotExt for RenderStats {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("RenderStats::init called");

        let mut label = Label::new_alloc();

        // The trick to right-aligning a label is to set its anchor to the
        // right, change its alignment mode to right aligned, and (IMPORTANT)
        // also change the horizontal grow direction from 'end' (growing
        // to the right) to 'begin' (growing to the left).
        utils::gd_set_anchor_right!(label, 1.0);
        label.set_horizontal_alignment(HorizontalAlignment::HORIZONTAL_ALIGNMENT_RIGHT);
        label.set_h_grow_direction(GrowDirection::GROW_DIRECTION_BEGIN);
        label.add_theme_color_override("font_color".into(), Color::new(0.6, 0.6, 0.9, 1.0));

        utils::gd_add_child!(base, label.share());

        Self { label }
    }

    fn ready(&mut self) {
        godot_print!("RenderStats::ready called");
    }

    /*
    fn _input(&mut self) {
        godot_print!("RenderStats::_input called");
    }
    */

    fn process(&mut self, _delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            self.label
                .set_text(format!("FPS: {}", Engine::singleton().get_frames_per_second()).into());
        }
    }
}
