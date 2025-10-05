use godot::prelude::*;

// Import modules here
// mod rust_example;
mod selling_phase;
mod customer_spawner;
mod customer;

struct GdRust;

#[gdextension]
unsafe impl ExtensionLibrary for GdRust {}
