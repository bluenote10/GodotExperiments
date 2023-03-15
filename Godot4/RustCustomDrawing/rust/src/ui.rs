use crate::render_stats::RenderStats;
use crate::utils::gd_add_child;
use godot::engine::{Control, ControlVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Ui {
    #[base]
    base: Base<Control>,
}

#[godot_api]
impl ControlVirtual for Ui {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Ui::init called");
        Self { base }
    }

    fn ready(&mut self) {
        let render_stats = Gd::<RenderStats>::new_default();
        gd_add_child!(self.base, render_stats);
    }

    fn process(&mut self, _delta: f64) {}

    fn draw(&mut self) {
        self.base.draw_line(
            Vector2::new(0.5, 0.5),
            Vector2::new(100.5, 100.5),
            Color::from_rgb(0.0, 0.0, 1.0),
            0.5,
            true,
        );
    }
}
