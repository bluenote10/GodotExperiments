use crate::render_stats::RenderStats;
use crate::utils::gd_add_child;
use godot::engine::{Sprite2D, SubViewport};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Ui {
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for Ui {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Ui::init called");
        Self { base }
    }

    fn ready(&mut self) {
        let render_stats = Gd::<RenderStats>::new_default();
        gd_add_child!(self.base, render_stats);

        let mut viewport = SubViewport::new_alloc();

        let custom_drawing_node_2d = Gd::<CustomDrawingNode2D>::new_default();
        gd_add_child!(viewport, custom_drawing_node_2d);

        gd_add_child!(self.base, viewport.share());

        let mut sprite = Sprite2D::new_alloc();
        sprite.set_centered(false);
        sprite.set_texture(viewport.get_texture().unwrap().upcast());
        gd_add_child!(self.base, sprite.share());
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct CustomDrawingNode2D {
    #[base]
    base: Base<Node2D>,
    time: f64,
}

#[godot_api]
impl Node2DVirtual for CustomDrawingNode2D {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base, time: 0.0 }
    }

    fn process(&mut self, delta: f64) {
        self.time += delta;
        self.queue_redraw();
    }

    fn draw(&mut self) {
        self.base.draw_line(
            Vector2::new(0.5, 0.5),
            Vector2::new(200.5, 0.5),
            Color::from_rgb(0.0, 1.0, 0.0),
            0.5,
            true,
        );
        self.base.draw_line(
            Vector2::new(0.5, 0.5),
            Vector2::new(0.5, 100.5),
            Color::from_rgb(0.0, 1.0, 0.0),
            0.5,
            true,
        );
        self.base.draw_line(
            Vector2::new(0.5, 0.5),
            Vector2::new(200.5, 100.5),
            Color::from_rgb(0.0, 1.0, 0.0),
            0.5,
            true,
        );

        let x_1 = 50.5;
        let y_1 = 50.5;
        let x_2 = x_1 + 50.0 * self.time.cos() as f32;
        let y_2 = y_1 + 50.0 * self.time.sin() as f32;
        self.base.draw_line(
            Vector2::new(x_1, y_1),
            Vector2::new(x_2, y_2),
            Color::from_rgb(1.0, 0.0, 0.0),
            2.0,
            true,
        );
    }
}
