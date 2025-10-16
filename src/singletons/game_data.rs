use godot::{classes::Engine, prelude::*};


#[derive(Default)]
pub struct CoffeeComponent {
    pub coffee: f32,
    pub milk: f32,
    pub sugar: f32,
}

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct GameDataSingleton {
    base: Base<Object>,
    pub stock: i32,
    pub money: i32,
    pub day: i32,
    pub price: i32,
    pub cup: i32,
    pub inventory: CoffeeComponent,
    pub recipe: CoffeeComponent,
}

#[godot_api]
impl GameDataSingleton {
    pub fn get_instance() -> Gd<Self> {
        Engine::singleton().get_singleton(&StringName::from("GameDataSingleton")).unwrap().cast()
    }

    pub fn start_new(&mut self) {
        self.stock = 0;
        self.money = 5_000;
        self.day = 1;
        self.inventory = CoffeeComponent {
            coffee: 300.0,
            milk: 1000.0,
            sugar: 1000.0,
        };
        self.price = 8;
        self.cup = 50;
    }

    pub fn is_new_game(&self) -> bool {
        self.day == 0
    }

    pub fn save_recipe(&mut self, coffee: f32, milk: f32, sugar: f32) {
        self.recipe.coffee = coffee;
        self.recipe.milk = milk;
        self.recipe.sugar = sugar;

        let coffee_divided = self.inventory.coffee / self.recipe.coffee;
        let milk_divided = self.inventory.milk / self.recipe.milk;
        let sugar_divided = self.inventory.sugar / self.recipe.sugar;

        let possible_cups = f32::min(coffee_divided, milk_divided);
        let possible_cups = f32::min(possible_cups, sugar_divided);

        self.stock = possible_cups.floor() as i32;
    }

    pub fn start_day(&mut self) {
        self.day += 1;
        self.cup -= self.stock;
        self.inventory.coffee -= self.recipe.coffee;
        self.inventory.milk -= self.recipe.milk;
        self.inventory.sugar -= self.recipe.sugar;
    }

    pub fn add_money(&mut self, amount: i32) -> i32 {
        self.money += amount;
        self.money
    }
}

#[test]
fn save_recipe() {
    // Test data
    let inventory = CoffeeComponent {
        coffee: 300.0, // grams
        milk: 1000.0, // mL
        sugar: 1000.0, // grams
    };
    let cup: i32 = 50;
    
    let coffee: f32 = 7.0;
    let milk: f32 = 120.0;
    let sugar: f32 = 10.0;

    // Expected values
    let expected_stock = 8;
    let expected_cup = 42;

    // Execute test
    let mut game_data = GameDataSingleton::new_alloc();
    game_data.bind_mut().inventory = inventory;
    game_data.bind_mut().cup = cup;
    game_data.bind_mut().save_recipe(coffee, milk, sugar);

    assert_eq!(game_data.bind().cup, expected_cup);
    assert_eq!(game_data.bind().stock, expected_stock);
}