#[macro_use]
extern crate gdnative as godot;
extern crate rand;
extern crate npy;

use godot::GodotString;
use godot::NodePath;
use godot::Vector2;
use rand::Rng;

//use std::fs::File;
use std::io::Read;
use npy::NpyData;

#[derive(godot::NativeClass)]
#[inherit(godot::Node2D)]
#[user_data(godot::user_data::ArcData<HelloWorld>)]
struct HelloWorld;

#[godot::methods]
impl HelloWorld {

    fn _init(_owner: godot::Node2D) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, mut owner: godot::Node2D) {
        godot_print!("hello, world.");
        godot_print!("{:?}", std::env::current_dir().unwrap());

        let mut buf = vec![];
        std::fs::File::open("plain.npy").unwrap().read_to_end(&mut buf).unwrap();

        let data: NpyData<f64> = NpyData::from_bytes(&buf).unwrap();
        for number in data {
            godot_print!("{}", number);
        }

        let size = 512;

        let mut image = godot::Image::new();
        image.create(size, size, false, godot::ImageFormat::Rgb8 as i64);

        image.lock();
        let mut rng = rand::thread_rng();
        for i in 0 .. size {
            for j in 0 .. size {
                let value = rng.gen_range(0.0f32, 1.0f32);
                image.set_pixel(i, j, godot::Color::rgba(value, value, value, 1f32));
            }
        }
        image.unlock();

        let mut texture = godot::ImageTexture::new();
        texture.create(
            size, size,
            image.get_format() as i64,
            0
            //godot::TextureFlags::FlagFilter as i64 | godot::TextureFlags::FlagAnisotropicFilter as i64
        );
        texture.set_data(Some(image));

        let mut sprite = godot::Sprite::new();

        unsafe {
            sprite.set_name(GodotString::from_str("Sprite"));
            sprite.set_texture(texture.cast());
            owner.add_child(Some(sprite.to_node()), true);
        }
    }

    #[export]
    fn _process(&self, mut owner: godot::Node2D, delta: f64) {
        unsafe {
            let sprite: Option<godot::Sprite> = owner.get_node(NodePath::from_str("Sprite")).and_then(|node| node.cast());
            if let Some(sprite) = sprite {
                // godot_print!("Sprite: {:?}", sprite);
                if let Some(mut texture) = sprite.get_texture().and_then(|texture| texture.cast::<godot::ImageTexture>()) {
                    if let Some(mut image) = texture.get_data() {
                        image.lock();
                        let mut rng = rand::thread_rng();
                        for i in 0 .. image.get_width() {
                            for j in 0 .. image.get_height() {
                                let value = rng.gen_range(0.0f32, 1.0f32);
                                image.set_pixel(i, j, godot::Color::rgba(value, value, value, 1f32));
                            }
                        }
                        image.unlock();
                        texture.set_data(Some(image));
                    }
                }
            }
        }
    }

}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
