use godot::{classes::{ICharacterBody2D, CharacterBody2D}, prelude::*};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Customer {
    base: Base<CharacterBody2D>,

    // Change or add your own properties here
    // #[export]
}

#[godot_api]
impl ICharacterBody2D for Customer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        // This is where you would put your initialization code
        godot_print!("Customer is ready!");
    }

    fn process(&mut self, _delta: f64) {
    }  
}     
        