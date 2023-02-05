use godot::engine::control::{GrowDirection, LayoutPreset};
use godot::engine::global::Side;
use godot::engine::Control;
use godot::prelude::*;

macro_rules! gd_add_child {
    ($base:expr, $child:expr) => {
        $base.add_child(
            $child.upcast(),
            false,
            godot::engine::node::InternalMode::INTERNAL_MODE_DISABLED,
        );
    };
}

pub(crate) use gd_add_child;

pub fn set_full_rect(control: &mut Gd<Control>) {
    control.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT, false);
    control.set_anchor(Side::SIDE_RIGHT, 1.0, false, true);
    control.set_anchor(Side::SIDE_BOTTOM, 1.0, false, true);
    control.set_h_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
    control.set_v_grow_direction(GrowDirection::GROW_DIRECTION_BOTH);
}

pub fn set_full_rect_generic<C>(control: &mut Gd<C>)
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
