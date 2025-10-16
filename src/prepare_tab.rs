use godot::{classes::{Button, Control, Engine, IControl, LineEdit, RichTextLabel}, prelude::*};

use crate::{prep_phase::PrepPhase, singletons::game_data::GameDataSingleton};

#[derive(GodotClass)]
#[class(base=Control)]
struct PrepareTab {
    base: Base<Control>,

    // Change or add your own properties here
    #[export]
    coffee_label: Option<Gd<RichTextLabel>>,
    #[export]
    milk_label: Option<Gd<RichTextLabel>>,
    #[export]
    sugar_label: Option<Gd<RichTextLabel>>,
    #[export]
    cup_label: Option<Gd<RichTextLabel>>,
    #[export]
    coffee_input: Option<Gd<LineEdit>>,
    #[export]
    milk_input: Option<Gd<LineEdit>>,
    #[export]
    sugar_input: Option<Gd<LineEdit>>,
    #[export]
    price_input: Option<Gd<LineEdit>>,
    #[export]
    save_recipe_button: Option<Gd<Button>>,
    #[export]
    prep_phase: Option<Gd<PrepPhase>>,
}

#[godot_api]
impl IControl for PrepareTab {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            coffee_label: None,
            milk_label: None,
            sugar_label: None,
            cup_label: None,
            coffee_input: None,
            milk_input: None,
            sugar_input: None,
            price_input: None,
            save_recipe_button: None,
            prep_phase: None,
        }
    }

    fn ready(&mut self) {
        let mut game_data: Gd<GameDataSingleton> = Engine::singleton().get_singleton(&StringName::from("GameDataSingleton")).unwrap().cast();
        if game_data.bind_mut().is_new_game() {
            game_data.bind_mut().start_new();
        }
        let inventory = &game_data.bind().inventory;
        let recipe = &game_data.bind().recipe;
        
        self.get_coffee_label().unwrap().set_text(&format!("Owned: {} g", inventory.coffee));
        self.get_milk_label().unwrap().set_text(&format!("Owned: {} mL", inventory.milk));
        self.get_sugar_label().unwrap().set_text(&format!("Owned: {} g", inventory.sugar));
        self.get_cup_label().unwrap().set_text(&format!("Owned: {}", game_data.bind().cup));

        self.get_coffee_input().unwrap().set_text(&format!("{}", recipe.coffee));
        self.get_milk_input().unwrap().set_text(&format!("{}", recipe.milk));
        self.get_sugar_input().unwrap().set_text(&format!("{}", recipe.coffee));
        self.get_price_input().unwrap().set_text(&format!("{}", game_data.bind().price));

        let save_recipe_button = self.save_recipe_button.as_ref().unwrap();
        save_recipe_button
            .signals()
            .pressed()
            .connect_other(&*self, Self::_on_save_recipe_button_pressed);
        
    }

    fn process(&mut self, _delta: f64) {
        // This is where you would put your game logic
    }
}     

impl PrepareTab {
    fn _on_save_recipe_button_pressed(&mut self) {
        let mut game_data: Gd<GameDataSingleton> = Engine::singleton().get_singleton(&StringName::from("GameDataSingleton")).unwrap().cast();
        
        // Save recipe logic here
        let coffee = self.get_coffee_input().unwrap().get_text().to_float() as f32;
        let milk = self.get_milk_input().unwrap().get_text().to_float() as f32;
        let sugar = self.get_sugar_input().unwrap().get_text().to_float() as f32;
        game_data.bind_mut().save_recipe(coffee, milk, sugar);

        let stock = game_data.bind().stock;

        let prep_phase = self.get_prep_phase();
        prep_phase.unwrap().bind_mut().update_stock(stock);
    }
}