use std::fs;
use std::sync::OnceLock;
use tera::{Context, Tera};

///
/// Approach inspired by:
/// https://cetra3.github.io/blog/drawing-svg-graphs-rust/
///

fn get_templates() -> &'static Tera {
    static TEMPLATES: OnceLock<Tera> = OnceLock::new();

    TEMPLATES.get_or_init(|| {
        // Tera::new("examples/basic/templates/**/*").expect("Failed to load svg templates")
        // Using the pattern:
        // https://stackoverflow.com/questions/57760236/why-cant-my-rust-code-load-a-tera-template
        // https://docs.rs/tera/latest/tera/struct.Tera.html#method.add_raw_template
        let mut tera = Tera::default();
        tera.add_raw_template("button.svg", include_str!("svgs/button.svg"))
            .unwrap();
        tera
    })
}

fn write_debug_output(filename: &str, content: &str) {
    let temp_dir = std::env::temp_dir().join("debug_svg_out");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
    let temp_file = temp_dir.join(filename);
    fs::write(temp_file, content).expect("Unable to write temporary svg file");
}

fn get_svg(filename: &str, context: &Context) -> String {
    let tera = get_templates();

    let svg = tera
        .render("button.svg", &context)
        .expect("Failed to render template");

    write_debug_output(filename, &svg);
    println!("{}", svg);

    svg
}

pub fn get_svg_button() -> String {
    let mut context = Context::new();
    context.insert("width", &100);
    context.insert("height", &100);
    context.insert("padding", &10);
    context.insert("name", "Hello World");
    get_svg("button.svg", &context)
}
