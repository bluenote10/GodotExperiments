mod svg_gen;
mod svg_gen_rust;
mod svg_render;

use godot::engine::control::LayoutPreset;
use godot::engine::{Control, ControlVirtual, ImageTexture, Label};
use godot::prelude::*;

use crate::svg_gen::get_svg_button;
use crate::svg_render::render_svg;

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TreeRoot {
    #[base]
    base: Base<Control>,
    image_texture: Gd<ImageTexture>,
}

#[godot_api]
impl ControlVirtual for TreeRoot {
    fn init(mut base: Base<Self::Base>) -> Self {
        godot_print!("init");
        base.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT);

        let mut label = Label::new_alloc();
        label.set_text("Hello World".into());

        base.add_child(label.upcast());

        let svg = get_svg_button();
        let image = render_svg(&svg);
        println!("Image size: {} x {}", image.get_width(), image.get_height());

        let image_texture =
            ImageTexture::create_from_image(image).expect("Failed to create image texture");

        base.queue_redraw();

        Self {
            base,
            image_texture,
        }
    }

    fn draw(&mut self) {
        println!("Drawing...");
        self.base.draw_texture(
            self.image_texture.share().upcast(),
            Vector2 { x: 100.0, y: 100.0 },
        );
    }
}
