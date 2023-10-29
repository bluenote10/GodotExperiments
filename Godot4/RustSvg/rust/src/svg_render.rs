use std::fs;

use godot::engine::Image;
use godot::prelude::*;
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{self, TreeParsing, TreeWriting, XmlOptions};

fn write_debug_output(filename: &str, content: &[u8]) {
    let temp_dir = std::env::temp_dir().join("debug_svg_out");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
    let temp_file = temp_dir.join(filename);
    fs::write(temp_file, content).expect("Unable to write temporary svg file");
}

pub fn render_svg(svg: &str) -> Gd<Image> {
    let options = usvg::Options::default();
    let svg_tree = usvg::Tree::from_str(svg, &options).expect("Failed to parse svg");

    let show_parsed_tree = false;
    if show_parsed_tree {
        println!("{}", svg_tree.to_string(&XmlOptions::default()));
    }

    let render_tree = resvg::Tree::from_usvg(&svg_tree);

    let mut pixmap =
        Pixmap::new(svg_tree.size.width() as u32, svg_tree.size.height() as u32).unwrap();
    render_tree.render(Transform::identity(), &mut pixmap.as_mut());

    let png_data = pixmap.encode_png().expect("Failed to convert to png");

    write_debug_output("button.png", &png_data);

    let packed_byte_array = PackedByteArray::from(png_data.as_slice());
    let mut image = Image::new();
    image.load_png_from_buffer(packed_byte_array);
    image.generate_mipmaps();

    image
}
