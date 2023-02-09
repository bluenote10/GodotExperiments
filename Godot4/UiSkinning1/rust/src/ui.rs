use crate::utils::{gd_add_child, set_full_rect, set_full_rect_generic};
use godot::engine::text_server::{FontAntialiasing, Hinting, SubpixelPositioning};
use godot::engine::{
    Button, Control, Engine, FontFile, HBoxContainer, Label, MarginContainer, Theme, VBoxContainer,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Ui {
    #[base]
    base: Base<Control>,
    time: f64,
}

#[godot_api]
impl GodotExt for Ui {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("Ui::init called");

        set_full_rect(&mut base);

        Self { base, time: 0.0 }
    }

    fn ready(&mut self) {
        godot_print!("Ui::ready called");

        //let font = FontFile::new();
        let mut font = load::<FontFile>("res://font/Ubuntu-R.ttf");
        font.set_antialiasing(FontAntialiasing::FONT_ANTIALIASING_LCD);
        font.set_hinting(Hinting::HINTING_LIGHT);
        font.set_subpixel_positioning(SubpixelPositioning::SUBPIXEL_POSITIONING_ONE_QUARTER);
        godot_print!("{}", font);

        let mut theme = Theme::new();
        theme.set_color(
            "font_color".into(),
            "Button".into(),
            Color::new(1.0, 0.0, 0.0, 1.0),
        );
        //theme.set_font("font".into(), "Button".into(), font.share().upcast());
        theme.set_default_font(font.upcast());
        theme.set_default_font_size(14);
        self.base.set_theme(theme);

        let mut vbox = VBoxContainer::new_alloc();
        set_full_rect_generic(&mut vbox);

        let mut label = Label::new_alloc();
        label.set_text("Hello World".into());

        // Pointless, but just to demonstrate generic call:
        set_full_rect_generic(&mut label);

        gd_add_child!(vbox, label);

        // let another_node = Gd::<AnotherNode>::with_base(AnotherNode::init);
        let another_node = Gd::<AnotherNode>::new_default(); // This seems to call AnotherNode::init implicitly, which is nice.
        gd_add_child!(vbox, another_node);

        gd_add_child!(vbox, {
            let mut button = Button::new_alloc();
            button.set_text("Ok".into());
            button
        });

        gd_add_child!(
            vbox,
            ({
                let mut hbox = HBoxContainer::new_alloc();
                gd_add_child!(
                    hbox,
                    ({
                        let mut button = Button::new_alloc();
                        button.set_text("Ok".into());
                        button
                    })
                );
                gd_add_child!(
                    hbox,
                    ({
                        let mut button = Button::new_alloc();
                        button.set_text("Cancel".into());
                        button
                    })
                );
                hbox
            })
        );

        gd_add_child!(self.base, {
            let mut margin_container = MarginContainer::new_alloc();
            margin_container.add_theme_constant_override("margin_top".into(), 20);
            margin_container.add_theme_constant_override("margin_left".into(), 20);
            margin_container.add_theme_constant_override("margin_bottom".into(), 20);
            margin_container.add_theme_constant_override("margin_right".into(), 20);
            gd_add_child!(margin_container, vbox);
            margin_container
        });
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
    #[export(
        getter = "get_rotation",
        setter = "set_rotation",
        variant_type = "::godot::sys::VariantType::Float" // Int, String, Bool, ...
    )]
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
        self.base.set_rotation_degrees(rotation)
    }
}

#[godot_api]
impl GodotExt for AnotherNode {
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
