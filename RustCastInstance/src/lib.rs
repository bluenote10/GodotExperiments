#[macro_use]
extern crate gdnative as godot;

use godot::api::Object;
use godot::core_types::Variant;

#[derive(NativeClass)]
#[inherit(Object)]
struct WorldData {
    some_data: Vec<String>,
}

#[godot::methods]
impl WorldData {
    fn new(_owner: &Object) -> Self {
        WorldData {
            some_data: Vec::new(),
        }
    }
}

#[derive(NativeClass)]
#[inherit(Object)]
struct WorldManager;

#[godot::methods]
impl WorldManager {
    fn new(_owner: &Object) -> Self {
        WorldManager
    }

    #[export]
    fn set_world_data(&self, _owner: &Object, world_data: Variant) {
        // Goal: access world_data.some_data
        let world_data = world_data.try_to_object::<Object>().expect("failed to cast to object");
        let world_data = world_data.cast_instance::<WorldData>().expect("failed to cast to native script");
    }
}

fn init(handle: gdnative::nativescript::InitHandle) {
    handle.add_class::<WorldData>();
    handle.add_class::<WorldManager>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
