use crate::utils::{self, gd_add_child};
use godot::engine::control::GrowDirection;
use godot::engine::global::HorizontalAlignment;
use godot::engine::{Control, ControlVirtual, Engine, Label, LabelVirtual};
use godot::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Ui {
    #[base]
    base: Base<Control>,
    num_lines: i32,
}

#[godot_api]
impl ControlVirtual for Ui {
    fn init(mut base: Base<Self::Base>) -> Self {
        utils::set_full_rect_generic(&mut base);
        Self {
            base,
            num_lines: 5000,
        }
    }

    fn ready(&mut self) {
        let render_stats = Gd::<FpsLabel>::new_default();
        gd_add_child!(self.base, render_stats);
    }

    fn process(&mut self, _delta: f64) {
        self.queue_redraw();
    }

    fn draw(&mut self) {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(0.0..500.0);

        for _ in 0..self.num_lines {
            let x_1 = dist.sample(&mut rng);
            let y_1 = dist.sample(&mut rng);
            let x_2 = dist.sample(&mut rng);
            let y_2 = dist.sample(&mut rng);
            self.base.draw_line(
                Vector2::new(x_1, y_1),
                Vector2::new(x_2, y_2),
                Color::from_rgb(1.0, 0.0, 0.0),
                0.5,
                true,
            );
        }
    }
}

#[derive(GodotClass)]
#[class(base=Label)]
pub struct FpsLabel {
    #[base]
    base: Base<Label>,
}

#[godot_api]
impl LabelVirtual for FpsLabel {
    fn init(mut base: Base<Self::Base>) -> Self {
        utils::gd_set_anchor_right!(base, 1.0);
        base.set_horizontal_alignment(HorizontalAlignment::HORIZONTAL_ALIGNMENT_RIGHT);
        base.set_h_grow_direction(GrowDirection::GROW_DIRECTION_BEGIN);
        Self { base }
    }

    fn process(&mut self, _delta: f64) {
        self.base
            .set_text(format!("FPS: {}", Engine::singleton().get_frames_per_second()).into());
    }
}
