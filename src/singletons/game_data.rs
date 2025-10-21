use godot::{classes::{file_access::ModeFlags, Engine, FileAccess, Json}, global::clampf, prelude::*};
use serde::{Deserialize, Serialize};

use crate::enums::customer_feedback::CustomerFeedback;


#[derive(Default, Serialize, Deserialize)]
pub struct CoffeeComponent {
    pub coffee: f32,
    pub milk: f32,
    pub sugar: f32,
}

impl CoffeeComponent {
    pub fn _is_empty(&self) -> bool {
        match (self.coffee, self.milk, self.sugar) {
            (0.0, 0.0, 0.0) => true,
            _ => false,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct GameDataSave {
    pub stock: i32,
    pub money: i32,
    pub day: i32,
    pub price: i32,
    pub cup: i32,
    pub favorability: f32,
    pub inventory: CoffeeComponent,
    pub recipe: CoffeeComponent,
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
    pub favorability: f32,
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
        self.money = 300;
        self.day = 1;
        self.inventory = CoffeeComponent {
            coffee: 300.0,
            milk: 1000.0,
            sugar: 1000.0,
        };
        self.price = 8;
        self.cup = 50;
        self.favorability = 0.5;
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
        // self.day += 1;
        self.cup -= self.stock;
        self.inventory.coffee -= self.recipe.coffee;
        self.inventory.milk -= self.recipe.milk;
        self.inventory.sugar -= self.recipe.sugar;
    }

    pub fn add_money(&mut self, amount: i32) -> i32 {
        self.money += amount;
        self.money
    }

    pub fn update_favorability(&mut self, feedback: &CustomerFeedback) {
        self.favorability = match feedback {
            CustomerFeedback::Love => clampf(self.favorability as f64 + 0.05, 0.0, 1.0),
            CustomerFeedback::Like => clampf(self.favorability as f64 + 0.02, 0.0, 1.0),
            CustomerFeedback::Dislike => clampf(self.favorability as f64 + 0.04, 0.0, 1.0),
            _ => self.favorability as f64,
        } as f32
    }

    pub fn save_game(&self) {
        let file = FileAccess::open("user://savegame.json", ModeFlags::WRITE);
        if let Some(mut file) = file {
            let save_data = self.to_save();
            let save_json = serde_json::to_string(&save_data);
            if let Ok(json_string) = save_json {
                file.store_string(&json_string);
            }
        };
    }

    pub fn load_game(&mut self) {
        let file = FileAccess::open("user://savegame.json", ModeFlags::READ);
        if let Some(file) = file {
            let json_string = file.get_as_text();
            let save_data: Result<GameDataSave, _> = serde_json::from_str(json_string.to_string().as_str());
            if let Ok(save) = save_data {
                self.from_save(save);
            }
        };
    }

    fn to_save(&self) -> GameDataSave {
        GameDataSave {
            stock: self.stock,
            money: self.money,
            day: self.day,
            price: self.price,
            cup: self.cup,
            favorability: self.favorability,
            inventory: CoffeeComponent {
                coffee: self.inventory.coffee,
                milk: self.inventory.milk,
                sugar: self.inventory.sugar,
            },
            recipe: CoffeeComponent {
                coffee: self.recipe.coffee,
                milk: self.recipe.milk,
                sugar: self.recipe.sugar,
            },
        }
    }

    fn from_save(&mut self, save: GameDataSave) {
        self.stock = save.stock;
        self.money = save.money;
        self.day = save.day;
        self.price = save.price;
        self.cup = save.cup;
        self.favorability = save.favorability;
        self.inventory = CoffeeComponent {
            coffee: save.inventory.coffee,
            milk: save.inventory.milk,
            sugar: save.inventory.sugar,
        };
        self.recipe = CoffeeComponent {
            coffee: save.recipe.coffee,
            milk: save.recipe.milk,
            sugar: save.recipe.sugar,
        };
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