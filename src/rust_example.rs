use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct RustExample {
    base: Base<Node2D>,

    #[export]
    value: i32,
}

#[godot_api]
impl INode2D for RustExample {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            value: 0,
        }
    }

    fn ready(&mut self) {
        // This is where you would put your initialization code
        godot_print!("RustExample is ready!");
    }

    fn process(&mut self, _delta: f64) {
        // This is where you would put your game logic
        godot_print!("Hello from Rust!");
    }

    fn physics_process(&mut self, _delta: f64) {
        // This is where you would put your physics logic
        godot_print!("Hello from Rust in physics process!");
    }    
}