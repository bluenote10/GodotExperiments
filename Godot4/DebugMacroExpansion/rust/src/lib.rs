#![feature(trace_macros)]
#![feature(log_syntax)]

trace_macros!(true);

// Also worth trying:
//
// RUSTFLAGS="-Zunpretty=expanded" cargo +nightly build 2>&1 | less
// rustc +nightly -Zunpretty=expanded ./src/lib.rs
// cargo +nightly rustc -- -Zunpretty=expanded
// cargo expand
//
// The latter two give the best output. Full resulting macro is visible.
//
// There is also something in the VSCode palette: `rust-analyzer: Expand macro recursively`
// This expands the macro itself recursively, but the submacros are not expanded yet,
// i.e., it is still an intermediate expansion.
//

use godot::{engine::InputEvent, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MinimalNode {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl GodotExt for MinimalNode {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let _ = "in ready body";
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let _ = "in input body";
        godot_print!("in input");
        godot_print!("instance_id: {}", event.instance_id());
        godot_print!("is_instance_valid: {}", event.is_instance_valid());
        godot_print!("ref count: {}", event.get_reference_count());
        // godot_print!("event: {}", event);
    }
}

struct ExtensionImpl;

#[gdextension]
unsafe impl ExtensionLibrary for ExtensionImpl {}
