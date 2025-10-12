use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct GameDataSingleton {
    base: Base<Object>,
    pub stock: i32,
    pub money: i32,

}

// #[godot_api]
// impl GameDataSingleton {
// }
