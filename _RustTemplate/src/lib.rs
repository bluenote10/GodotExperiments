use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TestClass {}

#[gdnative::methods]
impl TestClass {
    fn new(_owner: &Node) -> Self {
        TestClass {}
    }
}

fn init(handle: gdnative::nativescript::InitHandle) {
    handle.add_class::<TestClass>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
