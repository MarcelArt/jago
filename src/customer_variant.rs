use godot::{classes::{IResource, Resource}, prelude::*};

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct CustomerVariant {
    base: Base<Resource>,

    // Change or add your own properties here
    #[export]
    coffee_pref: f32,
    #[export]
    milk_pref: f32,
    #[export]
    sugar_pref: f32,
}

#[godot_api]
impl IResource for CustomerVariant {
    fn init(base: Base<Resource>) -> Self {
        Self {
            base,
            coffee_pref: 7.0,
            milk_pref: 120.0,
            sugar_pref: 10.0,
        }
    }  
}     
        