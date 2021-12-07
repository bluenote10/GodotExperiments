use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TestClass {
    #[property]
    enemy_count: i32,
}

#[gdnative::methods]
impl TestClass {
    fn new(_owner: &Node) -> Self {
        godot_print!("Initializing native class...");
        TestClass { enemy_count: 42 }
    }

    #[export]
    fn _process(&self, _owner: &Node, _t: f64) {
        godot_print!("enemy_count: {}", self.enemy_count);
    }
}

fn init(handle: gdnative::nativescript::InitHandle) {
    handle.add_class::<TestClass>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
