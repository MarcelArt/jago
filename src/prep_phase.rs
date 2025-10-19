use godot::{classes::{Button, Control, IControl, LineEdit, RichTextLabel}, prelude::*};

use crate::{error_alert::ErrorAlert, shop_tab::ShopTab, singletons::game_data::GameDataSingleton};


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
    #[export]
    price_input: Option<Gd<LineEdit>>,
    #[export]
    shop_tab: Option<Gd<ShopTab>>,
    #[export]
    error_alert: Option<Gd<ErrorAlert>>,
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
            price_input: None,
            shop_tab: None,
            error_alert: None,
        }
    }

    fn ready(&mut self) {
        let mut game_data= GameDataSingleton::get_instance();
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

        let shop_tab = self.get_shop_tab().unwrap();
        shop_tab.signals()
            .on_buy_success()
            .connect_other(&*self, Self::update_money_label);
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
        let mut game_data= GameDataSingleton::get_instance();
        godot_print!("stock={}", game_data.bind().stock);
        if game_data.bind().stock <= 0 {
            let mut error_alert = self.get_error_alert().unwrap();
            error_alert.bind_mut().show_alert(GString::from("Stock are empty please make coffee first"));
            return;
        }
        
        game_data.bind_mut().price = self.get_price_input().unwrap().get_text().to_int() as i32;
        if game_data.bind().price <= 0 {
            let mut error_alert = self.get_error_alert().unwrap();
            error_alert.bind_mut().show_alert(GString::from("Put a price on your coffee"));
            return;
        }
        
        game_data.bind_mut().start_day();

        let mut tree = self.base().get_tree().unwrap();
        tree.change_scene_to_file("res://scenes/selling_phase.tscn");
    }

    fn update_money_label(&mut self) {
        let game_data = GameDataSingleton::get_instance();
        self.get_money_label().unwrap().set_text(&format!("{}", game_data.bind().money));
    }
}