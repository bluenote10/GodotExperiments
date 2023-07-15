use crate::utils;
use godot::engine::control::GrowDirection;
use godot::engine::global::HorizontalAlignment;
use godot::engine::{Engine, InputEvent, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct RenderStats {
    #[base]
    base: Base<Node>,
    label: Option<Gd<Label>>,
}

#[godot_api]
impl NodeVirtual for RenderStats {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("RenderStats::init called");
        Self { base, label: None }
    }

    /*
    fn input(&mut self, event: Gd<InputEvent>) {
        godot_print!("RenderStats::input called");
        godot_print!("{:?}", event);
        # GDScript version
        if event is InputEventKey and event.scancode == KEY_ASCIICIRCUM and event.pressed and not event.echo:
            if self.label.is_some() {
                self.remove_label();
            } else {
                self.add_label();
            }
    }
    */

    fn process(&mut self, _delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            if let Some(label) = &mut self.label {
                label.set_text(
                    format!("FPS: {}", Engine::singleton().get_frames_per_second()).into(),
                );
            }
        }
    }
}

impl RenderStats {
    fn add_label(&mut self) {
        let mut label = Label::new_alloc();

        // The trick to right-aligning a label is to set its anchor to the
        // right, change its alignment mode to right aligned, and (IMPORTANT)
        // also change the horizontal grow direction from 'end' (growing
        // to the right) to 'begin' (growing to the left).
        utils::gd_set_anchor_right!(label, 1.0);
        label.set_horizontal_alignment(HorizontalAlignment::HORIZONTAL_ALIGNMENT_RIGHT);
        label.set_h_grow_direction(GrowDirection::GROW_DIRECTION_BEGIN);
        label.add_theme_color_override("font_color".into(), Color::from_rgb(0.6, 0.6, 0.9));

        utils::gd_add_child!(self.base, label.share());

        self.label = Some(label);
    }

    fn remove_label(&mut self) {
        if let Some(label) = &mut self.label {
            label.queue_free();
            self.label = None;
        }
    }
}
