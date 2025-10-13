use godot::{classes::{Button, Control, Engine, IControl, LineEdit}, prelude::*};

use crate::singletons::game_data::GameDataSingleton;

#[derive(GodotClass)]
#[class(base=Control)]
struct PrepPhase {
    base: Base<Control>,
    start_day_button: Option<Gd<Button>>,
    stock_input: Option<Gd<LineEdit>>,

    // Change or add your own properties here
    // #[export]
}

#[godot_api]
impl IControl for PrepPhase {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            start_day_button: None,
            stock_input: None,
        }
    }

    fn ready(&mut self) {
        self.start_day_button = Some(self.base().get_node_as("StartDayButton"));
        let start_day_button = self.start_day_button.as_ref().unwrap();
        start_day_button
            .signals()
            .pressed()
            .connect_other(&*self, Self::_on_start_day_button_pressed);

        self.stock_input = Some(self.base().get_node_as("StockInput"));
    }

    fn process(&mut self, _delta: f64) {

    } 
}     

impl PrepPhase {
    fn _on_start_day_button_pressed(&mut self) {
        let mut game_data: Gd<GameDataSingleton> = Engine::singleton().get_singleton(&StringName::from("GameDataSingleton")).unwrap().cast();
        game_data.bind_mut().stock = self.stock_input.as_ref().unwrap().get_text().to_int() as i32;

        let mut tree = self.base().get_tree().unwrap();
        tree.change_scene_to_file("res://scenes/selling_phase.tscn");
    }
}