use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct GameDataSingleton {
    base: Base<Object>,
    pub stock: i32,
    pub money: i32,
    pub day: i32,
    pub coffee: i32,
    pub milk: i32,
    pub sugar: i32,
    pub cup: i32,
    pub price: i32,
}

#[godot_api]
impl GameDataSingleton {
    pub fn start_new(&mut self) {
        self.stock = 0;
        self.money = 5_000_000;
        self.day = 1;
        self.coffee = 0;
        self.milk = 0;
        self.sugar = 0;
        self.cup = 0;
        self.price = 8000;
    }

    pub fn is_new_game(&self) -> bool {
        self.day == 0
    }
}
