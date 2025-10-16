use godot::{classes::{Button, Control, Engine, IControl, RichTextLabel}, prelude::*};

use crate::singletons::game_data::GameDataSingleton;


#[derive(GodotClass)]
#[class(base=Control)]
pub struct PrepPhase {
    base: Base<Control>,
    
    // Change or add your own properties here
    #[export]
    start_day_button: Option<Gd<Button>>,
    #[export]
    money_label: Option<Gd<RichTextLabel>>,
    #[export]
    stock_label: Option<Gd<RichTextLabel>>,
    #[export]
    day_count_label: Option<Gd<RichTextLabel>>,
}

#[godot_api]
impl IControl for PrepPhase {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            start_day_button: None,
            money_label: None,
            stock_label: None,
            day_count_label: None,
        }
    }

    fn ready(&mut self) {
        let mut game_data: Gd<GameDataSingleton> = Engine::singleton().get_singleton(&StringName::from("GameDataSingleton")).unwrap().cast();
        if game_data.bind().is_new_game() {
            game_data.bind_mut().start_new();
        }
        
        let money = game_data.bind().money;
        let day = game_data.bind().day;
        self.money_label.as_mut().unwrap().set_text(&money.to_string());
        self.day_count_label.as_mut().unwrap().set_text(format!("Day {}", day).as_str());

        let start_day_button = self.start_day_button.as_ref().unwrap();
        start_day_button
            .signals()
            .pressed()
            .connect_other(&*self, Self::_on_start_day_button_pressed);
    }

    fn process(&mut self, _delta: f64) {

    } 
}     

#[godot_api]
impl PrepPhase {
    #[func]
    pub fn update_stock(&mut self, stock: i32) {
        self.stock_label.as_mut().unwrap().set_text(&format!("Stock: {}", stock));
    }

    fn _on_start_day_button_pressed(&mut self) {
        // let mut game_data: Gd<GameDataSingleton> = Engine::singleton().get_singleton(&StringName::from("GameDataSingleton")).unwrap().cast();

        let mut tree = self.base().get_tree().unwrap();
        tree.change_scene_to_file("res://scenes/selling_phase.tscn");
    }
}