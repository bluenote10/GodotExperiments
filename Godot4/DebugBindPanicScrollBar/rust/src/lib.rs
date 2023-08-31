use godot::engine::control::LayoutPreset;
use godot::engine::global::Side;
use godot::engine::notify::ControlNotification;
use godot::engine::{Control, ControlVirtual, Font, SystemFont, VScrollBar};
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

        let mut item_list = Gd::<SimpleItemList>::new_default();
        item_list.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT);

        item_list
            .bind_mut()
            .set_items((1..=100).map(|i| format!("Item {i}")).collect());

        base.add_child(item_list.upcast());

        Self { _base: base }
    }
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct SimpleItemList {
    #[base]
    base: Base<Control>,
    font: Gd<Font>,
    item_height: i32,
    scroll_bar: Gd<VScrollBar>,
    items: Vec<String>,
    shape_check_required: bool,
}

#[godot_api]
impl ControlVirtual for SimpleItemList {
    fn init(mut base: Base<<Self as godot::prelude::GodotClass>::Base>) -> Self {
        let mut scroll_bar = VScrollBar::new_alloc();
        scroll_bar.connect("value_changed".into(), base.callable("on_scroll_changed"));

        base.add_child(scroll_bar.share().upcast());

        base.set_clip_contents(true);

        Self {
            base,
            font: SystemFont::new().upcast(),
            item_height: 20,
            scroll_bar,
            items: vec![],
            shape_check_required: true,
        }
    }

    fn on_notification(&mut self, what: ControlNotification) {
        godot_print!("on_notification: {what:?}");
        match what {
            ControlNotification::Resized => {
                self.shape_check_required = true;
                self.base.queue_redraw();
            }
            ControlNotification::DrawOrNodeRecacheRequested => {
                self.draw_contents();
            }
            _ => (),
        }
    }
}

#[godot_api]
impl SimpleItemList {
    #[func]
    fn on_scroll_changed(&mut self) {
        godot_print!("on_scroll_changed (queue_redraw)");
        self.base.queue_redraw()
    }
}

impl SimpleItemList {
    pub fn ready(&mut self) {
        godot_print!("ready");
        self.scroll_bar.set_value(0.0);
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        godot_print!("set_items (queue_redraw)");
        self.items = items;
        self.shape_check_required = true;
        self.base.queue_redraw()
    }

    fn check_shape_changed(&mut self) {
        if !self.shape_check_required {
            return;
        }
        godot_print!("check_shape_changed");
        //self.scroll_bar.hide();

        let size = self.base.get_size();
        let height = size.y;

        let virtual_height = self.item_height * self.items.len() as i32;

        self.scroll_bar.set_page(height as f64);
        self.scroll_bar
            .set_max(virtual_height as f64 - height as f64);

        self.shape_check_required = false;
    }

    fn draw_contents(&mut self) {
        godot_print!("draw_contents");

        self.check_shape_changed();

        // Layout scroll bar
        let scroll_bar_minwidth = self.scroll_bar.get_minimum_size().x;
        self.scroll_bar
            .set_anchor_and_offset(Side::SIDE_RIGHT, 1.0, 0.0);
        self.scroll_bar
            .set_anchor_and_offset(Side::SIDE_LEFT, 1.0, -scroll_bar_minwidth);
        self.scroll_bar
            .set_anchor_and_offset(Side::SIDE_TOP, 0.0, 0.0);
        self.scroll_bar
            .set_anchor_and_offset(Side::SIDE_BOTTOM, 1.0, 0.0);
        let mut offset = -self.scroll_bar.get_value();

        for item in &self.items {
            self.base
                .draw_string_ex(
                    self.font.share(),
                    Vector2 {
                        x: 0.0,
                        y: offset as f32 + self.item_height as f32 / 2.0,
                    },
                    item.into(),
                )
                .font_size(14)
                .modulate(Color::from_rgb(0.1, 0.1, 0.1))
                .done();
            offset += self.item_height as f64;
        }
    }
}
